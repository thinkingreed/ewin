use crate::{_cfg::keys::KeyCmd, model::EvtProc, prompt::cont::promptcont::*, util::*};
use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;

impl PromptCont {
    pub fn insert_char(&mut self, c: char) {
        self.buf.insert(self.cur.x, c);
        self.cur.disp_x += c.width().unwrap_or(0);
        self.cur.x += 1;
    }

    pub fn cur_left(&mut self) {
        if self.cur.x != 0 {
            self.cur.x = max(self.cur.x - 1, 0);
            self.cur.disp_x -= get_char_width_not_tab(self.buf[self.cur.x]);
        }
    }
    pub fn cur_right(&mut self) {
        if self.cur.x < self.buf.len() {
            self.cur.disp_x += get_char_width_not_tab(self.buf[self.cur.x]);
            self.cur.x = min(self.cur.x + 1, self.buf.len());
        }
    }
    pub fn delete(&mut self, ep: &mut EvtProc) {
        if self.cur.x < self.buf.len() {
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
    }

    pub fn backspace(&mut self, ep: &mut EvtProc) {
        if self.cur.x > 0 {
            self.cur.x -= 1;
            self.cur.disp_x -= get_char_width_not_tab(self.buf[self.cur.x]);
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
    }
    pub fn end(&mut self) {
        self.cur.x = self.buf.len();
        let (_, width) = get_row_width(&self.buf[..], 0, false);
        self.cur.disp_x = width;
    }

    pub fn home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
    }

    pub fn operation(&mut self) {
        match self.keycmd {
            KeyCmd::CursorLeft => self.cur_left(),
            KeyCmd::CursorRight => self.cur_right(),
            KeyCmd::CursorRowHome => self.home(),
            KeyCmd::CursorRowEnd => self.end(),
            _ => {}
        }
    }
}
