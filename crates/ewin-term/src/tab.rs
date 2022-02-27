use ewin_com::_cfg::model::default::Cfg;

use crate::{
    bar::statusbar::*,
    ewin_com::{_cfg::lang::lang_cfg::*, file::*, log::*, model::*},
    ewin_editor::model::*,
    ewin_prom::model::*,
    model::*,
};
use std::{path::Path, time::SystemTime};

impl Tab {
    pub fn save(term: &mut Terminal, save_type: SaveType) -> ActType {
        Log::debug_key("save");
        Log::debug("save_type", &save_type);

        let path = Path::new(&term.curt_h_file().fullpath);
        if save_type != SaveType::NewName && (!path.is_file() || !path.exists()) {
            term.curt().prom_save_new_file();
            return ActType::Render(RParts::All);
        } else {
            match save_type {
                SaveType::Normal => {
                    // Check if the file has been updated after opening
                    if let Some(latest_modified_time) = File::get_modified_time(&term.curt_h_file().fullpath) {
                        if latest_modified_time > term.curt_h_file().modified_time {
                            Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > term.curt_h_file().modified_time));
                            let h_file = term.curt_h_file().clone();
                            term.curt().prom_save_forced(&h_file.modified_time, &h_file.fullpath);
                            return ActType::Render(RParts::All);
                        }
                    }
                }
                SaveType::NewName => {
                    if !term.curt_h_file().fullpath.contains('.') && !Cfg::get().general.editor.save.extension_when_saving_new_file.is_empty() {
                        let extension = &Cfg::get().general.editor.save.extension_when_saving_new_file;
                        term.curt_h_file().fullpath = format!("{}.{}", term.curt_h_file().fullpath, extension);
                        term.curt_h_file().filenm = format!("{}.{}", term.curt_h_file().filenm, extension);
                    }
                }
                SaveType::Forced => {}
            }

            let h_file = term.curt_h_file().clone();
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = term.curt().editor.buf.write_to(&h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Render(RParts::AllMsgBar(Lang::get().cannot_convert_encoding.to_string()));
                    } else {
                        if save_type == SaveType::NewName {
                            Terminal::set_title(&term.curt_h_file().filenm);
                        }
                        term.curt_h_file().modified_time = File::get_modified_time(&term.curt_h_file().fullpath).unwrap();

                        term.curt().prom.clear();
                        term.curt().mbar.clear();
                        if !term.curt().state.is_save_confirm {
                            term.curt().state.clear();
                        }
                        Log::info("Saved file", &term.curt_h_file().filenm.as_str());
                        if term.curt().editor.state.is_changed || save_type == SaveType::NewName {
                            term.curt().editor.state.is_changed = false;
                            return ActType::Render(RParts::All);
                        } else {
                            return ActType::Render(RParts::None);
                        };
                    }
                }
                Err(err) => {
                    Log::error("err", &err.to_string());
                    return ActType::Render(RParts::AllMsgBar(format!("{} {:?}", Lang::get().file_saving_problem, err.kind())));
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
        self.prom.save_new_file(self.editor.get_candidate_new_filenm());
    }

    pub fn prom_save_forced(&mut self, modified_time: &SystemTime, fullpath: &str) {
        Log::debug_key("Tab::prom_save_forced");
        self.state.is_save_forced = true;
        let last_modified_time = File::get_modified_time(fullpath).unwrap();
        self.prom.save_forced(modified_time, last_modified_time);
    }

    pub fn prom_save_confirm(term: &mut Terminal) -> bool {
        Log::debug_key("Tab::prom_save_confirm");
        if term.tabs[term.tab_idx].editor.state.is_changed {
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
            term.del_tab(term.tab_idx);
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
    pub fn prom_watch_result(&mut self) {
        Log::debug_key("Tab::prom_watch_result");
        self.state.is_watch_result = true;
        self.prom.watch_result();
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
