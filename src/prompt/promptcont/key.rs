use crate::model::*;
use crate::util::*;
use crossterm::event::KeyCode;
use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;

impl PromptCont {
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
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        } else {
            if self.cur.x < self.buf.len() {
                self.buf.remove(self.cur.x);
            }
        }
    }

    pub fn backspace(&mut self) {
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        } else {
            if self.cur.x > 0 {
                self.cur.x -= 1;
                self.cur.disp_x -= self.buf[self.cur.x].width().unwrap_or(0);
                self.buf.remove(self.cur.x);
            }
        }
    }
    pub fn end(&mut self) {
        self.cur.x = self.buf.len();
        let (_, width) = get_row_width(&self.buf, 0, self.cur.x);
        self.cur.disp_x = width + 1;
    }

    pub fn home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 1;
    }

    pub fn edit(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Delete => self.delete(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            _ => {}
        }
    }
}
