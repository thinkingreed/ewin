use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Editor, EvtProcess, Log, MsgBar, Process, Prompt, PromptBufPosi, PromptCont, StatusBar, Terminal};
use crate::util::*;
use crossterm::event::KeyCode;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::cmp::{max, min};
use std::io::Write;
use termion::color;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Prompt {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("★　Prompt draw");
        if self.cont.guide.len() > 0 {
            let cont_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.cont.guide.clone());
            str_vec.push(cont_desc);

            let key_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 1) as u16), clear::CurrentLine, self.cont.key_desc.clone());
            str_vec.push(key_desc);

            if self.is_save_new_file || self.is_search {
                let buf = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont.buf.iter().collect::<String>());
                str_vec.push(buf);
            }

            if self.is_replace {
                let buf_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont.buf_desc.clone());
                str_vec.push(buf_desc);
                let buf = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 3) as u16), clear::CurrentLine, self.cont.buf.iter().collect::<String>());
                str_vec.push(buf);
                let buf_sub_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 4) as u16), clear::CurrentLine, self.cont_sub.buf_desc.clone());
                str_vec.push(buf_sub_desc);
                let buf_sub = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 5) as u16), clear::CurrentLine, self.cont_sub.buf.iter().collect::<String>());
                str_vec.push(buf_sub);
            }
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("★　Prompt draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        self.draw_cur(&mut v);
        write!(out, "{}", v.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn draw_cur(&mut self, str_vec: &mut Vec<String>) {
        if self.is_replace {
            if self.buf_posi == PromptBufPosi::Main {
                Log::ep("prom.cont.cur.disp_x", self.cont.cur.disp_x);
                str_vec.push(cursor::Goto(self.cont.cur.disp_x as u16, (self.disp_row_posi + 3) as u16).to_string());
            } else {
                Log::ep("prom.cont_sub.cur.disp_x", self.cont_sub.cur.disp_x);
                str_vec.push(cursor::Goto(self.cont_sub.cur.disp_x as u16, (self.disp_row_posi + 5) as u16).to_string());
            }
        } else {
            str_vec.push(cursor::Goto(self.cont.cur.disp_x as u16, (self.disp_row_posi + self.disp_row_num - 1) as u16).to_string());
        }
    }
    pub fn check_prom<T: Write>(&mut self, out: &mut T, terminal: &mut Terminal, editor: &mut Editor, mbar: &mut MsgBar, sbar: &mut StatusBar) -> EvtProcess {
        if self.is_save_new_file == true || self.is_search == true || self.is_close_confirm == true || self.is_replace == true {
            match editor.curt_evt {
                Key(KeyEvent { code, modifiers: KeyModifiers::CONTROL }) => match code {
                    Char('c') => {
                        self.clear();
                        mbar.clear();
                        terminal.draw(out, editor, mbar, self, sbar).unwrap();
                        return EvtProcess::Next;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if self.is_save_new_file == true || self.is_search == true {
            match editor.curt_evt {
                Key(KeyEvent { code, .. }) => match code {
                    Left | Right | Delete | Backspace => {
                        self.cont.edit(code);
                        self.draw_only(out);
                        return EvtProcess::Hold;
                    }
                    Char(c) => {
                        self.cont.insert_char(c);
                        self.draw_only(out);
                        return EvtProcess::Hold;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if self.is_save_new_file == true {
            return Process::save_new_filenm(out, terminal, editor, mbar, self, sbar);
        } else if self.is_close_confirm == true {
            return Process::close(out, terminal, editor, mbar, self, sbar);
        } else if self.is_search == true {
            return Process::search(editor, mbar, self);
        } else if self.is_replace == true {
            return Process::replace(out, terminal, editor, mbar, self, sbar);
        } else {
            Log::ep_s("EvtProcess::NextEvtProcess");
            return EvtProcess::Next;
        }
    }
    pub fn cursor_down(&mut self) {
        Log::ep_s("◆　cursor_down");

        if self.buf_posi == PromptBufPosi::Main {
            self.buf_posi = PromptBufPosi::Sub;
            self.cont_sub.cur.updown_x = self.cont.cur.disp_x;
            let (cur_x, width) = get_until_updown_x(0, &self.cont_sub.buf, self.cont_sub.cur.updown_x);
            self.cont_sub.cur.x = cur_x;
            self.cont_sub.cur.disp_x = width;
        }
    }
    pub fn cursor_up(&mut self) {
        Log::ep_s("cursor_up");

        if self.buf_posi == PromptBufPosi::Sub {
            self.buf_posi = PromptBufPosi::Main;
            self.cont.cur.updown_x = self.cont_sub.cur.disp_x;
            let (cur_x, width) = get_until_updown_x(0, &self.cont.buf, self.cont.cur.updown_x);
            self.cont.cur.x = cur_x;
            self.cont.cur.disp_x = width;
        }
    }
}

impl PromptCont {
    pub fn new(lang_cfg: LangCfg) -> Self {
        PromptCont { lang: lang_cfg, ..PromptCont::default() }
    }

    pub fn insert_char(&mut self, c: char) {
        self.buf.insert(self.cur.x, c);
        self.cur.disp_x += c.width().unwrap_or(0);
        self.cur.x += 1;
    }

    pub fn cursor_left(&mut self) {
        if self.cur.x != 0 {
            self.cur.x = max(self.cur.x - 1, 0);
            self.cur.disp_x -= get_cur_x_width(&self.buf, self.cur.x);
        }
    }
    pub fn cursor_right(&mut self) {
        if self.cur.x < self.buf.len() {
            self.cur.disp_x += get_cur_x_width(&self.buf, self.cur.x);
            self.cur.x = min(self.cur.x + 1, self.buf.len());
        }
    }
    pub fn delete(&mut self) {
        if self.cur.x < self.buf.len() {
            self.buf.remove(self.cur.x);
        }
    }
    pub fn backspace(&mut self) {
        if self.cur.x > 0 {
            self.cur.x -= 1;
            self.cur.disp_x -= self.buf[self.cur.x].width().unwrap_or(0);
            self.buf.remove(self.cur.x);
        }
    }

    pub fn edit(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Delete => self.delete(),
            KeyCode::Backspace => self.backspace(),
            _ => {}
        }
    }
}

impl MsgBar {
    pub fn set_err_str(&mut self, msg: String) {
        let msg = format!("{}{}", &color::Fg(color::White).to_string(), msg,);
        let msg_str = format!("{msg:^width$}", msg = msg, width = self.disp_col_num);
        self.msg_disp = format!("{}{}{}", &color::Bg(color::Red).to_string(), msg_str, &color::Bg(color::Black).to_string(),);
    }
}
