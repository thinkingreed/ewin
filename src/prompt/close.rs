use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;

impl EvtAct {
    pub fn close<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prompt: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if editor.save(mbar, prompt, sbar) {
                        return EvtActType::Exit;
                    } else {
                        term.draw(out, editor, mbar, prompt, sbar).unwrap();
                        return EvtActType::Hold;
                    }
                } else if c == 'n' {
                    return EvtActType::Exit;
                } else {
                    return EvtActType::Hold;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    pub fn save_confirm_str(&mut self) {
        self.disp_row_num = 2;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_save_confirm();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}", Colors::get_msg_fg(), self.lang.save_confirmation_to_close.clone());
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}Ctrl + c{}",
            Colors::get_default_fg(),
            self.lang.yes.clone(),
            Colors::get_msg_fg(),
            Colors::get_default_fg(),
            self.lang.no.clone(),
            Colors::get_msg_fg(),
            Colors::get_default_fg(),
            self.lang.cancel.clone(),
            Colors::get_msg_fg(),
            Colors::get_default_fg(),
        );
    }
}
