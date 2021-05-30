use crate::{def::*, global::CFG, log::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*};
use std::cmp::min;

impl Editor {
    pub fn cur_up(&mut self) {
        if self.cur.y == 0 {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        if self.cur.y > 0 {
            self.cur.y -= 1;
            self.cur_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_down(&mut self) {
        if self.cur.y == self.buf.len_lines() {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        Log::debug_s("              c_d start");
        if self.cur.y + 1 < self.buf.len_lines() {
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
        // Not set for Left and Right
        if self.evt == Key(Left.into()) || self.evt == Key(Right.into()) {
        } else {
            let (cur_x, disp_x) = get_until_x(&self.buf.char_vec_line(self.cur.y), self.updown_x);
            self.cur.disp_x = disp_x;
            self.cur.x = cur_x;
        }
    }

    pub fn cur_left(&mut self) {
        // 0, 0の位置の場合
        if self.cur.y == 0 && self.cur.x == 0 {
            return;
        // 行頭の場合
        } else if self.cur.x == 0 {
            self.cur_up();
            self.set_cur_target_ex(self.cur.y, self.buf.len_line_chars(self.cur.y), false);
        } else {
            let c = self.buf.char(self.cur.y, self.cur.x - 1);

            if c == TAB {
                let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.width;
                let (_, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[0..self.cur.x - 1], self.offset_disp_x, false);
                self.cur.disp_x -= cfg_tab_width - width % cfg_tab_width;
                self.cur.x -= 1;
            } else {
                self.cur.x -= 1;
                self.cur.disp_x -= get_char_width_not_tab(c);
                if c == NEW_LINE_CR && (self.evt == SHIFT_LEFT || self.evt == LEFT) {
                    self.cur.disp_x -= 1;
                    self.cur.x -= 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn get_tab_pre_width(&mut self) {}

    pub fn cur_right(&mut self) {
        let mut is_end_of_line = false;
        let c = self.buf.char(self.cur.y, self.cur.x);
        if self.evt == RIGHT || self.evt == PASTE {
            if self.mode == TermMode::Normal {
                if is_line_end(c) {
                    is_end_of_line = true;
                }
            } else {
                let vec = self.buf.char_vec_line(self.cur.y);
                let (cur_x, _) = get_row_width(&vec[..], self.offset_disp_x, false);
                if self.cur.x == cur_x {
                    is_end_of_line = true;
                }
            }
        } else if self.evt == SHIFT_RIGHT {
            let len_line_chars = self.buf.len_line_chars(self.cur.y);
            let x = if c == NEW_LINE_CR { len_line_chars - 1 } else { len_line_chars };
            if self.cur.x == x {
                is_end_of_line = true;
            }
        }

        // End of line
        if is_end_of_line {
            // Last line
            if self.cur.y == self.buf.len_lines() - 1 {
                return;
            } else {
                self.updown_x = 0;
                self.cur.disp_x = 0;
                self.cur.x = 0;
                self.cur_down();
            }
        } else {
            if c == EOF_MARK {
                return;
            }

            if c == TAB {
                let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.width;
                let tab_width = cfg_tab_width - (self.cur.disp_x % cfg_tab_width);
                self.cur.disp_x += tab_width;
                self.cur.x += 1;
            } else {
                self.cur.disp_x += get_char_width_not_tab(c);
                self.cur.x = min(self.cur.x + 1, self.buf.len_line_chars(self.cur.y));
                if self.evt == SHIFT_RIGHT && c == NEW_LINE_CR {
                    self.cur.disp_x += 1;
                    self.cur.x += 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn enter(&mut self) {
        let nl_str = if self.h_file.nl == NEW_LINE_LF_STR { NEW_LINE_LF.to_string() } else { NEW_LINE_CRLF.to_string() };
        self.buf.insert(self.cur.y, self.cur.x, &nl_str);
        self.set_cur_target_ex(self.cur.y + 1, 0, false);
        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        } else {
            self.d_range = DRange::new(self.cur.y - 1, 0, DrawType::After);
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn insert_char(&mut self, c: char) {
        self.buf.insert_char(self.cur.y, self.cur.x, c);
        self.cur_right();
        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        } else {
            self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::After);
        }
    }

    pub fn back_space(&mut self, ep: &mut EvtProc) {
        Log::debug_s("              back_space");
        // beginning of the line
        if self.cur.x == 0 {
            self.cur.y -= 1;
            self.d_range = DRange::new(self.cur.y, 0, DrawType::After);
            let (mut cur_x, _) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], self.offset_disp_x, true);

            // ' ' is meaningless value
            let c = if cur_x > 0 { self.buf.char(self.cur.y, cur_x - 1) } else { ' ' };
            ep.str = if c == NEW_LINE_CR { NEW_LINE_CRLF.to_string() } else { NEW_LINE_LF.to_string() };
            // Minus for newline code
            cur_x -= 1;

            self.buf.remove_del_bs(EvtType::BS, self.cur.y, self.buf.len_line_chars(self.cur.y) - 1);
            self.set_cur_target_ex(self.cur.y, cur_x, false);
            self.scroll();
            self.scroll_horizontal();
        } else {
            self.cur_left();
            ep.str = self.buf.char(self.cur.y, self.cur.x).to_string();
            self.buf.remove_del_bs(EvtType::BS, self.cur.y, self.cur.x);
            if self.is_enable_syntax_highlight {
                self.d_range.draw_type = DrawType::All;
            } else {
                self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);
            }
        }
    }

    pub fn delete(&mut self, ep: &mut EvtProc) {
        Log::debug_s("              delete");
        let c = self.buf.char(self.cur.y, self.cur.x);
        ep.str = if c == NEW_LINE_CR { format!("{}{}", c.to_string(), NEW_LINE_LF) } else { c.to_string() };
        self.buf.remove_del_bs(EvtType::Del, self.cur.y, self.cur.x);
        self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);

        if is_line_end(c) {
            self.set_cur_target_ex(self.cur.y, self.cur.x, false);
            self.d_range.draw_type = DrawType::After;
            self.scroll();
            self.scroll_horizontal();
        }
        if self.is_enable_syntax_highlight {
            self.d_range.draw_type = DrawType::All;
        }
    }

    pub fn home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
        self.scroll_horizontal();
    }

    pub fn end(&mut self) {
        self.set_cur_target_ex(self.cur.y, self.buf.len_line_chars(self.cur.y), false);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.disp_row_num, self.buf.len_lines() - 1);
        self.cur_updown_com();
        self.scroll();
    }

    pub fn page_up(&mut self) {
        self.cur.y = if self.cur.y > self.disp_row_num { self.cur.y - self.disp_row_num } else { 0 };
        self.cur_updown_com();
        self.scroll();
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::*;


    #[test]
    pub fn test_insert_char() {
        let (mut e, mut mbar) = UT::init_ut();

        // first char
        e.insert_char('A');
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}", "A", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));

        // second char
        e.insert_char('B');
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}", "AB", EOF_MARK));
        // println!(" multi char {:?}", e.get_buf_str());
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 2, disp_x: e.rnw + 1 + 2 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));

        UT::undo_all(&mut e, &mut mbar);
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
    }
    #[test]
    fn test_enter() {
        let (mut e, mut mbar) = UT::init_ut();

        e.enter();
        // println!(" enter {:?}", e.get_buf_str());
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}", NEW_LINE, EOF_MARK));
        assert_eq!(e.cur, Cur { y: 1, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y - 1, e.cur.y, DType::After));

        e.insert_char('A');
        e.enter();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}{}{}", NEW_LINE, "A", NEW_LINE, EOF_MARK));
        assert_eq!(e.cur, Cur { y: 2, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y - 1, e.cur.y, DType::After));

        UT::undo_all(&mut e, &mut mbar);
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
    }

    #[test]
    fn test_back_space() {
        let (mut e, mut mbar) = UT::init_ut();
        // normal
        e.insert_char('A');
        e.back_space();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));

