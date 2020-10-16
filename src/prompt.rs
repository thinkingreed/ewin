use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Log, Prompt, PromptCont, PromptContType};
use crossterm::event::KeyCode;
use std::io::Write;

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
            if self.cur_posi == PromptContType::Main {
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
            self.cur.x -= 1;
            self.cur.disp_x -= self.buf[self.cur.x].width().unwrap_or(0);
        }
    }
    pub fn cursor_right(&mut self) {
        if self.cur.x < self.buf.len() {
            self.cur.x += 1;
            if self.cur.x == self.buf.len() {
                self.cur.disp_x += 1;
            } else {
                self.cur.disp_x += self.buf[self.cur.x].width().unwrap_or(0);
            }
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
