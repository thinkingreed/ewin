#![allow(clippy::needless_return, clippy::iter_nth_zero)]

use clap::*;
use crossterm::event::{Event::Mouse, EventStream, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use ewin_com::{
    _cfg::{cfg::*, key::keycmd::*, lang::lang_cfg::*},
    def::*,
    global::*,
    log::*,
    model::*,
};
use ewin_term::model::*;
use futures::{channel::mpsc::Receiver, future::FutureExt, select, SinkExt, StreamExt};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde_json::Value;
use std::{
    collections::HashMap,
    io::*,
    panic,
    path::Path,
    sync::mpsc::*,
    thread::{self},
    time::{self, SystemTime, UNIX_EPOCH},
};
use tokio_util::codec::*;

#[tokio::main]
async fn main() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        eprintln!("{}", panic_info);
        Log::error("Unexpected panic", panic_info);

        Terminal::finalize();
        // Set hook to log crash reason
        default_hook(panic_info);
    }));

    let matches = App::new(APP_NAME).version(env!("CARGO_PKG_VERSION")).bin_name(APP_NAME).setting(AppSettings::DeriveDisplayOrder).arg(Arg::with_name("file").required(false)).arg(Arg::from_usage("[output-config] -o --output-config 'output config file'")).get_matches();
    let args = Args::new(&matches);

    let err_str = Cfg::init(&args, include_str!("../../setting.toml"));
    if !err_str.is_empty() {
        Terminal::exit_file_open(&err_str);
    }
    let err_str = Keybind::init(&args, include_str!("../../keybind.json5"));
    if !err_str.is_empty() {
        Terminal::exit_file_open(&err_str);
    }
    let _ = APP_VERSION.set(get_app_version());

    if args.out_config_flg {
        return;
    }

    let _ = LANG.set(Lang::read_lang_cfg());

    // Processing ends when the terminal size is small
    if !Terminal::check_displayable() {
        println!("{:?}", &Lang::get().terminal_size_small);
        return;
    }

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    Terminal::init();
    let mut term = Terminal::new();
    term.activate(&args);
    term.init_draw(&mut out);

    let (tx, rx) = std::sync::mpsc::channel();
    let mut tx_grep = Sender::clone(&tx);
    let tx_watch = Sender::clone(&tx);

    // It also reads a normal Event to support cancellation.
    let mut reader = EventStream::new();
    tokio::spawn(async move {
        loop {
            if let Some(Ok(evt)) = reader.next().fuse().await {
                match evt {
                    Mouse(M_Event { kind: M_Kind::Up(M_Btn::Left), .. }) => continue,
                    Mouse(M_Event { kind: M_Kind::Up(M_Btn::Right), .. }) => continue,
                    _ => {}
                }
                let job = Job { job_type: JobType::Event, job_evt: Some(JobEvent { evt }), ..Job::default() };
                let _ = tx.send(job);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            thread::sleep(time::Duration::from_millis(1000));

            if let Some(Ok(mut grep_info_vec)) = GREP_INFO_VEC.get().map(|vec| vec.try_lock()) {
                if grep_info_vec.is_empty() {
                    continue;
                }
                let grep_info_idx = grep_info_vec.len() - 1;
                if let Some(mut grep_info) = grep_info_vec.get_mut(grep_info_idx) {
                    if grep_info.is_result && !grep_info.is_cancel && !(grep_info.is_stdout_end && grep_info.is_stderr_end) {
                        let mut child = EvtAct::get_grep_child(&grep_info.search_str, &grep_info.search_folder, &grep_info.search_filenm);
                        let mut reader_stdout = FramedRead::new(child.stdout.take().unwrap(), LinesCodec::new());
                        let mut reader_stderr = FramedRead::new(child.stderr.take().unwrap(), LinesCodec::new());

                        loop {
                            // Sleep to receive key event
                            thread::sleep(time::Duration::from_millis(50));

                            {
                                if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
                                    let is_cancel = grep_cancel_vec[grep_info_idx];
                                    if is_cancel {
                                        drop(child);
                                        grep_info.is_cancel = true;
                                        send_grep_job("".to_string(), &mut tx_grep, grep_info);
                                        grep_info.is_result = false;
                                        grep_info.is_cancel = false;
                                        break;
                                    }
                                }
                            }
                            let mut read_stdout = reader_stdout.next().fuse();
                            let mut read_stderr = reader_stderr.next().fuse();
                            select! {
                                std_out = read_stdout => {
                                    match std_out {
                                        Some(Ok(grep_str))=> send_grep_job(grep_str, &mut tx_grep, grep_info),
                                        None=> grep_info.is_stdout_end = true,
                                        _ => {},
                                    }
                                },
                                std_err = read_stderr => {
                                    match std_err {
                                      Some(Ok(grep_str)) => send_grep_job(grep_str, &mut tx_grep, grep_info),
                                      None => grep_info.is_stderr_end = true,
                                        _ => {},
                                    }
                                }
                            }
                            if grep_info.is_stdout_end && grep_info.is_stderr_end {
                                //     drop(child);
                                send_grep_job("".to_string(), &mut tx_grep, grep_info);
                                grep_info.is_result = false;
                                grep_info.is_stdout_end = false;
                                grep_info.is_stderr_end = false;
                                break;
                            }
                        }
                    }
                }
            }
        }
    });
    let path = if cfg!(target_os = "windows") { "C:\\Users\\hi\\rust\\ewin\\target\\debug\\notify.txt" } else { "/home/thinkingreed/rust/ewin/target/debug/notify.txt" };

    let (mut watcher, mut rx_notify) = async_watcher().unwrap();
    let mut watch_state_org = WatchState::default();

    tokio::spawn(async move {
        loop {
            thread::sleep(time::Duration::from_millis(1000));

            if let Some(Ok(mut watch_state)) = WATCH_STATE.get().map(|watch_state| watch_state.try_lock()) {
                Log::debug_s("loop 111");

                if watch_state.fullpath != watch_state_org.fullpath {
                    Log::debug("watch_state.fullpath != watch_state_org.fullpath", &(watch_state.fullpath != watch_state_org.fullpath));
                    let path = Path::new(&watch_state.fullpath);
                    watcher.unwatch(path);
                    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
                }

                let ss = rx_notify.next().fuse();
                select! {
                        evt = ss => {
                            if evt.kind.is_modify() {
                                let unixtime_seq = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                                let job = Job { job_type: JobType::Watch, job_watch: Some(JobWatch { watch_state: WatchState { unixtime_seq, ..WatchState::default() } }), ..Job::default() };
                                let _ = tx_watch.send(job);
                                if watch_state.unixtime_seq != watch_state_org.unixtime_seq {
                                    watch_state_org = watch_state.clone();
                                }
                            }
                    }
                }
                while let Ok(Ok(evt)) = rx_notify.next().fuse() {
                    if evt.kind.is_modify() {
                        let unixtime_seq = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                        let job = Job { job_type: JobType::Watch, job_watch: Some(JobWatch { watch_state: WatchState { unixtime_seq, ..WatchState::default() } }), ..Job::default() };
                        let _ = tx_watch.send(job);
                        if watch_state.unixtime_seq != watch_state_org.unixtime_seq {
                            watch_state_org = watch_state.clone();
                        }
                    }
                }
                Log::debug_s("loop 222");
                if watch_state_org != *watch_state {
                    Log::debug("watch_state_org != watch_state", &(watch_state_org != *watch_state));
                }
            }
        }
    });

    for job in rx {
        match job.job_type {
            JobType::Event => {
                let keys = Keybind::evt_to_keys(&job.job_evt.unwrap().evt);
                if EvtAct::match_event(keys, &mut out, &mut term) {
                    break;
                }
                term.keys_org = keys;
            }
            JobType::GrepResult => EvtAct::draw_grep_result(&mut out, &mut term, job.job_grep.unwrap()),
            JobType::Watch => Log::debug("JobType::Watch", &job.job_watch),
        }
    }
    Terminal::exit();
}

pub fn send_grep_job(grep_str: String, tx_grep: &mut Sender<Job>, grep_info: &GrepState) {
    let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str, is_result: grep_info.is_result, is_cancel: grep_info.is_cancel, is_stdout_end: grep_info.is_stdout_end, is_stderr_end: grep_info.is_stderr_end }), ..Job::default() };
    let _ = tx_grep.send(job);
}

/// Get version of app as a whole
pub fn get_app_version() -> String {
    let cfg_str = include_str!("../../Cargo.toml");
    let map: HashMap<String, Value> = toml::from_str(cfg_str).unwrap();
    let mut s = map["package"]["version"].to_string();
    s.retain(|c| c != '"');
    return s;
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = futures::channel::mpsc::channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(move |res| {
        futures::executor::block_on(async {
            tx.send(res).await.unwrap();
        })
    })?;

    Ok((watcher, rx))
}
