use crate::{colors::*, global::*, help::*, log::*, model::*, msgbar::*, prompt::prompt::*, prompt::promptcont::promptcont::*, statusbar::*, terminal::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn replace<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, help: &mut Help, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(&LANG.not_entered_search_str);
                    } else if prom.cont_2.buf.len() == 0 {
                        mbar.set_err(&LANG.not_entered_replace_str);
                    } else if editor.get_search_ranges(&search_str.clone(), 0, editor.buf.len_chars()).len() == 0 {
                        mbar.set_err(&LANG.cannot_find_char_search_for);
                    } else {
                        let search_set = editor.buf.search(&search_str.clone(), 0, editor.buf.len_chars());
                        if search_set.len() == 0 {
                            mbar.set_err(&LANG.cannot_find_char_search_for);
                            return EvtActType::DrawOnly;
                        }
                        editor.replace(prom, search_set);
                        mbar.clear();
                        prom.clear();
                        prom.is_change = true;
                    }
                    editor.d_range.draw_type = DrawType::All;
                    Terminal::draw(out, editor, mbar, prom, help, sbar).unwrap();
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(&mut self) {
        self.is_replace = true;
        self.disp_row_num = 7;
        let mut cont_1 = PromptCont::new();
        let mut cont_2 = PromptCont::new();
        cont_1.set_replace(PromptBufPosi::First);
        cont_2.set_replace(PromptBufPosi::Second);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_replace);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                Colors::get_default_fg(),
                &LANG.all_replace,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.move_input_field,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
            );
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.search_str,);
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), &LANG.replace_char,);
        }
        self.set_opt_case_sens();
        self.set_opt_regex();
    }
}
