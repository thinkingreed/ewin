use crate::{colors::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn move_row<T: Write>(out: &mut T, tab: &mut Tab) -> EvtActType {
        Log::ep_s("　　　　　　　　EvtAct.move_row");

        match tab.editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let str = tab.prom.cont_1.buf.iter().collect::<String>();
                    if str.is_empty() {
                        tab.mbar.set_err(&LANG.not_entered_row_number_to_move);
                        tab.mbar.draw_only(out);
                        return EvtActType::Hold;
                    }
                    let row_num: usize = str.parse().unwrap();
                    if row_num > tab.editor.buf.len_lines() || row_num == 0 {
                        tab.mbar.set_err(&LANG.number_within_current_number_of_rows);
                        tab.mbar.draw_only(out);
                        return EvtActType::Hold;
                    }

                    tab.editor.cur.y = row_num - 1;
                    tab.editor.cur.x = tab.editor.get_rnw();
                    tab.editor.cur.disp_x = tab.editor.get_rnw() + 1;

                    tab.prom.clear();
                    tab.state.clear();
                    tab.mbar.clear();
                    tab.editor.scroll_move_row();
                    tab.editor.scroll_horizontal();
                    tab.editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn move_row(&mut self) {
        self.is_move_line = true;
        self.disp_row_num = 3;
        let mut cont = PromptCont::new_edit(self.disp_row_posi as u16, PromptContPosi::First);
        cont.set_move_row();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_move_row);
        self.key_desc = format!(
            "{}{}:{}Enter  {}{}:{}Esc{}",
            Colors::get_default_fg(),
            &LANG.move_to_specified_row,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );
        let base_posi = self.disp_row_posi - 1;
        self.guide_row_posi = base_posi;
        self.key_desc_row_posi = base_posi + 1;
        self.buf_row_posi = base_posi + 2;
    }
}
