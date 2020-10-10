use crate::model::{Editor, EvtProcess, MsgBar, Process, Prompt, PromptCont, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::color;

impl Process {
    pub fn save_new_filenm<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        match editor.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                Char('c') => {
                    prom.clear();
                    mbar.clear();
                    terminal.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtProcess::Next;
                }
                _ => return EvtProcess::Hold,
            },
            Key(KeyEvent { code: Char(c), .. }) => {
                prom.cont.insert_char(c);
                prom.draw_only(out);
                return EvtProcess::Hold;
            }
            Key(KeyEvent { code, .. }) => match code {
                Left => {
                    prom.cont.cursor_left();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Right => {
                    prom.cont.cursor_right();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Delete => {
                    prom.cont.delete();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Backspace => {
                    prom.cont.backspace();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Enter => {
                    if prom.cont.buf.len() == 0 {
                        mbar.set_not_entered_filenm();
                    } else {
                        // TODO 存在するファイル名の対応
                        sbar.filenm = prom.cont.buf.iter().collect::<String>();
                        editor.save(mbar, prom, sbar);
                    }
                    terminal.draw(out, editor, mbar, prom, sbar).unwrap();
                    return EvtProcess::Hold;
                }
                _ => return EvtProcess::Hold,
            },
            _ => return EvtProcess::Hold,
        }
    }
}

impl Prompt {
    pub fn save_new_file(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_new_file_name();
        self.cont = cont;
    }
}

impl PromptCont {
    pub fn set_new_file_name(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_new_filenm.clone(), "\n");
        self.input_desc = format!(
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

impl MsgBar {
    pub fn set_not_entered_filenm(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.not_entered_filenm.clone());
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
}
