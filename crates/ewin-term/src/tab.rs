use crate::{
    bar::statusbar::*,
    ewin_com::{files::file::*, model::*},
    ewin_editor::model::*,
    global_term::*,
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::{_cfg::key::cmd::*, util::*};
use ewin_const::def::*;
use ewin_prom::{
    cont::parts::pulldown::*,
    each::{enc_nl::*, grep::*, grep_result::*, greping::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*},
    model::*,
};
use ewin_widget::widget::pulldown::*;
use std::{cmp::min, path::Path};

impl Tab {
    pub fn save(term: &mut Terminal, save_type: SaveType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let mut vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
        let path = Path::new(&vec.get(term.tab_idx).unwrap().fullpath);
        if save_type != SaveType::NewName && (!path.is_file() || !path.exists()) {
            return term.curt().prom_save_new_file();
        } else {
            match save_type {
                SaveType::Normal => {
                    // Check if the file has been updated after opening
                    if let Some(latest_modified_time) = File::get_modified_time(&vec.get_mut(term.tab_idx).unwrap().fullpath) {
                        if latest_modified_time > vec.get_mut(term.tab_idx).unwrap().mod_time {
                            Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > vec.get_mut(term.tab_idx).unwrap().mod_time));
                            return term.curt().prom_show_com(&CmdType::Saveforced);
                        }
                    }
                }
                SaveType::NewName | SaveType::Forced => {}
            }

            let h_file = vec.get_mut(term.tab_idx).unwrap().clone();
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = term.curt().editor.buf.write_to(&h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(Lang::get().writing_cannot_convert_encoding.to_string()));
                    } else {
                        if save_type == SaveType::NewName {
                            Terminal::set_title(&vec.get_mut(term.tab_idx).unwrap().filenm);
                        }
                        vec.get_mut(term.tab_idx).unwrap().mod_time = File::get_modified_time(&vec.get_mut(term.tab_idx).unwrap().fullpath).unwrap();

                        term.curt().prom.clear();
                        term.curt().state.clear();

                        Log::info("Saved file", &vec.get_mut(term.tab_idx).unwrap());
                        if term.curt().editor.state.is_changed || save_type == SaveType::NewName || save_type == SaveType::Forced {
                            term.curt().editor.state.is_changed = false;
                            return ActType::Draw(DParts::All);
                        } else {
                            return ActType::Draw(DParts::None);
                        };
                    }
                }
                Err(err) => {
                    Log::error("err", &err.to_string());
                    return ActType::Draw(DParts::AllMsgBar(format!("{} {:?}", Lang::get().file_saving_problem, err.kind())));
                }
            }
        }
    }

    pub fn prom_show_com(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("Tab::prom_show_com");
        Log::debug("cmd_type", &cmd_type);
        self.prom.row_bottom_posi = get_term_size().1 - STATUSBAR_ROW_NUM - if HELP_DISP.get().unwrap().try_lock().unwrap().is_disp { HELP_DISP.get().unwrap().try_lock().unwrap().row_num } else { 0 };
        match cmd_type {
            CmdType::FindProm => return self.prom_search(),
            CmdType::ReplaceProm => return self.prom_replace(),
            CmdType::GrepProm => return self.prom_grep(),
            CmdType::GrepingProm => return self.prom_greping(),
            CmdType::GrepResultProm => return self.prom_grep_result(),
            CmdType::MoveRowProm => return self.prom_move_row(),
            CmdType::EncodingProm => return self.prom_enc_nl(),
            CmdType::openFileProm(open_file_type) => return self.prom_open_file(open_file_type),
            CmdType::CloseFile => return self.prom_save_confirm(),
            CmdType::SaveNewFile => return self.prom_save_new_file(),
            CmdType::Saveforced => return self.prom_save_forced(),
            CmdType::WatchFileResult => return self.prom_watch_result(),
            _ => ActType::Cancel,
        };

        return ActType::Cancel;
    }
    fn prom_search(&mut self) -> ActType {
        self.state.prom = PromState::Search;
        self.prom.init(Box::new(PromSearch::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_new_file(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_new_file");
        self.state.prom = PromState::SaveNewFile;
        self.prom.init(Box::new(PromSaveNewFile::new(self.editor.get_candidate_new_filenm())));
        if let Ok(pulldown_cont) = self.prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
            let height = min(pulldown_cont.pulldown.widget.cont.cont_vec.len(), Editor::get_disp_row_num());
            pulldown_cont.pulldown.widget.init_menu(pulldown_cont.base.row_posi_range.end, Pulldown::MARGIN, height);
        }
        return ActType::Draw(DParts::All);
    }

    fn prom_save_forced(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_forced");
        let vec = H_FILE_VEC.get().unwrap().try_lock().unwrap();
        let h_file = vec.get(self.idx).unwrap();
        let last_modified_time = File::get_modified_time(&h_file.fullpath).unwrap();
        self.state.prom = PromState::SaveForced;
        self.prom.init(Box::new(PromSaveForced::new(&h_file.mod_time, last_modified_time)));
        return ActType::Draw(DParts::All);
    }

    fn prom_save_confirm(&mut self) -> ActType {
        Log::debug_key("Tab::prom_save_confirm");
        if self.editor.state.is_changed {
            if !self.state.is_nomal() {
                self.clear_curt_tab(true);
            }
            self.prom.init(Box::new(PromSaveConfirm::new()));
            // term.curt().state.is_save_confirm = true;
            self.state.prom = PromState::SaveConfirm;
            return ActType::Draw(DParts::All);
        };
        return ActType::Next;
    }

    fn prom_replace(&mut self) -> ActType {
        self.state.prom = PromState::Replase;
        self.prom.init(Box::new(PromReplace::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_open_file(&mut self, open_file_type: &OpenFileType) -> ActType {
        self.state.prom = PromState::OpenFile;
        self.prom.init(Box::new(PromOpenFile::new(open_file_type)));
        return ActType::Draw(DParts::All);
    }
    fn prom_move_row(&mut self) -> ActType {
        self.state.prom = PromState::MoveRow;
        self.prom.init(Box::new(PromMoveRow::new()));
        return ActType::Draw(DParts::All);
    }

    fn prom_grep(&mut self) -> ActType {
        self.state.prom = PromState::Grep;
        self.prom.init(Box::new(PromGrep::new()));
        return ActType::Draw(DParts::All);
    }
    fn prom_greping(&mut self) -> ActType {
        self.state.prom = PromState::Greping;
        self.prom.init(Box::new(PromGreping::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_grep_result(&mut self) -> ActType {
        self.state.prom = PromState::GrepResult;
        self.prom.init(Box::new(PromGrepResult::new(self.state.grep.is_empty, self.state.grep.is_cancel)));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_enc_nl(&mut self) -> ActType {
        Log::debug_key("Tab::prom_enc_nl");
        self.state.prom = PromState::EncNl;
        self.prom.init(Box::new(PromEncNl::new()));
        self.prom.curt.downcast_mut::<PromEncNl>().unwrap().set_default_choice_enc_nl(H_FILE_VEC.get().unwrap().try_lock().unwrap().get(self.idx).unwrap());
        return ActType::Draw(DParts::All);
    }
    fn prom_watch_result(&mut self) -> ActType {
        Log::debug_key("Tab::prom_watch_result");
        self.state.prom = PromState::WatchFile;
        self.prom.init(Box::new(PromWatchFile::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn clear_curt_tab(&mut self, is_clear_editor_state: bool) {
        Log::debug_key("clear_curt_tab");
        self.prom.clear();
        self.state.clear();
        self.msgbar.clear();
        if is_clear_editor_state {
            self.editor.cancel_state();
        }
        if self.is_grep_result_state() {
            let _ = self.prom_show_com(&CmdType::GrepResultProm);
        };
    }

    pub fn is_grep_result_state(&mut self) -> bool {
        return !self.state.grep.search_str.is_empty();
    }

    pub fn is_prom_pulldown(&self) -> bool {
        if self.state.prom != PromState::None {
            for cont in self.prom.curt.as_base().cont_vec.iter() {
                if let Ok(pulldown_cont) = cont.downcast_ref::<PromContPulldown>() {
                    if pulldown_cont.pulldown.is_disp {
                        return true;
                    }
                }
            }
        }
        return false;
    }
    pub fn new() -> Self {
        Tab { idx: 0, editor: Editor::new(), msgbar: MsgBar::new(), prom: Prom::default(), sbar: StatusBar::new(), state: TabState::default() }
    }
}
