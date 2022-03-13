#![allow(clippy::needless_return, clippy::iter_nth_zero)]

use clap::*;
use crossterm::event::{Event::Mouse, EventStream, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use ewin_com::{
    _cfg::{key::keycmd::*, lang::lang_cfg::*, model::default::*},
    def::*,
    global::*,
    log::*,
    model::*,
    util::*,
};
use ewin_term::model::*;
use futures::{future::FutureExt, StreamExt};
use std::{io::*, panic, sync::mpsc::*, time::Duration};
mod watch_file;
use watch_file::*;
mod watch_grep;
use watch_grep::*;

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

    Log::info("args", &args);

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
    // Processing ends when the terminal size is small
    if !Terminal::check_displayable() {
        println!("{}", &Lang::get().terminal_size_small);
        return;
    }
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    Terminal::init();
    let mut term = Terminal::new();
    term.activate(&args);
    term.init_draw(&mut out);

    let (tx, rx) = std::sync::mpsc::channel();
    let tx_grep = Sender::clone(&tx);
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
    watching_grep(tx_grep);

    loop {
        if let Ok(job) = rx.recv_timeout(Duration::from_millis(16)) {
            // for job in rx {
            match job.job_type {
                JobType::Event => {
                    let keys = Keybind::evt_to_keys(&job.job_evt.unwrap().evt);
                    if EvtAct::match_event(keys, &mut out, &mut term) {
                        break;
                    }
                    term.keys_org = keys;
                }
                JobType::GrepResult => EvtAct::draw_grep_result(&mut out, &mut term, job.job_grep.unwrap()),
                JobType::Watch => {
                    if let Some(job_watch) = job.job_watch {
                        if EvtAct::draw_watch_result(&mut out, &mut term) {
                            WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.history_set.remove(&(job_watch.fullpath_str, job_watch.unixtime_str))).unwrap();
                        }
                    }
                }
            }
        }
    }
    Terminal::exit();
}
