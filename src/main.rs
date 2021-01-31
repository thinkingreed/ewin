#[macro_use]
extern crate clap;
use clap::{App, Arg};
use crossterm::{
    event::{Event, EventStream},
    ErrorKind,
};
use ewin::model::*;
use futures::{future::FutureExt, select, StreamExt};
use std::ffi::OsStr;
use std::io::{stdout, BufWriter, Write};
use std::panic;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    let matches = App::new("ewin").version(crate_version!()).bin_name("ewin").arg(Arg::with_name("file").required(false)).get_matches();
    let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();

    // Processing ends when the terminal size is small
    if !Terminal::check_displayable() {
        return;
    }
    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |e| {
        eprintln!("{}", e);
        Terminal::exit();
        // Set hook to log crash reason
        default_hook(e);
    }));

    let mut editor = Core::new();
    let mut mbar = MsgBar::new();
    let mut prom = Prompt::new();
    let mut sbar = StatusBar::new();

    Terminal::set_disp_size(&mut editor, &mut mbar, &mut prom, &mut sbar);
    Terminal::init();
    Terminal::activate(&mut editor, &mut mbar, &mut prom, &mut sbar, file_path);

    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    Terminal::draw(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).unwrap();

    if prom.is_grep_result {
        if let Err(err) = exec_events_grep_result(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).await {
            Log::ep("err", &err.to_string());
        }
    } else {
        if let Err(err) = exec_events(&mut out, &mut editor, &mut mbar, &mut prom, &mut sbar).await {
            Log::ep("err", &err.to_string());
        }
    }
}

async fn exec_events_grep_result<T: Write>(out: &mut T, editor: &mut Core, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> anyhow::Result<()> {
    // It also reads a normal Event to support cancellation.
    let mut reader = EventStream::new();
    let mut child = EvtAct::exec_grep(editor);

    let mut reader_stdout = FramedRead::new(child.stdout.take().unwrap(), LinesCodec::new());
    let mut reader_stderr = FramedRead::new(child.stderr.take().unwrap(), LinesCodec::new());
    let mut is_exit = false;

    loop {
        let mut event_next = reader.next().fuse();
        if prom.is_grep_stdout || prom.is_grep_stderr {
            let mut std_out_str = reader_stdout.next().fuse();
            let mut std_err_str = reader_stderr.next().fuse();
            select! {
                std_out_event = std_out_str => {
                    EvtAct::draw_grep_result(out, editor, mbar, prom, sbar, std_out_event, true,&mut child);
                },
                std_err_event = std_err_str => {
                    EvtAct::draw_grep_result(out, editor, mbar, prom, sbar, std_err_event, false, &mut child);
                },
                maybe_event = event_next => {
                    is_exit =  run_events(out, editor, mbar, prom,  sbar, maybe_event);
                }
            }
        } else {
            select! {
            maybe_event = event_next => {
                is_exit =  run_events(out,  editor, mbar, prom,  sbar, maybe_event);
            }}
        }
        if is_exit {
            break;
        }
    }
    Ok(())
}

async fn exec_events<T: Write>(out: &mut T, editor: &mut Core, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> anyhow::Result<()> {
    let mut reader = EventStream::new();
    let mut is_exit = false;
    loop {
        let mut event_next = reader.next().fuse();
        select! {
            maybe_event = event_next => {
                is_exit =  run_events(out, editor, mbar, prom,  sbar, maybe_event);
            }
        }
        if is_exit {
            break;
        }
    }
    Ok(())
}

fn run_events<T: Write>(out: &mut T, editor: &mut Core, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar, maybe_event: Option<Result<Event, ErrorKind>>) -> bool {
    let mut is_exit = false;

    if let Some(Ok(event)) = maybe_event {
        editor.evt = event.clone();
        is_exit = EvtAct::match_event(out, editor, mbar, prom, sbar);
        if is_exit {
            Terminal::exit();
        }
    }
    return is_exit;
}
