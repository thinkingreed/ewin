use clap::{App, Arg};
use crossterm::event::{Event::Mouse, EventStream, MouseEvent as M_Event, MouseEventKind as M_Kind};
use ewin::{_cfg::cfg::*, global::*, model::*, terminal::*};
use futures::{future::FutureExt, select, StreamExt};
use std::{
    ffi::OsStr,
    io::{stdout, BufWriter},
    panic,
    process::*,
    sync::mpsc::*,
    thread, time,
};
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME")).version(env!("CARGO_PKG_VERSION")).bin_name(env!("CARGO_PKG_NAME")).arg(Arg::with_name("file").required(false)).get_matches();
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

    Terminal::init();

    let args = Terminal::init_args(&file_path);
    let err_str = Cfg::init(&args);
    if !err_str.is_empty() {
        println!("{}", err_str);
    }
    let mut term = Terminal::new();

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    term.activate(&args, &mut out);

    let (tx, rx) = channel();
    let mut tx_grep = Sender::clone(&tx);

    // It also reads a normal Event to support cancellation.
    let mut reader = EventStream::new();
    tokio::spawn(async move {
        loop {
            if let Some(Ok(event)) = reader.next().fuse().await {
                match event {
                    Mouse(M_Event { kind: M_Kind::Moved, .. }) => continue,
                    _ => {}
                }
                let job = Job { job_type: JobType::Event, job_evt: Some(JobEvent { evt: event }), ..Job::default() };
                let _ = tx.send(job);
            }
        }
    });

    tokio::spawn(async move {
        loop {
            thread::sleep(time::Duration::from_millis(1000));

            if let Some(Ok(mut grep_info_vec)) = GREP_INFO_VEC.get().map(|vec| vec.try_lock()) {
                let grep_info_vec_len = grep_info_vec.len() - 1;
                if let Some(mut grep_info) = grep_info_vec.get_mut(grep_info_vec_len) {
                    if grep_info.is_result && !grep_info.is_cancel && !(grep_info.is_stdout_end && grep_info.is_stderr_end) {
                        let mut child = EvtAct::get_grep_child(&"1".to_string(), &grep_info.search_folder, &"*.ttt".to_string());

                        let mut reader_stdout = FramedRead::new(child.stdout.take().unwrap(), LinesCodec::new());
                        let mut reader_stderr = FramedRead::new(child.stderr.take().unwrap(), LinesCodec::new());
                        loop {
                            // Sleep to receive key event
                            thread::sleep(time::Duration::from_millis(10));

                            {
                                if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
                                    let is_cancel = grep_cancel_vec[grep_info_vec_len];
                                    if is_cancel {
                                        drop(child);
                                        grep_info.is_cancel = true;
                                        send_grep_job("".to_string(), &mut tx_grep, &grep_info);
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
                                        Some(Ok(grep_str))=> send_grep_job(grep_str, &mut tx_grep, &grep_info),
                                        None=> grep_info.is_stdout_end = true,
                                        _ => {},
                                    }
                                },
                                std_err = read_stderr => {
                                    match std_err {
                                        Some(Ok(grep_str)) => send_grep_job(grep_str, &mut tx_grep, &grep_info),
                                        None => grep_info.is_stderr_end = true,
                                        _ => {},
                                    }
                                }
                            }
                            if grep_info.is_stdout_end && grep_info.is_stderr_end {
                                //     drop(child);
                                send_grep_job("".to_string(), &mut tx_grep, &grep_info);
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

    for job in rx {
        match job.job_type {
            JobType::Event => {
                if EvtAct::match_event(job.job_evt.unwrap().evt, &mut out, &mut term) {
                    break;
                }
            }
            JobType::GrepResult => EvtAct::draw_grep_result(&mut out, &mut term, job.job_grep.unwrap()),
        }
    }
    Terminal::exit();
    // TODO
    exit(0);
}

pub fn send_grep_job(grep_str: String, tx_grep: &mut Sender<Job>, grep_info: &GrepInfo) {
    let job = Job {
        job_type: JobType::GrepResult,
        job_grep: Some(JobGrep {
            grep_str,
            is_cancel: grep_info.is_cancel,
            is_stdout_end: grep_info.is_stdout_end,
            is_stderr_end: grep_info.is_stderr_end,
        }),
        ..Job::default()
    };
    let _ = tx_grep.send(job);
}
