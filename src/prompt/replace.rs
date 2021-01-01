use crate::{global::*, model::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn replace<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        Log::ep_s("Process.replace");

        match editor.evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let search_str = prom.cont_1.buf.iter().collect::<String>();

                    if search_str.len() == 0 {
                        mbar.set_err(&LANG.lock().unwrap().not_entered_search_str);
                    } else if prom.cont_2.buf.len() == 0 {
                        mbar.set_err(&LANG.lock().unwrap().not_entered_replace_str);
                    } else if editor.get_search_ranges(&search_str.clone()).len() == 0 {
                        mbar.set_err(&LANG.lock().unwrap().cannot_find_char_search_for);
                    } else {
                        editor.replace(prom);
                        mbar.clear();
                        prom.clear();
                        prom.is_change = true;
                    }
                    term.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtActType::Hold;
                }

                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn replace(&mut self) {
        self.disp_row_num = 6;
        let mut cont_1 = PromptCont::new(self.lang.clone());
        let mut cont_2 = PromptCont::new(self.lang.clone());
        cont_1.set_replace(PromptBufPosi::First);
        cont_2.set_replace(PromptBufPosi::Second);
        self.cont_1 = cont_1;
        self.cont_2 = cont_2;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_fg(), self.lang.set_replace.clone());
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                Colors::get_default_fg(),
                self.lang.all_replace.clone(),
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                self.lang.move_input_field.clone(),
                Colors::get_msg_fg(),
                Colors::get_default_fg(),
                self.lang.close.clone(),
                Colors::get_msg_fg(),
            );
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), self.lang.search_str.clone(),);
        } else {
            self.buf_desc = format!("{}{}", Colors::get_default_fg(), self.lang.replace_char.clone(),);
        }
    }
}
