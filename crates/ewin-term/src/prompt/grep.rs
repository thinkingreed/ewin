#![allow(clippy::needless_return, clippy::iter_nth_zero)]
use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, def::*, global::*, log::*, model::*},
    model::*,
    tab::Tab,
};
use core::time;
use globset::Glob;
use grep::cli;
use grep_regex::RegexMatcher;
use grep_searcher::{BinaryDetection, Searcher, SearcherBuilder, Sink, SinkMatch};
use ignore::{WalkBuilder, WalkState};
use std::{env, ffi::OsStr, io, path::PathBuf, path::*, str::from_utf8, sync::mpsc::Sender, thread};

impl EvtAct {
    pub fn grep(term: &mut Terminal) -> ActType {
        Log::debug_s("EvtAct.grep");
        match term.curt().prom.keycmd {
            KeyCmd::Prom(P_Cmd::Resize(_, _)) => {
                term.curt().prom_grep();
                return ActType::Render(RParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let search_filenm = term.curt().prom.cont_2.buf.iter().collect::<String>();
                let mut search_folder = term.curt().prom.cont_3.buf.iter().collect::<String>();

                Log::debug("search_str", &search_str);
                Log::debug("search_filenm", &search_filenm);
                Log::debug("search_folder", &search_folder);

                if search_str.is_empty() {
                    return ActType::Render(RParts::MsgBar(Lang::get().not_set_search_str.to_string()));
                } else if search_filenm.is_empty() {
                    return ActType::Render(RParts::MsgBar(Lang::get().not_entered_search_file.to_string()));
                } else if search_folder.is_empty() {
                    return ActType::Render(RParts::MsgBar(Lang::get().not_entered_search_folder.to_string()));
                } else {
                    term.clear_curt_tab(true, true);

                    // if search_folder.chars().nth(0).unwrap() != '/' && search_folder.chars().nth(0).unwrap() != 'C' {
                    if Path::new(&search_folder).is_relative() {
                        let current_dir = env::current_dir().unwrap().display().to_string();
                        search_folder = format!("{}/{}", current_dir, search_folder);
                    }
                    Log::debug_s(&search_folder);
                    let path = Path::new(&search_folder).join(&search_filenm);

                    term.curt().prom.prom_grep.cache_search_filenm = search_filenm.clone();
                    term.curt().prom.prom_grep.cache_search_folder = search_folder.clone();

                    let mut grep_tab = Tab::new();
                    grep_tab.editor.search.str = search_str.clone();
                    grep_tab.editor.search.fullpath = path.to_string_lossy().to_string();
                    grep_tab.editor.search.folder = search_folder.clone();
                    grep_tab.editor.e_cmd = E_Cmd::GrepResult;

                    grep_tab.mbar.set_info(&Lang::get().searching);

                    grep_tab.state.grep.is_grep = false;
                    grep_tab.state.grep.is_result = true;
                    grep_tab.state.grep.is_end = false;
                    grep_tab.state.grep.search_str = search_str.clone();
                    grep_tab.state.grep.search_filenm = search_filenm;
                    grep_tab.state.grep.search_folder = search_folder;

                    term.add_tab(grep_tab.clone(), HeaderFile::new(&format!(r#"{} "{}""#, &Lang::get().grep, &search_str)), FileOpenType::Nomal);
                    GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().resize_with(term.tabs.len(), || GrepCancelType::None);
                    if let Some(Ok(tx)) = TX_JOB.get().map(|tx| tx.try_lock()) {
                        EvtAct::watching_grep(tx.clone(), grep_tab.state.grep.clone());
                    }
                    term.curt().prom.set_grep_working();

                    // Clear(ClearType::CurrentLine) is not performed during grep to prevent flicker. Therefore, clear first
                    Terminal::clear_all();
                }
                return ActType::Render(RParts::All);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn watching_grep(tx_grep: Sender<(Job, usize)>, grep_info: GrepInfo) {
        tokio::spawn(async move {
            // let mut grep_info = grep_info.clone();
            let matcher = RegexMatcher::new_line_matcher(cli::pattern_from_os(OsStr::new(&grep_info.search_str)).unwrap()).unwrap();
            let searcher = SearcherBuilder::new()
                .binary_detection(BinaryDetection::quit(0)) // Binary not applicable
                .line_number(true)
                .build();

            let glob = Glob::new(&grep_info.search_filenm).unwrap().compile_matcher();

            let walker = WalkBuilder::new(&grep_info.search_folder)
                .filter_entry(move |entry| glob.is_match(entry.file_name()))
                .hidden(true) // Search for hidden files
                .ignore(false) // Also target files that have been ignored
                .parents(false) // Don't go through the parent directory to find .gitignore
                // Sort in ascending order
                .sort_by_file_name(|a, b| a.cmp(b))
                .build_parallel();

            walker.run(|| {
                // For each thread in the thread pool
                let tx_grep = tx_grep.clone();
                let matcher = matcher.clone();
                let mut searcher = searcher.clone();
                // let mut grep_info = grep_info.clone();

                Box::new(move |result| match result {
                    // This inner callback is called per file path
                    // For files
                    Ok(entry) if entry.file_type().map(|t| t.is_file()).unwrap_or(false) => {
                        thread::sleep(time::Duration::from_millis(10));
                        Log::debug("is_canceled(grep_info_idx)", &EvtAct::is_grep_cancel());

                        // Check cancel
                        if EvtAct::is_grep_cancel() {
                            Log::debug_s("is_cancellll 22222222222222222222222");

                            WalkState::Quit
                        } else {
                            if let Err(err) = searcher.search_path(&matcher, entry.path(), SearchSink { tx: tx_grep.clone(), path: &entry.clone().into_path() }) {
                                send_grep_job(format!("{}:{}", entry.path().display(), err), &tx_grep, &GrepInfo { is_cancel: false, is_end: true, ..GrepInfo::default() });
                            };
                            WalkState::Continue
                        }
                    }
                    // For directory
                    Ok(_) => WalkState::Continue,
                    Err(err) => {
                        Log::error("grep error", &err);
                        WalkState::Quit
                    }
                })
            });

            send_grep_job("".to_string(), &tx_grep, &GrepInfo { is_cancel: false, is_end: true, ..GrepInfo::default() });
        });
    }

    pub fn is_grep_canceled() -> bool {
        GrepCancelType::Canceled == if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None }
    }
    pub fn is_grep_canceling() -> bool {
        GrepCancelType::Canceling == if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None }
    }
    pub fn is_grep_cancel() -> bool {
        let is_cancel = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        return GrepCancelType::Canceled == is_cancel || GrepCancelType::Canceling == is_cancel;
    }
}

pub fn send_grep_job(grep_str: String, tx_grep: &Sender<(Job, usize)>, grep_info: &GrepInfo) {
    let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str, is_result: grep_info.is_result, is_cancel: grep_info.is_cancel, is_end: grep_info.is_end }), ..Job::default() };
    let _ = tx_grep.send((job, CHANNEL_PRIORITY_LOW));
}

struct SearchSink<'a> {
    tx: Sender<(Job, usize)>,
    path: &'a PathBuf,
}

// 結果を集めるためのコールバックを Sink で実装．マッチ箇所ごとに `matched` が呼ばれる
impl<'a> Sink for SearchSink<'a> {
    type Error = io::Error;
    // `SinkMatch` にマッチ情報が入っている
    fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> std::result::Result<bool, Self::Error> {
        Log::debug_s("Sink.matched");
        if EvtAct::is_grep_cancel() {
            return Ok(false);
        }
        let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str: format!("{}:{}:{}", self.path.to_str().unwrap(), mat.line_number().unwrap_or(0), from_utf8(mat.bytes()).unwrap_or("")), ..JobGrep::default() }), job_evt: None, job_watch: None };
        //
        if let Err(err) = self.tx.send((job, CHANNEL_PRIORITY_LOW)) {
            Log::error("grep matched error", &err);
        }

        Ok(true)
    }
}
