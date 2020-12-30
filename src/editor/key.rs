use crate::{def::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*};
use std::cmp::{max, min};

impl Editor {
    pub fn cur_up(&mut self) {
        Log::ep_s("　　　　　　　 c_u start");
        if self.cur.y > 0 {
            self.cur.y -= 1;

            self.cur_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cur_down(&mut self) {
        Log::ep_s("　　　　　　　 c_d start");
        if self.cur.y + 1 < self.buf.len() {
            self.cur.y += 1;

            self.cur_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_updown_com(&mut self) {
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
        // Left,Rightの場合は設定しない
        if self.evt == Key(Left.into()) || self.evt == Key(Right.into()) {
        } else {
            let (cur_x, disp_x) = get_until_updown_x(&self.buf[self.cur.y], self.updown_x - self.rnw);
            self.cur.disp_x = disp_x + self.rnw;
            self.cur.x = cur_x + self.rnw;
        }
    }

    pub fn cur_left(&mut self) {
        Log::ep_s("　　　　　　　  c_l start");
        // 0, 0の位置の場合
        if self.cur.y == 0 && self.cur.x == self.rnw {
            return;
        // 行頭の場合
        } else if self.cur.x == self.rnw {
            //   self.cur.x = rowlen + self.rnw - 1;
            let (cur_x, width) = get_row_width(&self.buf[self.cur.y - 1], 0, self.buf[self.cur.y - 1].len(), false);
            self.cur.x = cur_x + self.rnw;
            self.cur.disp_x = width + self.rnw + 1;
            self.d_range = DRnage::new(self.cur.y - 1, self.cur.y, DType::Target);

            self.cur_up();
        } else {
            self.cur.x = max(self.cur.x - 1, self.rnw);
            self.cur.disp_x -= get_cur_x_width(&self.buf[self.cur.y], self.cur.x - self.rnw);
            self.d_range = DRnage::new(self.cur.y, self.cur.y, DType::Target);
        }

        self.scroll();
        self.scroll_horizontal();
    }
    pub fn cur_right(&mut self) {
        Log::ep_s("　　　　　　　  c_r start");

        let mut is_end_of_line = false;
        if self.evt == RIGHT || self.evt == CTRL_V || self.evt == SHIFT_RIGHT {
            //   if self.buf.len() - 1 >= self.cur.x - self.rnw {
            let char = self.buf[self.cur.y][self.cur.x - self.rnw];
            if char == NEW_LINE_MARK {
                is_end_of_line = true;
            }
        //  }
        } else if self.evt == SHIFT_RIGHT && self.cur.x == self.buf[self.cur.y].len() + self.rnw {
            is_end_of_line = true;
        }

        // 行末(行末の空白対応で)
        if is_end_of_line {
            // 最終行の行末
            if self.cur.y == self.buf.len() - 1 {
                return;
            // その他の行末
            } else {
                self.updown_x = self.rnw;
                self.cur.disp_x = self.rnw + 1;
                self.cur.x = self.rnw;
                self.cur_down();
            }
        } else {
            let c = self.buf[self.cur.y][self.cur.x - self.rnw];
            if c == EOF {
                return;
            }
            // EOF
            if self.evt != SHIFT_RIGHT {
                if c == NEW_LINE_MARK {
                    return;
                }
            }
            self.cur.disp_x += get_cur_x_width(&self.buf[self.cur.y], self.cur.x - self.rnw);
            self.cur.x = min(self.cur.x + 1, self.buf[self.cur.y].len() + self.rnw);
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn enter(&mut self) {
        Log::ep_s("　　　　　　　  enter");
        let y_offset_org: usize = self.y_offset;
        let rnw_org = self.rnw;
        let mut evt_proc = EvtProc::new(DoType::Enter, self.cur, self.d_range);

        let rest: Vec<char> = self.buf[self.cur.y].drain(self.cur.x - self.rnw..).collect();
        self.buf[self.cur.y].push(NEW_LINE_MARK);
        self.buf.insert(self.cur.y + 1, rest);
        self.cur.y += 1;
        self.rnw = self.buf.len().to_string().len();
        self.cur.x = self.rnw;
        self.cur.disp_x = self.rnw + 1;

        self.scroll();
        self.scroll_horizontal();

        Log::ep("y_offset_org", y_offset_org);
        Log::ep("y_offset", self.y_offset);
        Log::ep("rnw_org == self.rnw", rnw_org == self.rnw);

        if y_offset_org == self.y_offset && rnw_org == self.rnw && !self.sel.is_selected() {
            self.d_range = DRnage::new(self.cur.y - 1, self.cur.y, DType::After);
        } else {
            self.d_range.d_type = DType::All;
        }
        evt_proc.d_range = self.d_range;
        evt_proc.cur_e = Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x };

        if !self.is_undo {
            self.undo_vec.push(evt_proc);
        }
    }
    pub fn insert_char(&mut self, c: char) {
        self.buf[self.cur.y].insert(self.cur.x - self.rnw, c);

        let mut ep = EvtProc::new(DoType::InsertChar, self.cur, self.d_range);
        self.cur_right();
        ep.cur_e = Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x };
        if self.sel.is_selected() {
            self.d_range.d_type = DType::All;
        } else {
            self.d_range = DRnage::new(self.cur.y, self.cur.y, DType::Target);
        }
        ep.d_range = self.d_range;
        ep.str_vec = vec![c.to_string()];
        self.undo_vec.push(ep);
    }

    pub fn back_space(&mut self) {
        Log::ep_s("　　　　　　　  back_space");
        if self.sel.is_selected() {
            self.set_sel_del_d_range();
            self.save_sel_del_evtproc(DoType::BS);
            self.del_sel_range();
            self.sel.clear();
        } else {
            // For the starting point
            if self.cur.y == 0 && self.cur.x == self.rnw {
                self.d_range.d_type = DType::Not;
                return;
            }

            let mut ep = EvtProc::new(DoType::BS, self.cur, self.d_range);

            // beginning of the line
            if self.cur.x == self.rnw {
                let rnw_org = self.rnw;

                self.d_range = DRnage::new(self.cur.y - 1, self.cur.y - 1, DType::After);

                let line = self.buf.remove(self.cur.y);
                self.cur.y -= 1;

                // del new line code
                let len = self.buf[self.cur.y].len();
                self.buf[self.cur.y].remove(len - 1);
                self.cur.x = self.buf[self.cur.y].len() + self.rnw;
                Log::ep("self.cur.x", self.cur.x);

                let (_, width) = get_row_width(&self.buf[self.cur.y], 0, self.buf[self.cur.y].len(), true);
                self.cur.disp_x = self.rnw + width + 1;
                self.buf[self.cur.y].extend(line.into_iter());
                self.rnw = self.buf.len().to_string().len();

                // When the number of digits in the line number decreases
                if rnw_org != self.rnw {
                    self.cur.x -= 1;
                    self.cur.disp_x -= 1;
                    self.d_range.d_type = DType::All;
                }
            } else {
                self.d_range = DRnage::new(self.cur.y, self.cur.y, DType::Target);

                self.cur_left();
                if let Some(c) = self.buf[self.cur.y].get(self.cur.x - self.rnw) {
                    ep.str_vec = vec![c.to_string()];
                };
                self.buf[self.cur.y].remove(self.cur.x - self.rnw);
            }
            // BS後のcurを設定
            ep.cur_e = Cur { y: self.cur.y, x: self.cur.x, disp_x: self.cur.disp_x };

            if !self.is_undo {
                self.undo_vec.push(ep);
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn delete(&mut self) {
        Log::ep_s("　　　　　　　  delete");

        if self.sel.is_selected() {
            self.set_sel_del_d_range();
            self.save_sel_del_evtproc(DoType::Del);
            self.del_sel_range();
            self.sel.clear();
        } else {
            // 最終行の終端
            if self.cur.y == self.buf.len() - 1 && self.cur.x == self.buf[self.cur.y].len() + self.rnw - 1 {
                self.d_range.d_type = DType::Not;
                return;
            }
            self.d_range = DRnage { sy: self.cur.y, ey: self.cur.y, d_type: DType::Target };

            // 行末
            if self.cur.x == self.buf[self.cur.y].len() + self.rnw - 1 {
                self.d_range.d_type = DType::After;

                let rnw_org = self.rnw;

                self.save_del_char_evtproc(DoType::Del);
                let index = self.buf[self.cur.y].len() - 1;
                self.buf[self.cur.y].remove(index);
                self.del_end_of_line_new_line(self.cur.y + 1);

                self.rnw = self.buf.len().to_string().len();
                // 行番号の桁数が減った場合
                if rnw_org != self.rnw {
                    self.cur.x -= 1;
                    self.cur.disp_x -= 1;
                    self.d_range.d_type = DType::All;
                }
            } else {
                self.save_del_char_evtproc(DoType::Del);
                self.buf[self.cur.y].remove(self.cur.x - self.rnw);
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn home(&mut self) {
        Log::ep_s("　　　　　　　  home");
        self.cur.x = self.rnw;
        self.cur.disp_x = self.rnw + 1;
        self.scroll_horizontal();
    }

    pub fn end(&mut self) {
        let row = &self.buf[self.cur.y];
        let (cur_x, disp_x) = get_row_width(row, 0, row.len(), false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = disp_x + self.rnw + 1;

        self.scroll_horizontal();
    }

    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.disp_row_num, self.buf.len() - 1);
        self.cur_updown_com();
        self.scroll();
    }
    pub fn page_up(&mut self) {
        if self.cur.y > self.disp_row_num {
            self.cur.y = self.cur.y - self.disp_row_num;
        } else {
            self.cur.y = 0
        }
        self.cur_updown_com();
        self.scroll();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_insert_char() {
        let mut e = Editor::default();
        let mut mbar = MsgBar::new();

        e.init_ut_editor();

        // first char
        e.insert_char('A');
        assert_eq!(e.get_buf_str(), format!("{}{}", "A", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));
        e.undo(&mut mbar);
        assert_eq!(e.get_buf_str(), EOF.to_string());

        // second char
        e.insert_char('B');
        assert_eq!(e.get_buf_str(), format!("{}{}", "AB", EOF));
        // println!(" multi char {:?}", e.get_buf_str());
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 2, disp_x: e.rnw + 1 + 2 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));
    }
    #[test]
    fn test_enter() {
        let mut e = Editor::default();
        e.init_ut_editor();
        e.enter();
        // println!(" enter {:?}", e.get_buf_str());
        assert_eq!(e.get_buf_str(), format!("{}{}", NEW_LINE_MARK, EOF));
        assert_eq!(e.cur, Cur { y: 1, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y - 1, e.cur.y, DType::After));

        e.insert_char('A');
        e.enter();
        assert_eq!(e.get_buf_str(), format!("{}{}{}{}", NEW_LINE_MARK, "A", NEW_LINE_MARK, EOF));
        assert_eq!(e.cur, Cur { y: 2, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y - 1, e.cur.y, DType::After));
    }

    #[test]
    fn test_back_space() {
        let mut e = Editor::default();
        // normal
        e.init_ut_editor();
        e.insert_char('A');
        e.back_space();
        assert_eq!(e.get_buf_str(), format!("{}", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));

        e.init_ut_editor();
        e.enter();
        e.back_space();
        assert_eq!(e.get_buf_str(), format!("{}", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::After));

        // sel range  one line no newline
        e.init_ut_editor();
        e.insert_char('A');
        e.shift_left();
        e.back_space();
        assert_eq!(e.get_buf_str(), format!("{}", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  one line with newline
        e.init_ut_editor();
        e.insert_char('A');
        e.enter();
        e.ctrl_home();
        e.shift_right();
        e.evt = SHIFT_RIGHT;
        e.shift_right();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        e.back_space();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        assert_eq!(e.get_buf_str(), format!("{}", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  multi line
        e.init_ut_editor();
        e.insert_str("AB");
        e.enter();
        e.insert_str("CD");
        // println!("sel range  multi line {:?}", e.get_buf_str());
        e.ctrl_home();
        e.cur_right();
        e.shift_down();
        e.back_space();
        // println!("sel range  multi line {:?}", e.get_buf_str());
        assert_eq!(e.get_buf_str(), format!("{}{}", "AD", EOF));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));
    }
}
