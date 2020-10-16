use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptCont, PromptContType, Search};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent};
use std::io::Write;
use termion::color;

impl Process {
    pub fn replace<T: Write>(out: &mut T, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt) -> EvtProcess {
        Log::ep_s("Process.replace");

        match editor.curt_evt {
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
                F(3) => {
                    Log::ep_s("search.F3");

                    if prom.cont.buf.len() == 0 {
                        mbar.set_not_entered_serach_str();
                    } else {
                        editor.search.str = prom.cont.buf.iter().collect::<String>();

                        Log::ep("search_str", editor.search.str.clone());
                        mbar.clear();
                        prom.clear();
                        editor.search.index = Search::INDEX_UNDEFINED;
                    }
                    return EvtProcess::Next;
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
        cont.set_replace(PromptContType::Main);
        self.cont = cont;
        let mut cont_sub = PromptCont::new(self.lang.clone());
        cont_sub.set_replace(PromptContType::Sub);
        self.cont_sub = cont_sub;
    }
}

impl PromptCont {
    pub fn set_replace(&mut self, cont_type: PromptContType) {
        if cont_type == PromptContType::Main {
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

impl MsgBar {
    pub fn set_replace(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.not_entered_search_str.clone(),);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
}
