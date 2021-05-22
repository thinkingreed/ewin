use crate::{prompt::promptcont::promptcont::*, util::*};
use crossterm::event::KeyCode;
use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;

impl PromptCont {
    pub fn insert_char(&mut self, c: char, is_move_line: bool, rnw: usize) {
        if is_move_line && !c.is_ascii_digit() {
            return;
        }

        let str: String = self.buf.iter().collect::<String>();
        if is_move_line && str.chars().count() == rnw {
            return;
        }
        if self.sel.is_selected() {
            self.del_sel_range();
            self.sel.clear();
        }
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
                self.cur.disp_x -= get_char_width_not_tab(self.buf[self.cur.x]);
                self.buf.remove(self.cur.x);
            }
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

    pub fn operation(&mut self, key: KeyCode) {
        match key {
            KeyCode::Left => self.cur_left(),
            KeyCode::Right => self.cur_right(),
            KeyCode::Delete => self.delete(),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Home => self.home(),
            KeyCode::End => self.end(),
            _ => {}
        }
    }
}
