use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptCont, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::color;

impl Process {
    pub fn search<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
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
                    /*
                    if prom.cont.buf.len() == 0 {
                        mbar.set_msg_set_file_name();
                    } else {
                        // TODO 存在するファイル名の対応
                        sbar.filenm = prom.cont.buf.iter().collect::<String>();
                        editor.save(mbar, prom, sbar);
                    }
                    */
                    terminal.draw(out, editor, mbar, prom, sbar).unwrap();

                    return EvtProcess::Next;
                }
                _ => return EvtProcess::Hold,
            },
            _ => return EvtProcess::Hold,
        }
    }
}

impl Prompt {
    pub fn search(&mut self) {
        self.disp_row_num = 3;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_search();
        self.cont = cont;
    }
}

impl PromptCont {
    pub fn set_search(&mut self) {
        self.desc = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_search_str.clone(), "\n");
        self.input_desc = format!(
            "{}{}:{}Enter  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.fixed.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.close.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}
