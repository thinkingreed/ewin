use crate::{colors::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn move_row<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("　　　　　　　　EvtAct.move_row");

        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let str = prom.cont_1.buf.iter().collect::<String>();
                    if str.is_empty() {
                        mbar.set_err(&LANG.not_entered_row_number_to_move);
                        mbar.draw_only(out);
                        return EvtActType::Hold;
                    }
                    let row_num: usize = str.parse().unwrap();
                    if row_num > editor.buf.len_lines() || row_num == 0 {
                        mbar.set_err(&LANG.number_within_current_number_of_rows);
                        mbar.draw_only(out);
                        return EvtActType::Hold;
                    }

                    editor.cur.y = row_num - 1;
                    editor.cur.x = editor.rnw;
                    editor.cur.disp_x = editor.rnw + 1;

                    prom.clear();
                    mbar.clear();
                    Terminal::set_disp_size(editor, mbar, prom, help, sbar);

                    editor.scroll_move_row();
                    editor.scroll_horizontal();
                    Terminal::init_draw(out, editor, mbar, prom, help, sbar);
                    return EvtActType::Hold;
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
        let mut cont = PromptCont::new();
        cont.set_move_row();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_move_row(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), LANG.set_move_row);
        self.key_desc = format!(
            "{}{}:{}Enter  {}{}:{}Ctrl + c{}",
            Colors::get_default_fg(),
            &LANG.move_to_specified_row,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
            &LANG.close,
            Colors::get_msg_highlight_fg(),
            Colors::get_default_fg(),
        );
    }
}
