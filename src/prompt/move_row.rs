use crate::{
    _cfg::keys::{KeyCmd, Keybind},
    colors::*,
    global::*,
    log::*,
    model::*,
    prompt::cont::promptcont::*,
    prompt::prompt::prompt::*,
    terminal::Terminal,
};

impl EvtAct {
    pub fn move_row(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct.move_row");
        match term.curt().editor.keycmd {
            KeyCmd::InsertChar(c) => {
                if !c.is_ascii_digit() {
                    return EvtActType::Hold;
                }
                let str: String = term.curt().prom.cont_1.buf.iter().collect::<String>();
                if str.chars().count() == term.curt().editor.get_rnw() {
                    return EvtActType::Hold;
                }
                term.curt().prom.insert_char(c);

                return EvtActType::DrawOnly;
            }
            KeyCmd::InsertLine => {
                let str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                if str.is_empty() {
                    term.curt().mbar.set_err(&LANG.not_entered_row_number_to_move);
                    return EvtActType::Hold;
                }
                let row_num: usize = str.parse().unwrap();
                if row_num > term.curt().editor.buf.len_lines() || row_num == 0 {
                    term.curt().mbar.set_err(&LANG.number_within_current_number_of_rows);
                    return EvtActType::Hold;
                }
                term.curt().editor.cur.y = row_num - 1;
                term.curt().editor.cur.x = 0;
                term.curt().editor.cur.disp_x = 0;

                term.clear_curt_tab();
                term.curt().editor.move_row();
                term.curt().editor.scroll_horizontal();
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn move_row(term: &mut Terminal) {
        term.curt().state.is_move_row = true;
        term.curt().prom.disp_row_num = 3;
        term.set_disp_size();
        let mut cont = PromptCont::new_edit_type(term.curt(), PromptContPosi::First);
        cont.set_move_row();
        term.curt().prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_move_row);
        self.key_desc = format!("{}{}:{}{}  {}{}:{}{}{}", Colors::get_default_fg(), &LANG.move_to_specified_row, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::ConfirmPrompt), Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(), Keybind::get_key_str(KeyCmd::EscPrompt), Colors::get_default_fg(),);
        let base_posi = self.disp_row_posi;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
    }
}
