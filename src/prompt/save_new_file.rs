use crate::model::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;
use termion::color;

impl EvtAct {
    pub fn save_new_filenm<T: Write>(out: &mut T, term: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtActType {
        match editor.curt_evt {
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    if prom.cont_1.buf.len() == 0 {
                        mbar.set_err(mbar.lang.not_entered_filenm.clone());
                    } else {
                        // TODO 存在するファイル名の対応
                        sbar.filenm = prom.cont_1.buf.iter().collect::<String>();
                        editor.save(mbar, prom, sbar);
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
    pub fn save_new_file(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_new_file_name();
        self.cont_1 = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.guide = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_new_filenm.clone(), "\n");
        self.key_desc = format!(
            "{}{}:{}Enter  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.fixed.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.cancel.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}
