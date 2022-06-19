#![allow(clippy::needless_return, clippy::iter_nth_zero)]
use clap::Parser;
use crossbeam::channel::Sender;
use crossterm::event::{Event::Mouse, EventStream, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use ewin_cfg::{
    global::*,
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
};
use ewin_com::{
    _cfg::key::{keybind::*, keys::Keys},
    global::*,
    model::*,
    util::*,
};
use ewin_term::model::*;
use futures::{future::FutureExt, StreamExt};
use std::{io::*, panic, time::Duration};
mod watch_file;
use watch_file::*;

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

    let args = AppArgs::new(Args::parse());
    let err_str = Cfg::init(&args);
    if !err_str.is_empty() {
        //   Log::info_s(&err_str);
        Terminal::exit_file_open(&err_str);
    }
    let err_str = Keybind::init(&args);
    if !err_str.is_empty() {
        Terminal::exit_file_open(&err_str);
    }
    let _ = APP_VERSION.set(get_app_version());

    if args.out_config_flg {
        return;
    }
    let _ = LANG.set(Lang::read_lang_cfg());

    Log::debug("LANG_MAP", &LANG_MAP);

    // Processing ends when the terminal size is small
    if !Terminal::check_displayable() {
        println!("{}", &Lang::get().terminal_size_small);
        return;
    }
    // let out = stdout();
    let mut out = BufWriter::new(stdout().lock());

    Terminal::init();
    let mut term = Terminal::new();
    term.activate(&args);
    term.init_draw(&mut out);

    let (tx, rx) = crossbeam::channel::unbounded(); // PriorityChannel::<Job, usize>::init(false);

    // If it is processed asynchronously, the line number of the matched file in the grep process will shift,
    // so it will be executed synchronously.
    let (tx_grep, rx_grep) = std::sync::mpsc::channel();
    let _ = TX_JOB.set(tokio::sync::Mutex::new(std::sync::mpsc::Sender::clone(&tx_grep)));
    let tx_watch = Sender::clone(&tx);

    // It also reads a normal Event to support cancellation.
    let mut reader = EventStream::new();
    tokio::spawn(async move {
        loop {
            if let Some(Ok(evt)) = reader.next().fuse().await {
                if let Mouse(M_Event { kind: M_Kind::Up(M_Btn::Right), .. }) = evt {
                    continue;
                }
                let job = Job { job_type: JobType::Event, job_evt: Some(JobEvent { evt }), ..Job::default() };
                let _ = tx.send(job);
            }
        }
    });

    watching_file(tx_watch);

    loop {
        if let Ok(job) = rx.recv_timeout(Duration::from_millis(16)) {
            match job.job_type {
                JobType::Event => {
                    let keys = Keys::evt_to_keys(&job.job_evt.unwrap().evt);
                    if EvtAct::match_event(keys, &mut out, &mut term) {
                        break;
                    }
                    term.keys_org = keys;
                }
                JobType::Watch => {
                    if let Some(job_watch) = job.job_watch {
                        if EvtAct::draw_watch_file(&mut out, &mut term) {
                            WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.history_set.remove(&(job_watch.fullpath_str, job_watch.unixtime_str))).unwrap();
                        }
                    }
                }
                JobType::GrepResult => {}
            }
        }
        if let Ok(job) = rx_grep.recv_timeout(Duration::from_millis(16)) {
            if job.job_type == JobType::GrepResult {
                EvtAct::draw_grep_result(&mut out, &mut term, job.job_grep.unwrap());
            }
        }
    }
    Terminal::exit();
}
