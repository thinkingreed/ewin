use crate::{
    bar::statusbar::*,
    ewin_core::{global::*, log::*, model::*},
    ewin_editor::model::*,
    ewin_prom::model::*,
    model::*,
    terminal::*,
};

impl Tab {
    pub fn save(term: &mut Terminal) -> ActType {
        let h_file = term.curt_h_file().clone();
        if h_file.filenm == LANG.new_file {
            term.curt().prom_save_new_file();
            return ActType::Draw(DParts::All);
        } else {
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = term.curt().editor.buf.write_to(&h_file.fullpath, &h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        return ActType::Draw(DParts::AllMsgBar(LANG.cannot_convert_encoding.to_string()));
                    } else {
                        term.curt().editor.state.is_changed = false;
                        term.curt().prom.clear();
                        term.curt().mbar.clear();
                        if !term.curt().state.is_close_confirm {
                            term.curt().state.clear();
                        }
                        Log::info("Saved file", &h_file.filenm.as_str());
                        // return true;
                        return ActType::Next;
                    }
                }
                Err(err) => {
                    Log::error("err", &err.to_string());
                    return ActType::Draw(DParts::AllMsgBar(format!("{} {:?}", LANG.file_saving_problem, err.kind())));
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

    pub fn prom_close(term: &mut Terminal) -> bool {
        Log::debug_key("Tab::prom_close");
        if term.tabs[term.idx].editor.state.is_changed == true {
            if !term.curt().state.is_nomal() {
                term.clear_curt_tab(true);
            }
            term.curt().prom.disp_row_num = 2;
            term.set_disp_size();
            let mut cont = PromptCont::new(None);
            cont.set_save_confirm();
            term.curt().prom.cont_1 = cont;
            term.curt().state.is_close_confirm = true;
            return false;
        };
        if term.tabs.len() == 1 {
            return true;
        } else {
            term.del_tab(term.idx);
            // Redraw the previous tab
            term.curt().editor.draw_range = EditorDrawRange::All;
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

#[derive(Debug, Clone)]
pub struct Tab {
    pub editor: Editor,
    // pub editor_draw: Draw,
    pub mbar: MsgBar,
    pub prom: Prompt,
    pub sbar: StatusBar,
    pub state: TabState,
}
