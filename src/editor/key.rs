use crate::{def::*, log::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*};
use std::cmp::{max, min};

impl Editor {
    pub fn cur_up(&mut self) {
        Log::ep_s("　　　　　　　 c_u start");
        Log::ep("self.cur.y", &self.cur.y);
        Log::ep("self.disp_row_posi", &self.disp_row_posi);
        if self.cur.y > 0 {
            self.cur.y -= 1;
            self.cur_updown_com();
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_down(&mut self) {
        Log::ep_s("　　　　　　　 c_d start");
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
        // Left, Rightの場合は設定しない
        if self.evt == Key(Left.into()) || self.evt == Key(Right.into()) {
        } else {
            let (cur_x, disp_x) = get_until_updown_x(&self.buf.char_vec_line(self.cur.y), self.updown_x - self.rnw);
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
            self.cur_up();
            self.set_cur_target(self.cur.y, self.buf.len_line_chars(self.cur.y));
        } else {
            self.cur.x = max(self.cur.x - 1, self.rnw);
            self.cur.disp_x -= get_char_width(self.buf.char(self.cur.y, self.cur.x - self.rnw));
            let c = self.buf.char(self.cur.y, self.cur.x - self.rnw);
            if c == NEW_LINE_CR && (self.evt == SHIFT_LEFT || self.evt == LEFT) {
                self.cur.disp_x -= 1;
                self.cur.x -= 1;
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_right(&mut self) {
        Log::ep_s("　　　　　　　  c_r start");

        let mut is_end_of_line = false;
        let c = self.buf.char(self.cur.y, self.cur.x - self.rnw);
        if self.evt == RIGHT || self.evt == PASTE {
            if is_line_end(c) {
                is_end_of_line = true;
            }
        } else if self.evt == SHIFT_RIGHT && self.cur.x == self.buf.len_line_chars(self.cur.y) + self.rnw {
            is_end_of_line = true;
        }

        // End of line
        if is_end_of_line {
            // Last line
            if self.cur.y == self.buf.len_lines() - 1 {
                return;
            } else {
                self.updown_x = self.rnw;
                self.cur.disp_x = self.rnw + 1;
                self.cur.x = self.rnw;
                self.cur_down();
            }
        } else {
            if c == EOF_MARK {
                return;
            }
            self.cur.disp_x += get_char_width(self.buf.char(self.cur.y, self.cur.x - self.rnw));
            self.cur.x = min(self.cur.x + 1, self.buf.len_line_chars(self.cur.y) + self.rnw);
            if self.evt == SHIFT_RIGHT && c == NEW_LINE_CR {
                self.cur.disp_x += 1;
                self.cur.x += 1;
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn enter(&mut self) {
        Log::ep_s("　　　　　　　  enter");
        self.buf.insert_char(self.cur.y, self.cur.x - self.rnw, NEW_LINE);
        self.set_cur_target(self.cur.y + 1, 0);
        self.d_range = DRange::new(self.cur.y - 1, 0, DrawType::After);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn insert_char(&mut self, c: char) {
        self.buf.insert_char(self.cur.y, self.cur.x - self.rnw, c);
        self.cur_right();
        // TODO ファイル形式でTarget・After判定
        self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);
    }

    pub fn back_space(&mut self, ep: &mut EvtProc) {
        Log::ep_s("　　　　　　　  back_space");
        // beginning of the line
        if self.cur.x == self.rnw {
            self.cur.y -= 1;
            self.d_range = DRange::new(self.cur.y, 0, DrawType::After);
            let (mut cur_x, _) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], true);

            // ' ' is meaningless value
            let c = if cur_x > 0 { self.buf.char(self.cur.y, cur_x - 1) } else { ' ' };
            ep.str = if c == NEW_LINE_CR { NEW_LINE_CRLF.to_string() } else { NEW_LINE.to_string() };
            // Minus for newline code
            cur_x -= 1;

            self.buf.remove_del_bs(EvtType::BS, self.cur.y, self.buf.len_line_chars(self.cur.y) - 1);
            self.set_cur_target(self.cur.y, cur_x);
            self.scroll();
            self.scroll_horizontal();
        } else {
            self.cur_left();
            ep.str = self.buf.char(self.cur.y, self.cur.x - self.rnw).to_string();
            self.buf.remove_del_bs(EvtType::BS, self.cur.y, self.cur.x - self.rnw);
            self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);
        }
    }

    pub fn delete(&mut self, ep: &mut EvtProc) {
        Log::ep_s("　　　　　　　  delete");
        let c = self.buf.char(self.cur.y, self.cur.x - self.rnw);
        ep.str = if c == NEW_LINE_CR { format!("{}{}", c.to_string(), NEW_LINE) } else { c.to_string() };
        self.buf.remove_del_bs(EvtType::Del, self.cur.y, self.cur.x - self.rnw);
        self.d_range = DRange::new(self.cur.y, self.cur.y, DrawType::Target);

        if is_line_end(c) {
            self.set_cur_target(self.cur.y, self.cur.x - self.rnw);
            self.d_range.draw_type = DrawType::After;
            self.scroll();
            self.scroll_horizontal();
        }
    }

    pub fn home(&mut self) {
        Log::ep_s("　　　　　　　  home");
        self.cur.x = self.rnw;
        self.cur.disp_x = self.rnw + 1;
        self.scroll_horizontal();
    }

    pub fn end(&mut self) {
        self.set_cur_target(self.cur.y, self.buf.len_line_chars(self.cur.y));
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
