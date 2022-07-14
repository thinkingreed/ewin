#![allow(clippy::needless_return, clippy::iter_nth_zero)]
use crate::{
    ewin_com::{_cfg::key::cmd::*, global::*, model::*},
    model::*,
    tab::Tab,
    terms::term::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*, model::default::*};
use globset::Glob;
use grep::cli;
use grep_regex::RegexMatcherBuilder;
use grep_searcher::{BinaryDetection, MmapChoice, Searcher, SearcherBuilder, Sink, SinkMatch};
use ignore::{WalkBuilder, WalkState};
use std::{env, ffi::OsStr, io, path::*, str::from_utf8, sync::mpsc::Sender};

impl EvtAct {
    pub fn grep(term: &mut Terminal) -> ActType {
        Log::debug_s("EvtAct.grep");
        match term.curt().prom.cmd.cmd_type {
            CmdType::Confirm => {
                let search_str = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                let search_filenm = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(1);
                let mut search_dir = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(2);

                Log::debug("search_str", &search_str);
                Log::debug("search_filenm", &search_filenm);
                Log::debug("search_folder", &search_dir);

                if search_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_set_search_str.to_string()));
                } else if search_filenm.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_search_file.to_string()));
                } else if search_dir.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_search_folder.to_string()));
                } else {
                    term.curt().clear_curt_tab(true);

                    // if search_folder.chars().nth(0).unwrap() != '/' && search_folder.chars().nth(0).unwrap() != 'C' {
                    if Path::new(&search_dir).is_relative() {
                        let current_dir = env::current_dir().unwrap().display().to_string();
                        search_dir = format!("{}/{}", current_dir, search_dir);
                    }
                    Log::debug_s(&search_dir);
                    let path = Path::new(&search_dir).join(&search_filenm);

                    // TODO cache
                    /*
                    term.curt().prom.prom_grep.cache_search_filenm = search_filenm.clone();
                    term.curt().prom.prom_grep.cache_search_folder = search_folder.clone();
                     */
                    let mut grep_tab = Tab::new();
                    grep_tab.editor.search.set_info(search_str.clone(), path.to_string_lossy().to_string(), search_dir.clone());
                    grep_tab.editor.cmd = Cmd::to_cmd(CmdType::GrepResultProm);

                    grep_tab.state.grep.search_str = search_str.clone();

                    term.add_tab(&mut grep_tab.clone(), HeaderFile::new(&format!(r#"{} "{}""#, &Lang::get().grep, &search_str)), FileOpenType::Nomal);
                    GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().push(GrepCancelType::Greping);
                    if let Some(Ok(tx)) = TX_JOB.get().map(|tx| tx.try_lock()) {
                        EvtAct::watching_grep(tx.clone(), search_str, search_filenm, search_dir);
                    }

                    term.curt().prom_show_com(&CmdType::GrepingProm);

                    // Clear(ClearType::CurrentLine) is not performed during grep to prevent flicker. Therefore, clear first
                    Terminal::clear_all();
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn watching_grep(tx_grep: Sender<Job>, search_str: String, search_filenm: String, search_dir: String) {
        Log::debug_s("EvtAct.watching_grep");
        tokio::spawn(async move {
            let cfg_search = CfgEdit::get_search();
            Log::debug("cfg_search", &cfg_search);

            let matcher = RegexMatcherBuilder::new().dot_matches_new_line(false).case_insensitive(!cfg_search.case_sensitive).build(cli::pattern_from_os(OsStr::new(&if cfg_search.regex { search_str } else { regex::escape(&search_str) })).unwrap_or("")).unwrap();
            let searcher = SearcherBuilder::new()
                .binary_detection(BinaryDetection::quit(0)) // Binary not applicable
                .line_number(true)
                .memory_map(unsafe { MmapChoice::auto() })
                .bom_sniffing(false)
                .build();

            let glob = Glob::new(&search_filenm).unwrap().compile_matcher();
            let walker = WalkBuilder::new(&search_dir)
                .hidden(true) // Search for hidden files
                .ignore(false) // Also target files that have been ignored
                .parents(false) // Don't go through the parent directory to find .gitignore
                // Sort in ascending order
                .sort_by_file_path(|a, b| a.cmp(b))
                .build_parallel();

            walker.run(|| {
                // For each thread in the thread pool
                let tx_grep = tx_grep.clone();
                let matcher = matcher.clone();
                let mut searcher = searcher.clone();
                let glob = glob.clone();

                Box::new(move |result| match result {
                    // This inner callback is called per file path
                    // For files
                    Ok(entry) if glob.is_match(entry.file_name()) => {
                        // if entry.file_type().map(|t| t.is_file()).unwrap_or(false) => {
                        Log::debug("entry.file_name()", &entry.file_name());
                        Log::debug("entry.path()", &entry.path());

                        // Check cancel
                        if EvtAct::is_grep_cancel() {
                            Log::debug_s("is_grep_cancel");
                            return WalkState::Quit;
                        } else if let Err(err) = searcher.search_path(&matcher, entry.path(), SearchSink { tx: tx_grep.clone(), path: entry.path() }) {
                            Log::error("grep error", &format!("{}:{}", &err, &entry.path().display()));
                        };
                        WalkState::Continue
                    }
                    //  For directory
                    Ok(_) => WalkState::Continue,
                    Err(err) => {
                        Log::error("grep error", &err);
                        WalkState::Continue
                    }
                })
            });
            send_grep_job("".to_string(), &tx_grep, true);
        });
    }

    pub fn is_grep_canceled() -> bool {
        let state = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceled == state || GrepCancelType::None == state
    }
    pub fn is_grep_canceling() -> bool {
        let state = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceling == state
    }
    pub fn is_grep_cancel() -> bool {
        return EvtAct::is_grep_canceling() || EvtAct::is_grep_canceled();
    }
}

pub fn send_grep_job(grep_str: String, tx_grep: &Sender<Job>, is_end: bool) {
    let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str, is_end }), ..Job::default() };
    let _ = tx_grep.send(job);
}
struct SearchSink<'a> {
    tx: Sender<Job>,
    path: &'a Path,
}

// Callback Sink to collect results. `matched` is called for each match
impl<'a> Sink for SearchSink<'a> {
    type Error = io::Error;
    // `SinkMatch` contains match information
    fn matched(&mut self, _searcher: &Searcher, mat: &SinkMatch<'_>) -> std::result::Result<bool, Self::Error> {
        Log::debug_s("Sink.matched");
        if EvtAct::is_grep_cancel() {
            Log::debug_s("EvtAct::is_grep_cancel()");
            return Ok(false);
        }

        let job = Job { job_type: JobType::GrepResult, job_grep: Some(JobGrep { grep_str: format!("{}:{}:{}", self.path.to_str().unwrap(), mat.line_number().unwrap_or(0), from_utf8(mat.bytes()).unwrap_or(""),), ..JobGrep::default() }), job_evt: None, job_watch: None };
        if let Err(err) = self.tx.send(job) {
            Log::error("grep matched error", &err);
        }
        Ok(true)
    }
}
