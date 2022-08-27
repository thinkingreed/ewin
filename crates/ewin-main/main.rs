#![allow(clippy::needless_return, clippy::iter_nth_zero)]
use clap::Parser;
use crossterm::event::{Event::Mouse, EventStream, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use ewin_cfg::{
    global::*,
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
};
use ewin_job::{global::*, job::*};
use ewin_key::{
    global::*,
    key::{keybind::*, keys::*},
};
use ewin_utils::util::*;

use ewin_term::{evt_act::*, term::*};
use ewin_view::view::*;
use futures::{future::FutureExt, StreamExt};
use std::{
    io::*,
    panic,
    sync::mpsc::{channel, Sender},
};
mod watch_file;
use watch_file::*;

#[tokio::main]
async fn main() {
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        eprintln!("{}", panic_info);
        Log::error("Unexpected panic", panic_info);

        Term::finalize();
        // Set hook to log crash reason
        default_hook(panic_info);
    }));

    let args = AppArgs::new(Args::parse());
    let err_str = Cfg::init(&args);
    if !err_str.is_empty() {
        //   Log::info_s(&err_str);
        Term::exit_show_msg(&err_str);
    }
    let err_str = Keybind::init(&args);
    if !err_str.is_empty() {
        Term::exit_show_msg(&err_str);
    }
    let _ = APP_VERSION.set(get_app_version());

    if args.out_config_flg {
        return;
    }
    let _ = LANG.set(Lang::read_lang_cfg());

    Log::debug("LANG_MAP", &LANG_MAP);
    Log::debug("Lang::get_lang_map", &Lang::get_lang_map());

    // Processing ends when the terminal size is small
    if !View::check_displayable() {
        println!("{}", &Lang::get().terminal_size_small);
        return;
    }
    // let out = stdout();
    let mut out = BufWriter::new(stdout().lock());

    Term::init();
    let mut term = Term::new();
    term.activate(&args);
    term.init_draw(&mut out);

    let (tx, rx) = channel();
    // If it is processed asynchronously, the line number of the matched file in the grep process will shift,
    // so it will be executed synchronously.
    let _ = TX_JOB.set(tokio::sync::Mutex::new(Sender::clone(&tx)));

    // It also reads a normal Event to support cancellation.
    let mut reader = EventStream::new();
    tokio::spawn(async move {
        loop {
            if let Some(Ok(evt)) = reader.next().fuse().await {
                if let Mouse(M_Event { kind: M_Kind::Up(M_Btn::Right), .. }) = evt {
                    continue;
                }
                let _ = tx.send(Job { cont: JobCont::Key(JobKey { evt }) });
            }
        }
    });

    watching_file();

    for job in rx {
        match job.cont {
            JobCont::Key(job_evt) => {
                let keys = Keys::evt_to_keys(&job_evt.evt);
                if EvtAct::match_key(keys, &mut out, &mut term) {
                    break;
                }
                term.keys_org = keys;
            }
            JobCont::Grep(job_grep) => term.tabs.draw_grep_result(&mut out, job_grep),
            JobCont::Watch(job_watch) => {
                if term.tabs.draw_watch_file(&mut out) {
                    WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.history_set.remove(&(job_watch.fullpath_str, job_watch.unixtime_str))).unwrap();
                }
            }
            JobCont::Cmd(job_cmd) => {
                if EvtAct::specify_cmd(&mut term, &mut out, job_cmd.cmd_type, job_cmd.place, job_cmd.act_type_opt) {
                    break;
                }
            }
        }
    }
    Term::exit_proc();
}
