use crate::model::{Editor, Log, Search, SearchRange};
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
        let (cursorx, line_width) = self.get_row_width(self.cur.y, 0, self.buf[self.cur.y].len());

        // Left,Rightの場合は設定しない
        if self.curt_evt == Key(Left.into()) || self.curt_evt == Key(Right.into()) {
        } else if line_width < self.x_offset_disp {
            Log::ep_s("c_d x_offset 切り替わる");
            self.cur.disp_x = line_width + self.lnw + 1;
            self.cur.x = cursorx + self.lnw;
        } else {
            Log::ep_s("c_d x_offset 切り替わっていない");
            let (cursorx, disp_x) = self.get_until_updown_x();
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
            let (_, width) = self.get_row_width(self.cur.y - 1, 0, rowlen);
            self.cur.disp_x = width + self.lnw + 1;

            self.cursor_up();
        } else {
            self.cur.x = max(self.cur.x - 1, self.lnw);
            self.cur.disp_x -= self.get_cur_x_width(self.cur.y);
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
            self.cur.disp_x += self.get_cur_x_width(self.cur.y);
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
        self.lnw = self.buf.len().to_string().len();
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn insert_char(&mut self, c: char) {
        //  if !c.is_control() {
        self.buf[self.cur.y].insert(self.cur.x - self.lnw, c);
        self.cursor_right();
        //}
    }

    pub fn back_space(&mut self) {
        Log::ep_s("★  back_space");
        if self.sel.is_unselected() {
            // 0,0の位置の場合
            if self.cur.y == 0 && self.cur.x == self.lnw {
                return;
            }
            if self.cur.x == self.lnw {
                let row_len_org = self.buf.len().to_string().len();

                // 行の先頭
                let line = self.buf.remove(self.cur.y);
                self.cur.y -= 1;
                self.cur.x = self.buf[self.cur.y].len() + self.lnw;
                let (_, width) = self.get_row_width(self.cur.y, 0, self.buf[self.cur.y].len());
                self.cur.disp_x = self.lnw + width + 1;
                self.buf[self.cur.y].extend(line.into_iter());

                let row_len_curt = self.buf.len().to_string().len();
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
        } else {
            self.del_sel_range();
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn delete(&mut self) {
        // 最終行の終端
        if self.cur.y == self.buf.len() - 1 && self.cur.x == self.buf[self.cur.y].len() + self.lnw {
            return;
        }
        if self.sel.is_unselected() {
            if self.cur.x == self.buf[self.cur.y].len() + self.lnw {
                // 行末
                let line = self.buf.remove(self.cur.y + 1);
                self.buf[self.cur.y].extend(line.into_iter());
            } else {
                self.buf[self.cur.y].remove(self.cur.x - self.lnw);
            }
        } else {
            self.del_sel_range();
        }
    }

    pub fn home(&mut self) {
        self.cur.x = self.lnw;
        self.cur.disp_x = self.get_cur_x_width(self.cur.y) + self.lnw;
        self.scroll_horizontal();
    }
    pub fn end(&mut self) {
        self.cur.x = self.buf[self.cur.y].len() + self.lnw;
        let (_, disp_x) = self.get_row_width(self.cur.y, 0, self.cur.x + 1);
        self.cur.disp_x = disp_x;
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

    pub fn search(&mut self) {
        Log::ep_s("★　search_str");

        if self.search.str.len() > 0 {
            // 初回検索
            if self.search.index == Search::INDEX_UNDEFINED {
                self.search.search_ranges.clear();
                Log::ep("search_str", self.search.str.clone());
                let len = self.search.str.chars().count();
                for (i, chars) in self.buf.iter().enumerate() {
                    let row_str = chars.iter().collect::<String>();
                    let v: Vec<(usize, &str)> = row_str.match_indices(&self.search.str).collect();
                    if v.len() == 0 {
                        continue;
                    }
                    for (_, indexs) in v.iter().enumerate() {
                        self.search.search_ranges.push(SearchRange { y: i, sx: indexs.0, ex: indexs.0 + len - 1 });
                    }
                }
                if self.search.search_ranges.len() > 0 {
                    self.search.index = 0;
                }

                eprintln!("self.search.search_ranges {:?}", self.search.search_ranges);
            } else {
                self.search.index += 1;
            }

            if self.search.index != Search::INDEX_UNDEFINED {
                let range = self.search.search_ranges[self.search.index];
                eprintln!("search.search_ranges[index] {:?}", range);
                self.cur.y = range.y;
                self.cur.x = range.sx + self.lnw;
                let (_, width) = self.get_row_width(self.cur.y, 0, range.sx + 1);
                self.cur.disp_x = width + self.lnw;
                self.scroll();
                self.scroll_horizontal();
            }
        }
    }
}
