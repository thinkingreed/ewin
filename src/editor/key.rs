use crate::model::*;
use crate::util::*;
use crossterm::event::Event::Key;
use crossterm::event::KeyCode::{Left, Right};
use std::cmp::{max, min};

impl Editor {
    pub fn cursor_up(&mut self) {
        Log::ep_s("★ c_u start");
        if self.cur.y > 0 {
            self.cur.y -= 1;

            self.cursor_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cursor_down(&mut self) {
        Log::ep_s("★ c_d start");
        if self.cur.y + 1 < self.buf.len() {
            self.cur.y += 1;

            self.cursor_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cursor_updown_com(&mut self) {
        if self.cur.updown_x == 0 {
            self.cur.updown_x = self.cur.disp_x;
        }
        // Left,Rightの場合は設定しない
        if self.curt_evt == Key(Left.into()) || self.curt_evt == Key(Right.into()) {
        } else {
            let (cursorx, disp_x) = get_until_updown_x(self.lnw, &self.buf[self.cur.y], self.cur.updown_x);
            self.cur.disp_x = disp_x;
            self.cur.x = cursorx;
        }
    }

    pub fn cursor_left(&mut self) {
        Log::ep_s("★  c_l start");
        // 0, 0の位置の場合
        if self.cur.y == 0 && self.cur.x == self.lnw {
            return;
        // 行頭の場合
        } else if self.cur.x == self.lnw {
            let rowlen = self.buf[self.cur.y - 1].len();
            self.cur.x = rowlen + self.lnw;
            let (_, width) = get_row_width(&self.buf[self.cur.y - 1], 0, rowlen);
            self.cur.disp_x = width + self.lnw + 1;

            self.cursor_up();
        } else {
            self.cur.x = max(self.cur.x - 1, self.lnw);
            self.cur.disp_x -= get_cur_x_width(&self.buf[self.cur.y], self.cur.x - self.lnw);
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cursor_right(&mut self) {
        Log::ep_s("★  c_r start");
        // 行末
        if self.cur.x == self.buf[self.cur.y].len() + self.lnw {
            // 最終行の行末
            if self.cur.y == self.buf.len() - 1 {
                return;
            // その他の行末
            } else {
                self.cur.updown_x = self.lnw;
                self.cur.disp_x = self.lnw + 1;
                self.cur.x = self.lnw;
                self.cursor_down();
            }
        } else {
            self.cur.disp_x += get_cur_x_width(&self.buf[self.cur.y], self.cur.x - self.lnw);
            self.cur.x = min(self.cur.x + 1, self.buf[self.cur.y].len() + self.lnw);
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn enter(&mut self) {
        Log::ep_s("★  enter");
        let rest: Vec<char> = self.buf[self.cur.y].drain(self.cur.x - self.lnw..).collect();
        self.buf.insert(self.cur.y + 1, rest);
        self.cur.y += 1;
        self.lnw = self.buf.len().to_string().len() + 1;
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn insert_char(&mut self, c: char) {
        //  if !c.is_control() {
        self.buf[self.cur.y].insert(self.cur.x - self.lnw, c);
        self.cursor_right();
        self.e_ranges.push(EditRnage { y: self.cur.y, e_type: EType::Mod });
    }

    pub fn back_space(&mut self) {
        Log::ep_s("★  back_space");
        if self.sel.is_selected() {
            self.del_sel_range();
        } else {
            // 0,0の位置の場合
            if self.cur.y == 0 && self.cur.x == self.lnw {
                return;
            }
            if self.cur.x == self.lnw {
                let row_len_org = self.buf.len().to_string().len() + 1;

                // 行の先頭
                let line = self.buf.remove(self.cur.y);
                self.cur.y -= 1;
                self.cur.x = self.buf[self.cur.y].len() + self.lnw;
                let (_, width) = get_row_width(&self.buf[self.cur.y], 0, self.buf[self.cur.y].len());
                self.cur.disp_x = self.lnw + width + 1;
                self.buf[self.cur.y].extend(line.into_iter());

                let row_len_curt = self.buf.len().to_string().len() + 1;
                // 行番号の桁数が減った場合
                if row_len_org != row_len_curt {
                    self.lnw -= 1;
                    self.cur.x -= 1;
                    self.cur.disp_x -= 1;
                }
            } else {
                self.cursor_left();
                self.buf[self.cur.y].remove(self.cur.x - self.lnw);
            }
            self.e_ranges.push(EditRnage { y: self.cur.y, e_type: EType::Mod });
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn delete(&mut self) {
        // 最終行の終端
        if self.cur.y == self.buf.len() - 1 && self.cur.x == self.buf[self.cur.y].len() + self.lnw {
            return;
        }
        if self.sel.is_selected() {
            self.del_sel_range();
        } else {
            if self.cur.x == self.buf[self.cur.y].len() + self.lnw {
                // 行末
                let line = self.buf.remove(self.cur.y + 1);
                self.buf[self.cur.y].extend(line.into_iter());
            } else {
                self.buf[self.cur.y].remove(self.cur.x - self.lnw);
            }
        }
    }

    pub fn home(&mut self) {
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
        self.scroll_horizontal();
    }
    pub fn end(&mut self) {
        self.cur.x = self.buf[self.cur.y].len() + self.lnw;
        let (_, disp_x) = get_row_width(&self.buf[self.cur.y], 0, self.buf[self.cur.y].len());
        self.cur.disp_x = disp_x + self.lnw + 1;

        Log::ep("end.cur.disp_x ", self.cur.disp_x);
        Log::ep("end.cur.x ", self.cur.x);

        self.scroll_horizontal();
    }
    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.disp_row_num, self.buf.len() - 1);
        self.cur.x = self.lnw;
        self.scroll();
    }
    pub fn page_up(&mut self) {
        if self.cur.y > self.disp_row_num {
            self.cur.y = self.cur.y - self.disp_row_num;
        } else {
            self.cur.y = 0
        }
        self.cur.x = self.lnw;
        self.scroll();
    }
}
