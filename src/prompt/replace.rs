use crate::{
    _cfg::keys::{KeyCmd, Keybind},
    colors::*,
    global::*,
    model::*,
    prompt::cont::promptcont::*,
    prompt::prompt::prompt::*,
    terminal::Terminal,
};

impl EvtAct {
    pub fn replace(term: &mut Terminal) -> EvtActType {
        match term.curt().editor.keycmd {
            KeyCmd::InsertLine => {
                let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let replace_str = term.curt().prom.cont_2.buf.iter().collect::<String>();

                if search_str.is_empty() {
                    term.curt().mbar.set_err(&LANG.not_entered_search_str);
                // } else if replace_str.is_empty() {
                //     term.curt().mbar.set_err(&LANG.not_entered_replace_str);
                } else {
                    let end_idx = term.curt().editor.buf.len_chars();
                    let search_set = term.curt().editor.buf.search(&search_str.clone(), 0, end_idx);
                    if search_set.len() == 0 {
                        term.curt().mbar.set_err(&LANG.cannot_find_char_search_for);
                        return EvtActType::DrawOnly;
                    }
                    REPLACE_SEARCH_RANGE.get().unwrap().try_lock().unwrap().push(search_set);

                    term.clear_curt_tab();
                    term.curt().editor.exec_edit_proc(EvtType::Replace, &search_str, &replace_str);

                    term.hbar.file_vec[term.idx].is_changed = true;
                }
                term.curt().editor.d_range.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(term: &mut Terminal) {
        term.curt().state.is_replace = true;
        term.curt().prom.disp_row_num = 7;
        term.set_disp_size();
        let mut cont_1 = PromptCont::new_edit_type(term.curt(), PromptContPosi::First);
        let mut cont_2 = PromptCont::new_edit_type(term.curt(), PromptContPosi::Second);
        cont_1.set_replace();
        cont_2.set_replace();
        term.curt().prom.cont_1 = cont_1;
        term.curt().prom.cont_2 = cont_2;
    }
    pub fn draw_replace(&self, str_vec: &mut Vec<String>) {
        Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());
    }
}

impl PromptCont {
    pub fn set_replace(&mut self) {
        let base_posi = self.disp_row_posi;

        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_replace);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}↓↑  {}{}:{}{}",
                Colors::get_default_fg(),
                &LANG.all_replace,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::ConfirmPrompt),
                Colors::get_default_fg(),
                &LANG.move_setting_location,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::EscPrompt),
            );
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.search_str,);

            self.guide_row_posi = base_posi;
            self.key_desc_row_posi = base_posi + 1;
            self.opt_row_posi = base_posi + 2;
            self.buf_desc_row_posi = base_posi + 3;
            self.buf_row_posi = base_posi + 4;
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.replace_char,);

            self.buf_desc_row_posi = base_posi + 5;
            self.buf_row_posi = base_posi + 6;
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
