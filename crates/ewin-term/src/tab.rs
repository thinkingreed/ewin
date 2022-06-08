use crate::{
    bar::statusbar::*,
    ewin_com::{files::file::*, model::*},
    ewin_editor::model::*,
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_prom::{
    cont::parts::pulldown::*,
    each::{enc_nl::*, grep::*, grep_result::*, greping::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*},
    model::*,
};
use ewin_widget::widget::pulldown::*;
use std::{cmp::min, path::Path, time::SystemTime};

impl Tab {
    pub fn save(term: &mut Terminal, save_type: SaveType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let path = Path::new(&term.curt_h_file().fullpath);
        if save_type != SaveType::NewName && (!path.is_file() || !path.exists()) {
            term.curt().prom_save_new_file();
            return ActType::Draw(DParts::All);
        } else {
            match save_type {
                SaveType::Normal => {
                    // Check if the file has been updated after opening
                    if let Some(latest_modified_time) = File::get_modified_time(&term.curt_h_file().fullpath) {
                        if latest_modified_time > term.curt_h_file().mod_time {
                            Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > term.curt_h_file().mod_time));
                            let h_file = term.curt_h_file().clone();
                            term.curt().prom_save_forced(&h_file.mod_time, &h_file.fullpath);
                            return ActType::Draw(DParts::All);
                        }
                    }
                }
                SaveType::NewName | SaveType::Forced => {}
            }

            let h_file = term.curt_h_file().clone();
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = term.curt().editor.buf.write_to(&h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(Lang::get().writing_cannot_convert_encoding.to_string()));
                    } else {
                        if save_type == SaveType::NewName {
                            Terminal::set_title(&term.curt_h_file().filenm);
                        }
                        term.curt_h_file().mod_time = File::get_modified_time(&term.curt_h_file().fullpath).unwrap();

                        term.curt().prom.clear();
                        term.curt().msgbar.clear();
                        /*
                        if !term.curt().state.is_save_confirm {
                            term.curt().state.clear();
                        }
                         */
                        term.curt().state.clear();

                        Log::info("Saved file", &term.curt_h_file());
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

    pub fn prom_search(&mut self) -> ActType {
        self.state.prom = PromState::Search;
        self.prom.init(Box::new(PromSearch::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_save_new_file(&mut self) {
        Log::debug_key("Tab::prom_save_new_file");
        self.state.prom = PromState::SaveNewFile;
        self.prom.init(Box::new(PromSaveNewFile::new(self.editor.get_candidate_new_filenm())));
        if let Ok(pulldown_cont) = self.prom.curt.as_mut_base().get_tgt_cont(3).unwrap().downcast_mut::<PromContPulldown>() {
            let height = min(pulldown_cont.pulldown.widget.cont.cont_vec.len(), Editor::get_disp_row_num());
            pulldown_cont.pulldown.widget.init_menu(pulldown_cont.base.row_posi_range.end, Pulldown::MARGIN, height);
        }
    }

    pub fn prom_save_forced(&mut self, modified_time: &SystemTime, fullpath: &str) -> ActType {
        Log::debug_key("Tab::prom_save_forced");
        let last_modified_time = File::get_modified_time(fullpath).unwrap();
        self.state.prom = PromState::SaveForced;
        self.prom.init(Box::new(PromSaveForced::new(modified_time, last_modified_time)));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_save_confirm(term: &mut Terminal) -> bool {
        Log::debug_key("Tab::prom_save_confirm");
        if term.curt().editor.state.is_changed {
            if !term.curt().state.is_nomal() {
                term.curt().clear_curt_tab(true);
            }

            term.curt().prom.init(Box::new(PromSaveConfirm::new()));
            // term.curt().state.is_save_confirm = true;
            term.curt().state.prom = PromState::SaveConfirm;
            return false;
        };
        if term.tabs.len() == 1 {
            return true;
        } else {
            term.del_tab(term.tab_idx);
            // Redraw the previous tab
            term.curt().editor.draw_range = E_DrawRange::All;
            return false;
        }
    }

    pub fn prom_replace(&mut self) -> ActType {
        self.state.prom = PromState::Replase;
        self.prom.init(Box::new(PromReplace::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_open_file(&mut self, open_file_type: OpenFileType) -> ActType {
        self.state.prom = PromState::OpenFile;
        self.prom.init(Box::new(PromOpenFile::new(open_file_type)));
        return ActType::Draw(DParts::All);
    }
    pub fn prom_move_row(&mut self) -> ActType {
        self.state.prom = PromState::MoveRow;
        self.prom.init(Box::new(PromMoveRow::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn prom_grep(&mut self) -> ActType {
        self.state.prom = PromState::Grep;
        self.prom.init(Box::new(PromGrep::new()));
        return ActType::Draw(DParts::All);
    }
    pub fn prom_greping(&mut self) {
        self.state.prom = PromState::Greping;
        self.prom.init(Box::new(PromGreping::new()));
    }
    pub fn prom_grep_result(&mut self) {
        self.state.prom = PromState::GrepResult;
        self.prom.init(Box::new(PromGrepResult::new(self.state.grep.is_empty, self.state.grep.is_cancel)));
    }
    pub fn prom_enc_nl(&mut self, h_file: &HeaderFile) -> ActType {
        self.state.prom = PromState::EncNl;
        self.prom.init(Box::new(PromEncNl::new()));
        self.prom.curt.downcast_mut::<PromEncNl>().unwrap().set_default_choice_enc_nl(h_file);
        return ActType::Draw(DParts::All);
    }
    pub fn prom_watch_result(&mut self) -> ActType {
        Log::debug_key("Tab::prom_watch_result");
        self.state.prom = PromState::WatchFile;
        self.prom.init(Box::new(PromWatchFile::new()));
        return ActType::Draw(DParts::All);
    }

    pub fn new() -> Self {
        Tab { editor: Editor::new(), msgbar: MsgBar::new(), prom: Prom::default(), sbar: StatusBar::new(), state: TabState::default() }
    }
    pub fn clear_curt_tab(&mut self, is_clear_editor_state: bool) {
        Log::debug_key("clear_curt_tab");
        self.prom.clear();

        self.state.clear();
        if self.is_grep_result_state() {
            self.prom_grep_result();
        }

        self.msgbar.clear();
        // self.set_disp_size();
        if is_clear_editor_state {
            self.editor.cancel_state();
        }
        self.editor.draw_range = E_DrawRange::All;
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
}
