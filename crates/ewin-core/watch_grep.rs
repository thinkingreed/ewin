#![allow(clippy::needless_return, clippy::iter_nth_zero)]
use ewin_com::{global::*, model::*};
use ewin_term::model::*;
use futures::{future::FutureExt, select, StreamExt};
use std::{
    panic,
    sync::mpsc::*,
    thread::{self},
    time::{self},
};
use tokio_util::codec::*;

pub fn watching_grep(tx_grep: Sender<Job>) {
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
                                        send_grep_job("".to_string(), &tx_grep, grep_info);
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
                                        Some(Ok(grep_str))=> send_grep_job(grep_str,& tx_grep, grep_info),
                                        None=> grep_info.is_stdout_end = true,
                                        _ => {},
                                    }
                                },
                                std_err = read_stderr => {
                                    match std_err {
                                      Some(Ok(grep_str)) => send_grep_job(grep_str, & tx_grep, grep_info),
                                      None => grep_info.is_stderr_end = true,
                                        _ => {},
                                    }
                                }
                            }
                            if grep_info.is_stdout_end && grep_info.is_stderr_end {
                                //     drop(child);
                                send_grep_job("".to_string(), &tx_grep, grep_info);
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
}
pub fn send_grep_job(grep_str: String, tx_grep: &Sender<Job>, grep_info: &GrepState) {
    let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str, is_result: grep_info.is_result, is_cancel: grep_info.is_cancel, is_stdout_end: grep_info.is_stdout_end, is_stderr_end: grep_info.is_stderr_end }), ..Job::default() };
    let _ = tx_grep.send(job);
}
