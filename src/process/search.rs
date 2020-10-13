use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptCont, Search, StatusBar, Terminal};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::io::Write;
use termion::color;

impl Process {
    pub fn search<T: Write>(out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, prom: &mut Prompt, sbar: &mut StatusBar) -> EvtProcess {
        Log::ep_s("Process.search");

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
            "{}{}:{}F3  {}{}:{}Ctrl + c{}",
            &color::Fg(color::White).to_string(),
            self.lang.search_start.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
            self.lang.close.clone(),
            &color::Fg(color::LightGreen).to_string(),
            &color::Fg(color::White).to_string(),
        );
    }
}

impl MsgBar {
    pub fn set_not_entered_serach_str(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.not_entered_search_str.clone(),);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }

    /*
    pub fn set_no_search_str_bottom(&mut self) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), self.lang.no_search_str_bottom.clone(),);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
    */
}
