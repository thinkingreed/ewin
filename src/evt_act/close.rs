use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;
use termion::color;

impl EvtAct {
    pub fn close<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prompt: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.curt_evt {
            Key(KeyEvent { code: Char(c), .. }) => {
                if c == 'y' {
                    // save成否判定
                    if editor.save(mbar, prompt, sbar) {
                        return EvtActType::Exit;
                    } else {
                        terminal.draw(out, editor, mbar, prompt, sbar).unwrap();
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
        self.cont = cont;
    }
}

impl PromptCont {
    pub fn set_save_confirm(&mut self) {
        self.guide = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.save_confirmation_to_close.clone(), "\n");
        self.key_desc = format!(
            "{}{}:{}Y  {}{}:{}N  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.yes.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.no.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.cancel.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}
