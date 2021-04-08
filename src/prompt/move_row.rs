use crate::{colors::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, terminal::Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn move_row<T: Write>(out: &mut T, term: &mut Terminal) -> EvtActType {
        Log::ep_s("　　　　　　　　EvtAct.move_row");

        match term.tabs[term.idx].editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let str = term.tabs[term.idx].prom.cont_1.buf.iter().collect::<String>();
                    if str.is_empty() {
                        term.tabs[term.idx].mbar.set_err(&LANG.not_entered_row_number_to_move);
                        term.tabs[term.idx].mbar.draw_only(out);
                        return EvtActType::Hold;
                    }
                    let row_num: usize = str.parse().unwrap();
                    if row_num > term.tabs[term.idx].editor.buf.len_lines() || row_num == 0 {
                        term.tabs[term.idx].mbar.set_err(&LANG.number_within_current_number_of_rows);
                        term.tabs[term.idx].mbar.draw_only(out);
                        return EvtActType::Hold;
                    }

                    term.tabs[term.idx].editor.cur.y = row_num - 1;
                    term.tabs[term.idx].editor.cur.x = term.tabs[term.idx].editor.get_rnw();
                    term.tabs[term.idx].editor.cur.disp_x = term.tabs[term.idx].editor.get_rnw() + 1;

                    term.tabs[term.idx].prom.clear();
                    term.tabs[term.idx].state.clear();
                    term.tabs[term.idx].mbar.clear();
                    term.tabs[term.idx].editor.scroll_move_row();
                    term.tabs[term.idx].editor.scroll_horizontal();
                    term.tabs[term.idx].editor.d_range.draw_type = DrawType::All;
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
        term.tabs[term.idx].prom.is_move_line = true;
        term.tabs[term.idx].prom.disp_row_num = 3;
        let mut cont = PromptCont::new_edit(term.tabs[term.idx].prom.disp_row_posi as u16, PromptContPosi::First);
        cont.set_move_row();
        term.tabs[term.idx].prom.cont_1 = cont;
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
