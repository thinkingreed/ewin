use crate::{colors::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn move_row<T: Write>(out: &mut T, term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　EvtAct.move_row");

        match term.curt().editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    if str.is_empty() {
                        term.curt().mbar.set_err(&LANG.not_entered_row_number_to_move);
                        term.curt().mbar.draw_only(out);
                        return EvtActType::Hold;
                    }
                    let row_num: usize = str.parse().unwrap();
                    if row_num > term.curt().editor.buf.len_lines() || row_num == 0 {
                        term.curt().mbar.set_err(&LANG.number_within_current_number_of_rows);
                        term.curt().mbar.draw_only(out);
                        return EvtActType::Hold;
                    }

                    term.curt().editor.cur.y = row_num - 1;
                    term.curt().editor.cur.x = 0;
                    term.curt().editor.cur.disp_x = 0;

                    term.curt().prom.clear();
                    term.curt().state.clear();
                    term.curt().mbar.clear();
                    term.curt().editor.scroll_move_row();
                    term.curt().editor.scroll_horizontal();
                    term.curt().editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn move_row(term: &mut Terminal) {
        term.curt().state.is_move_line = true;
        term.curt().prom.disp_row_num = 3;
        term.set_disp_size();
        let mut cont = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First);
        cont.set_move_row();
        term.curt().prom.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_move_row);
        self.key_desc = format!("{}{}:{}Enter  {}{}:{}Esc{}", Colors::get_default_fg(), &LANG.move_to_specified_row, Colors::get_msg_highlight_fg(), Colors::get_default_fg(), &LANG.close, Colors::get_msg_highlight_fg(), Colors::get_default_fg(),);
        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
    }
}
