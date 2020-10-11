use crate::_cfg::lang::cfg::LangCfg;
use crate::model::{Log, Prompt, PromptCont};
use std::io::Write;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Prompt {
    pub fn new(lang_cfg: LangCfg) -> Self {
        Prompt { lang: lang_cfg, ..Prompt::default() }
    }

    pub fn clear(&mut self) {
        Log::ep_s("★　Prompt clear");

        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.is_save_confirm = false;
        self.is_save_new_file = false;
        self.is_search_prom = false;
        //self.is_change = false;
        self.cont = PromptCont::default();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("★　Prompt draw");
        if self.cont.desc.len() > 0 {
            let cont_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi) as u16), clear::CurrentLine, self.cont.desc.clone());
            str_vec.push(cont_desc);

            let input_desc = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 1) as u16), clear::CurrentLine, self.cont.input_desc.clone());
            str_vec.push(input_desc);

            let input = format!("{}{}{}", cursor::Goto(1, (self.disp_row_posi + 2) as u16), clear::CurrentLine, self.cont.buf.iter().collect::<String>());
            if self.is_save_new_file || self.is_search_prom || self.cont.buf.len() > 0 {
                str_vec.push(input);
            }
        }
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("★　Prompt draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        write!(out, "{}{}", v.concat(), cursor::Goto(self.cont.cur.disp_x as u16, (self.disp_row_posi + self.disp_row_num - 1) as u16)).unwrap();
        out.flush().unwrap();
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
}