        e.enter();
        e.back_space();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::After));

        // sel range  one line no newline
        e.insert_char('A');
        e.shift_left();
        e.back_space();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  one line with newline
        e.insert_char('A');
        e.enter();
        e.ctrl_home();
        e.shift_right();
        e.evt = SHIFT_RIGHT;
        e.shift_right();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        e.back_space();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  multi line
        UT::insert_str(&mut e, "AB");
        e.enter();
        UT::insert_str(&mut e, "CD");
        e.ctrl_home();
        e.cur_right();
        e.shift_down();
        e.back_space();
        // println!("sel range  multi line {:?}", e.get_buf_str());
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}", "AD", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        UT::undo_all(&mut e, &mut mbar);
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
    }

    #[test]
    fn test_delete() {
        let (mut e, mut mbar) = UT::init_ut();

        // normal
        e.insert_char('A');
        e.cur_left();
        e.delete();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::Target));

        e.enter();
        e.cur_left();
        e.delete();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.cur.y, e.cur.y, DType::After));

        // sel range  one line no newline
        e.insert_char('A');
        e.shift_left();
        e.delete();
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  one line with newline
        e.insert_char('A');
        e.enter();
        e.ctrl_home();
        e.shift_right();
        e.evt = SHIFT_RIGHT;
        e.shift_right();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        e.delete();
        // println!(" sel range  one line with newline {:?}", e.get_buf_str());
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw, disp_x: e.rnw + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        // sel range  multi line
        UT::insert_str(&mut e, "AB");
        e.enter();
        UT::insert_str(&mut e, "CD");
        e.ctrl_home();
        e.cur_right();
        e.shift_down();
        e.delete();
        // println!("sel range  multi line {:?}", e.get_buf_str());
        assert_eq!(UT::get_buf_str(&mut e), format!("{}{}", "AD", EOF_MARK));
        assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 1 });
        assert_eq!(e.d_range, DRnage::new(e.sel.sy, e.sel.sy, DType::After));

        UT::undo_all(&mut e, &mut mbar);
        assert_eq!(UT::get_buf_str(&mut e), format!("{}", EOF_MARK));
    }
    #[test]
    fn test_cur_down() {
        let (mut e, _) = UT::init_ut();

        e.enter();
        e.cur_up();
        e.cur_down();
        assert_eq!(e.cur, Cur { y: 1, x: e.rnw, disp_x: e.rnw + 1 });

        e.set_cur_default();
        e.insert_char('A');
        e.cur_down();
        assert_eq!(e.cur, Cur { y: 1, x: e.rnw, disp_x: e.rnw + 1 });

        /*
             e.insert_char('あ');
             e.cur_up();
             assert_eq!(e.cur, Cur { y: 0, x: e.rnw + 1, disp_x: e.rnw + 1 + 2 });
        */
    }
    */
}
