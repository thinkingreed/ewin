use crate::{
    bar::statusbar::*,
    ewin_core::{_cfg::keys::*, global::*, log::*, model::*},
    ewin_editor::model::*,
    ewin_prom::{cont::promptcont::*, prompt::prompt::*},
    model::*,
    terminal::*,
};

impl Tab {
    pub fn new() -> Self {
        Tab { editor: Editor::new(), mbar: MsgBar::new(), prom: Prompt::new(), sbar: StatusBar::new(), state: TabState::default() }
    }

    pub fn save(term: &mut Terminal) -> bool {
        let h_file = term.curt_h_file().clone();
        if h_file.filenm == LANG.new_file {
            term.curt().prom_save_new_file();
            return false;
        } else {
            Log::info_s(&format!("Save {}, file info {:?}", &h_file.filenm, &h_file));
            let result = term.curt().editor.buf.write_to(&h_file.fullpath, &h_file);
            match result {
                Ok(enc_errors) => {
                    if enc_errors {
                        Log::info("Encoding errors", &enc_errors);
                        term.curt().mbar.set_err(&LANG.cannot_convert_encoding);
                    } else {
                        term.curt().editor.is_changed = false;
                        term.curt().prom.clear();
                        term.curt().mbar.clear();
                        if !term.curt().state.is_close_confirm {
                            term.curt().state.clear();
                        }
                        Log::info("Saved file", &h_file.filenm.as_str());
                        return true;
                    }
                }
                Err(err) => {
                    term.curt().mbar.set_err(&format!("{} {:?}", LANG.file_saving_problem, err.kind()));
                    Log::error("err", &err.to_string());
                }
            }
        }
        return false;
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
        if term.tabs[term.idx].editor.is_changed == true {
            if !term.curt().state.is_nomal() {
                term.clear_curt_tab();
            }
            term.curt().prom.disp_row_num = 2;
            term.set_disp_size();
            let keycmd = term.curt().prom.keycmd.clone();
            let mut cont = PromptCont::new_not_edit_type(keycmd);
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
            term.curt().editor.draw_type = DrawType::All;
            return false;
        }
    }
    pub fn prom_replace(&mut self) {
        self.state.is_replace = true;
        self.prom.replace();
    }
    pub fn prom_open_file(&mut self, keycmd: KeyCmd) {
        self.state.is_open_file = true;
        self.prom.open_file(keycmd);
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
        self.state.grep_state.is_grep = true;
        self.prom.grep();
    }
    pub fn prom_enc_nl(&mut self, h_file: &HeaderFile) {
        self.state.is_enc_nl = true;
        self.prom.enc_nl(h_file);
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
