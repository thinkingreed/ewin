use crate::{
    bar::statusbar::*,
    ewin_com::{_cfg::lang::lang_cfg::*, file::*, log::*, model::*},
    ewin_editor::model::*,
    ewin_prom::model::*,
    model::*,
};

impl Tab {
    pub fn save(term: &mut Terminal, is_forced: bool) -> ActType {
        let h_file = term.curt_h_file().clone();
        if h_file.filenm == Lang::get().new_file {
            term.curt().prom_save_new_file();
            return ActType::Draw(DParts::All);
        } else {
            if !is_forced {
                // Check if the file has been updated after opening
                let latest_modified_time = File::get_modified_time(&h_file.fullpath);
                if latest_modified_time > h_file.modified_time {
                    Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > h_file.modified_time));
                    term.curt().prom_save_forced(h_file);
                    return ActType::Draw(DParts::All);
                }
            }
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));

            let result = term.curt().editor.buf.write_to(&h_file.fullpath, &h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(Lang::get().cannot_convert_encoding.to_string()));
                    } else {
                        term.curt_h_file().modified_time = File::get_modified_time(&h_file.fullpath);

                        term.curt().prom.clear();
                        term.curt().mbar.clear();
                        if !term.curt().state.is_save_confirm {
                            term.curt().state.clear();
                        }
                        Log::info("Saved file", &h_file.filenm.as_str());
                        if term.curt().editor.state.is_changed {
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
    pub fn prom_search(&mut self) {
        self.state.is_search = true;
        self.prom.search();
    }

    pub fn prom_save_new_file(&mut self) {
        self.state.is_save_new_file = true;
        self.prom.save_new_file();
    }
    pub fn prom_save_forced(&mut self, h_file: HeaderFile) {
        Log::debug_key("Tab::prom_save_forced");
        self.state.is_save_forced = true;
        let last_modified_time = File::get_modified_time(&h_file.fullpath);
        self.prom.save_forced(h_file.modified_time, last_modified_time);
    }

    pub fn prom_save_confirm(term: &mut Terminal) -> bool {
        Log::debug_key("Tab::prom_save_confirm");
        if term.tabs[term.idx].editor.state.is_changed {
            if !term.curt().state.is_nomal() {
                term.clear_curt_tab(true);
            }
            term.curt().prom.save_confirm();
            term.curt().state.is_save_confirm = true;
            return false;
        };
        if term.tabs.len() == 1 {
            return true;
        } else {
            term.del_tab(term.idx);
            // Redraw the previous tab
            term.curt().editor.draw_range = E_DrawRange::All;
            return false;
        }
    }
    pub fn prom_replace(&mut self) {
        self.state.is_replace = true;
        self.prom.replace();
    }
    pub fn prom_open_file(&mut self, open_file_type: OpenFileType) {
        self.state.is_open_file = true;
        self.prom.open_file(open_file_type);
    }
    pub fn prom_move_row(&mut self) {
        self.state.is_move_row = true;
        self.prom.move_row();
    }
    pub fn prom_menu(&mut self) {
        self.state.is_menu = true;
        self.prom.menu();
    }
    pub fn prom_grep(&mut self) {
        self.state.grep.is_grep = true;
        self.prom.grep();
    }
    pub fn prom_enc_nl(&mut self) {
        self.state.is_enc_nl = true;
        self.prom.enc_nl();
    }
    pub fn new() -> Self {
        Tab { editor: Editor::new(), mbar: MsgBar::new(), prom: Prompt::new(), sbar: StatusBar::new(), state: TabState::default() }
    }
}
impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub editor: Editor,
    // pub editor_draw: Draw,
    pub mbar: MsgBar,
    pub prom: Prompt,
    pub sbar: StatusBar,
    pub state: TabState,
}
