use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_key::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::default::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_job::{global::*, job::*};
use ewin_key::{global::*, model::*};
use ewin_state::term::*;
use globset::Glob;
use grep::cli;
use grep_regex::RegexMatcherBuilder;
use grep_searcher::{BinaryDetection, MmapChoice, Searcher, SearcherBuilder, Sink, SinkMatch};
use ignore::{WalkBuilder, WalkState};
use std::{env, ffi::OsStr, io, path::Path, str::from_utf8, sync::mpsc::Sender};

impl PromGrep {
    pub fn grep(&mut self) -> ActType {
        Log::debug_s("EvtAct.grep");
        match self.base.cmd.cmd_type {
            CmdType::Confirm => {
                let search_str = self.as_mut_base().get_tgt_input_area_str(0);
                let search_filenm = self.as_mut_base().get_tgt_input_area_str(1);
                let mut search_dir = self.as_mut_base().get_tgt_input_area_str(2);

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
                    if Path::new(&search_dir).is_relative() {
                        let current_dir = env::current_dir().unwrap().display().to_string();
                        search_dir = format!("{}/{}", current_dir, search_dir);
                    }
                    State::get().curt_mut_state().clear();
                    Job::send_cmd(CmdType::GrepingProm(GrepInfo { search_str, search_filenm, search_dir, ..GrepInfo::default() }));
                }
                return ActType::None;
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn new() -> Self {
        let mut prom = PromGrep { base: PromBase { cfg: PromptConfig { is_updown_valid: true }, ..PromBase::default() } };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_grep.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let search = PromContKeyMenu { disp_str: Lang::get().search.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let switch_input_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown], &mut vec![CmdType::BackContent]) };
        let complement = PromContKeyMenu { disp_str: Lang::get().complement.to_string(), key: PromContKeyMenuType::PCmdAndStr(CmdType::NextContent, format!("({})", Lang::get().search_folder)) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![search, switch_input_area, complement, cancel]], ..PromContKeyDesc::default() }));

        prom.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        prom.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_str.to_string()], buf: vec![], ..PromContInputArea::default() }));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        // search_file
        // TODO cache
        prom.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_file.to_string()], buf: "*.*".chars().collect(), ..PromContInputArea::default() }));

        // search_folder
        let mut search_folder = PromContInputArea { desc_str_vec: vec![Lang::get().search_folder.to_string()], buf: vec![], config: PromInputAreaConfig { is_path: true, is_path_dir_only: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        // TODO cache
        if let Ok(path) = env::current_dir() {
            search_folder.buf = path.to_string_lossy().to_string().chars().collect();
        };
        prom.base.cont_vec.push(Box::new(search_folder));

        return prom;
    }
}
impl Grep {
    pub fn exe_grep(grep_info: GrepInfo) {
        Log::debug_s("EvtAct.watching_grep");
        if let Some(Ok(tx_job)) = TX_JOB.get().map(|tx| tx.try_lock()) {
            tokio::spawn(async move {
                let cfg_search = CfgEdit::get_search();
                Log::debug("cfg_search", &cfg_search);

                let matcher = RegexMatcherBuilder::new().dot_matches_new_line(false).case_insensitive(!cfg_search.case_sensitive).build(cli::pattern_from_os(OsStr::new(&if cfg_search.regex { grep_info.search_str } else { regex::escape(&grep_info.search_str) })).unwrap_or("")).unwrap();
                let searcher = SearcherBuilder::new()
                    .binary_detection(BinaryDetection::quit(0)) // Binary not applicable
                    .line_number(true)
                    .memory_map(unsafe { MmapChoice::auto() })
                    .bom_sniffing(false)
                    .build();

                let glob = Glob::new(&grep_info.search_filenm).unwrap().compile_matcher();
                let walker = WalkBuilder::new(&grep_info.search_dir)
                    .hidden(true) // Search for hidden files
                    .ignore(false) // Also target files that have been ignored
                    .parents(false) // Don't go through the parent directory to find .gitignore
                    // Sort in ascending order
                    .sort_by_file_path(|a, b| a.cmp(b))
                    .build_parallel();

                walker.run(|| {
                    // For each thread in the thread pool
                    let tx_job = tx_job.clone();
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
                            if Grep::is_cancel() {
                                Log::debug_s("is_grep_cancel");
                                return WalkState::Quit;
                            } else if let Err(err) = searcher.search_path(&matcher, entry.path(), SearchSink { tx: tx_job.clone(), path: entry.path() }) {
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
                Job::send_grep(&tx_job, "".to_string(), true);
            });
        }
    }

    pub fn is_canceled() -> bool {
        let state = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceled == state || GrepCancelType::None == state
    }
    pub fn is_canceling() -> bool {
        let state = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
        GrepCancelType::Canceling == state
    }
    pub fn is_cancel() -> bool {
        {
            let state = if let Some(Ok(grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|grep_cancel_vec| grep_cancel_vec.try_lock()) { grep_cancel_vec[grep_cancel_vec.len() - 1] } else { GrepCancelType::None };
            Log::debug("state", &state);
        }

        return Grep::is_canceling() || Grep::is_canceled();
    }
}

#[derive(Default, Debug, Clone)]
pub struct Grep {}
#[derive(Default, Debug, Clone)]
pub struct PromGrep {
    pub base: PromBase,
}
impl PromTrait for PromGrep {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
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
        if Grep::is_cancel() {
            Log::debug_s("EvtAct::is_grep_cancel()");
            return Ok(false);
        }
        let job = Job { cont: JobCont::Grep(JobGrep { grep_str: format!("{}:{}:{}", self.path.to_str().unwrap(), mat.line_number().unwrap_or(0), from_utf8(mat.bytes()).unwrap_or(""),), ..JobGrep::default() }) };
        if let Err(err) = self.tx.send(job) {
            Log::error("grep matched error", &err);
        }
        Ok(true)
    }
}
