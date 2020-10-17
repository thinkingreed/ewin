use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptBufPosi, PromptCont, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;
use termion::color;

impl Process {
    pub fn replace<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        Log::ep_s("Process.replace");

        match editor.curt_evt {
            Key(KeyEvent { code, .. }) => match code {
                Char(c) => {
                    match prom.buf_posi {
                        PromptBufPosi::Main => prom.cont.insert_char(c),
                        PromptBufPosi::Sub => prom.cont_sub.insert_char(c),
                    }
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Down => {
                    prom.cursor_down();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Up => {
                    prom.cursor_up();
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Left => {
                    match prom.buf_posi {
                        PromptBufPosi::Main => prom.cont.cursor_left(),
                        PromptBufPosi::Sub => prom.cont_sub.cursor_left(),
                    }
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Right => {
                    match prom.buf_posi {
                        PromptBufPosi::Main => prom.cont.cursor_right(),
                        PromptBufPosi::Sub => prom.cont_sub.cursor_right(),
                    }
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Delete => {
                    match prom.buf_posi {
                        PromptBufPosi::Main => prom.cont.delete(),
                        PromptBufPosi::Sub => prom.cont_sub.delete(),
                    }
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Backspace => {
                    match prom.buf_posi {
                        PromptBufPosi::Main => prom.cont.backspace(),
                        PromptBufPosi::Sub => prom.cont_sub.backspace(),
                    }
                    prom.draw_only(out);
                    return EvtProcess::Hold;
                }
                Enter => {
                    if prom.cont.buf.len() == 0 {
                        mbar.set_err_str(mbar.lang.not_entered_search_str.clone());
                    } else if prom.cont_sub.buf.len() == 0 {
                        mbar.set_err_str(mbar.lang.not_entered_replace_str.clone());
                    } else {
                        editor.replace(prom);
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
    pub fn replace(&mut self) {
        self.disp_row_num = 6;
        let mut cont = PromptCont::new(self.lang.clone());
        cont.set_replace(PromptBufPosi::Main);
        self.cont = cont;
        let mut cont_sub = PromptCont::new(self.lang.clone());
        cont_sub.set_replace(PromptBufPosi::Sub);
        self.cont_sub = cont_sub;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self, cont_type: PromptBufPosi) {
        if cont_type == PromptBufPosi::Main {
            self.guide = format!("{}{}{}", &color::Fg(color::LightGreen).to_string(), self.lang.set_replace.clone(), "\n");
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}↓↑  {}{}:{}Ctrl + c",
                &color::Fg(color::White).to_string(),
                self.lang.all_replace.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.move_input_field.clone(),
                &color::Fg(color::LightGreen).to_string(),
                &color::Fg(color::White).to_string(),
                self.lang.close.clone(),
                &color::Fg(color::LightGreen).to_string(),
            );
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.search_char.clone(),);
        } else {
            self.buf_desc = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.replace_char.clone(),);
        }
    }
}
