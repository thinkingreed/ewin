use crate::{
    _cfg::keys::KeyCmd,
    clipboard::get_clipboard,
    def::*,
    global::CFG,
    log::*,
    model::*,
    sel_range::{BoxInsertMode, SelMode, SelRange},
    util::*,
};
use std::cmp::min;

impl Editor {
    pub fn cur_move_com(&mut self) {
        Log::debug("self.sel.mode", &self.sel.mode);

        match self.keycmd {
            KeyCmd::CursorUp | KeyCmd::MouseScrollUp => self.cur_up(),
            KeyCmd::CursorDown | KeyCmd::MouseScrollDown => self.cur_down(),
            KeyCmd::CursorLeft => self.cur_left(),
            KeyCmd::CursorRight => self.cur_right(),
            KeyCmd::CursorRowHome => self.cur_home(),
            KeyCmd::CursorRowEnd => self.cur_end(),
            _ => {}
        }

        if self.sel.mode == SelMode::BoxSelect {
            self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
            self.draw_type = DrawType::get_type(self.sel.mode, self.cur_y_org, self.cur.y);
            self.box_insert.vec = self.slice_box_sel().1;
        }
    }

    pub fn cur_up(&mut self) {
        if self.cur.y == 0 {
            self.draw_type = DrawType::Not;
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
        Log::debug_key("c_d start");
        if self.cur.y == self.buf.len_lines() {
            self.draw_type = DrawType::Not;
            return;
        }
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
        if self.keycmd == KeyCmd::CursorLeft || self.keycmd == KeyCmd::CursorRight {
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
            self.set_cur_target(self.cur.y, self.buf.len_line_chars(self.cur.y), false);
        } else {
            let c = self.buf.char(self.cur.y, self.cur.x - 1);

            if c == TAB_CHAR {
                let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.size;
                let (_, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[self.offset_x..self.cur.x - 1], self.offset_disp_x, false);
                self.cur.disp_x -= cfg_tab_width - ((self.offset_disp_x + width) % cfg_tab_width);
                self.cur.x -= 1;
            } else {
                self.cur.x -= 1;
                self.cur.disp_x -= get_char_width_not_tab(&c);
                if c == NEW_LINE_CR && (self.keycmd == KeyCmd::CursorLeftSelect || self.keycmd == KeyCmd::CursorLeft) {
                    self.cur.disp_x -= 1;
                    self.cur.x -= 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_right(&mut self) {
        let mut is_end_of_line = false;
        let c = self.buf.char(self.cur.y, self.cur.x);
        match self.keycmd {
            KeyCmd::CursorRight | KeyCmd::InsertStr(_) => {
                if self.mouse_mode == MouseMode::Normal {
                    if is_line_end(c) {
                        is_end_of_line = true;
                    }
                } else {
                    let (cur_x, _) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], 0, false);
                    if self.cur.x == cur_x {
                        is_end_of_line = true;
                    }
                }
            }
            KeyCmd::CursorRightSelect => {
                let len_line_chars = self.buf.len_line_chars(self.cur.y);
                let x = if c == NEW_LINE_CR { len_line_chars - 1 } else { len_line_chars };
                if self.cur.x == x - 1 {
                    is_end_of_line = true;
                }
            }
            _ => {}
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
                self.draw_type = DrawType::Target(self.cur.y, self.cur.y + 1);
                self.cur_down();
            }
        } else {
            if c == EOF_MARK {
                return;
            }
            if c == TAB_CHAR {
                let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.size;
                let tab_width = cfg_tab_width - (self.cur.disp_x % cfg_tab_width);
                self.cur.disp_x += tab_width;
                self.cur.x += 1;
            } else {
                self.cur.disp_x += get_char_width_not_tab(&c);
                self.cur.x = min(self.cur.x + 1, self.buf.len_line_chars(self.cur.y));
                if self.keycmd == KeyCmd::CursorRightSelect && c == NEW_LINE_CR {
                    self.cur.disp_x += 1;
                    self.cur.x += 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn insert_str(&mut self, ep: &mut Proc) {
        Log::debug_key("insert_str");
        Log::debug("ep", &ep);

        if self.is_enable_syntax_highlight {
            self.draw_type = DrawType::All;
        } else {
            self.draw_type = if self.box_insert.mode == BoxInsertMode::Insert { DrawType::All } else { DrawType::After(self.cur.y) };
        }
        ep.draw_type = self.draw_type;
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
        if ep.box_sel_vec.is_empty() {
            self.ins_str(&ep.str);
            ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
            ep.cur_e = self.cur;
        } else {
            let first_sel = ep.box_sel_vec.first().unwrap().0;
            if self.sel.is_selected() {
                if !ep.str.is_empty() {
                    self.insert_box(ep, first_sel.sy, first_sel.sx, first_sel.s_disp_x)
                }
            } else if self.box_insert.mode == BoxInsertMode::Insert {
                self.insert_box(ep, first_sel.sy, first_sel.sx, first_sel.s_disp_x)
            } else {
                self.insert_box(ep, self.cur.y, self.cur.x, self.cur.disp_x)
            }
        }
    }

    pub fn get_clipboard(&mut self, ep: &mut Proc) -> bool {
        Log::debug_key("get_clipboard");
        // for Paste
        let mut clipboard = get_clipboard().unwrap_or("".to_string());

        change_nl(&mut clipboard, &self.h_file);

        if self.box_insert.mode == BoxInsertMode::Normal {
            // Paste of the string copied in box insert mode
            if self.box_insert.get_str(&NL::get_nl(&self.h_file.nl)) == clipboard {
                ep.box_sel_vec = self.box_insert.vec.clone();
                ep.str = self.box_insert.get_str(&NL::get_nl(&self.h_file.nl));
            } else {
                self.set_clipboard_and_clear_box_sel(ep, clipboard);
            }
        } else {
            let row_vec = clipboard.split(&NL::get_nl(&self.h_file.nl)).collect::<Vec<&str>>();

            if row_vec.len() == 1 {
                for i in 0..ep.box_sel_vec.len() {
                    ep.box_sel_vec[i].1 = clipboard.clone();
                }
            } else {
                self.set_clipboard_and_clear_box_sel(ep, clipboard);
                return true;
            }
        }

        return false;
    }
    pub fn set_clipboard_and_clear_box_sel(&mut self, ep: &mut Proc, clipboard: String) {
        self.box_insert.clear_clipboard();
        ep.str = clipboard;
        ep.box_sel_vec.clear();
    }

    pub fn insert_box(&mut self, ep: &mut Proc, sy: usize, sx: usize, s_disp_x: usize) {
        Log::debug_key("insert_box");
        Log::debug("ep.box_sel_vec", &ep.box_sel_vec);
        let (mut ex, mut box_sel_redo_vec, mut box_sel_undo_vec) = (0, vec![], vec![]);

        let vec_len = ep.box_sel_vec.len() - 1;
        for (i, (_, sel_str)) in ep.box_sel_vec.iter().enumerate() {
            // Exist row
            if sy + i <= self.buf.len_lines() - 1 {
                // If there are characters up to the column to insert
                if let Some(cur_x) = get_row_x(&self.buf.char_vec_line(sy + i)[..], s_disp_x, false) {
                    self.buf.insert(sy + i, cur_x, &sel_str);
                    let sel = SelRange { sy: sy + i, sx: cur_x, ex: cur_x + sel_str.chars().count(), s_disp_x, e_disp_x: s_disp_x + get_str_width(sel_str), ..SelRange::default() };
                    box_sel_undo_vec.push((sel, sel_str.clone()));
                    box_sel_redo_vec.push((sel, sel_str.clone()));
                    if i == 0 {
                        ex = cur_x + sel_str.chars().count();
                    }
                } else {
                    let (cur_x, width) = get_row_width(&self.buf.char_vec_line(sy + i)[..], 0, false);

                    let insert_str = format!("{}{}", " ".repeat(s_disp_x - width), &sel_str);
                    self.buf.insert(sy + i, cur_x, &insert_str);
                    box_sel_undo_vec.push((SelRange { sy: sy + i, sx: cur_x, ex: cur_x + insert_str.chars().count(), s_disp_x: width, e_disp_x: width + get_str_width(&insert_str), ..SelRange::default() }, sel_str.to_string()));
                    box_sel_redo_vec.push((SelRange { sy: sy + i, sx: cur_x + s_disp_x - width, s_disp_x, ex: sx + insert_str.chars().count(), ..SelRange::default() }, sel_str.to_string()));
                    if i == 0 {
                        ex = " ".repeat(s_disp_x - width).chars().count() + sel_str.chars().count();
                    }
                }
            } else {
                //// Not exist row, Create new row
                // Delete EOF_MARK once
                self.buf.remove(self.buf.len_chars() - 1, self.buf.len_chars());

                // Insert a new line at the end of the current last line
                let nl_code = &NL::get_nl(&self.h_file.nl);
                let end_idx = self.buf.len_line_chars(self.buf.len_lines() - 1);
                let (_, width) = get_row_width(&self.buf.char_vec_line(self.buf.len_lines() - 1)[..], 0, false);

                self.buf.insert_end(nl_code);
                box_sel_undo_vec.push((SelRange { sy: sy + i - 1, sx: end_idx, ex: end_idx + nl_code.chars().count(), s_disp_x: width, e_disp_x: width + get_str_width(&nl_code), ..SelRange::default() }, "".to_string()));

                // add new row
                let new_row_str = &format!("{}{}", " ".repeat(s_disp_x), &sel_str);
                self.buf.insert_end(new_row_str);
                box_sel_undo_vec.push((SelRange { sy: sy + i, sx: 0, ex: new_row_str.chars().count(), s_disp_x: 0, e_disp_x: get_str_width(new_row_str), ..SelRange::default() }, sel_str.to_string()));
                box_sel_redo_vec.push((SelRange { sy: sy + i, sx: s_disp_x, s_disp_x, ex: sx + sel_str.chars().count(), ..SelRange::default() }, sel_str.to_string()));

                self.buf.insert_end(&EOF_MARK.to_string());
                if i == 0 {
                    ex = " ".repeat(s_disp_x).chars().count() + sel_str.chars().count();
                }
            }

            if i == vec_len {
                let ey = sy + i;
                let e_disp_x;
                if self.box_insert.mode == BoxInsertMode::Normal {
                    e_disp_x = get_row_width(&self.buf.char_vec_line(sy)[..ex], 0, false).1;
                    self.set_cur_target(sy, sx, false);
                } else {
                    // BoxSelMode::InsertStr
                    e_disp_x = get_row_width(&self.buf.char_vec_line(sy)[..ex], 0, false).1;
                    self.set_cur_target(sy, ex, false);
                }
                ep.cur_e = self.cur;
                ep.sel.set_e(ey, ex, e_disp_x);
            }
        }
        ep.box_sel_vec = box_sel_undo_vec;

        ep.box_sel_redo_vec = box_sel_redo_vec;
        if self.box_insert.mode == BoxInsertMode::Insert {
            self.box_insert.vec = ep.box_sel_vec.clone();
        }
    }

    pub fn undo_del_box(&mut self, box_sel_vec: &Vec<(SelRange, String)>) {
        Log::debug_key("undo_del_box");
        for (sel, _) in box_sel_vec.iter().rev() {
            Log::debug("sel", &sel);

            let s_idx = self.buf.line_to_char(sel.sy) + sel.sx;
            let e_idx = self.buf.line_to_char(sel.sy) + sel.ex;
            self.buf.remove(s_idx, e_idx);
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ins_str(&mut self, str: &str) {
        Log::debug_key("ins_str");

        self.buf.insert(self.cur.y, self.cur.x, str);
        let insert_strs: Vec<&str> = str.split(&NL::get_nl(&self.h_file.nl)).collect();

        self.cur.y += insert_strs.len() - 1;

        Log::debug("insert_strs", &insert_strs);

        Log::debug("self.cur 111", &self.cur);

        let last_str_len = insert_strs.last().unwrap().chars().count();
        let x = if insert_strs.len() == 1 { self.cur.x + last_str_len } else { last_str_len };
        self.set_cur_target(self.cur.y, x, false);

        Log::debug("self.cur 222", &self.cur);
    }

    pub fn enter(&mut self) {
        self.buf.insert(self.cur.y, self.cur.x, &NL::get_nl(&self.h_file.nl));
        self.set_cur_target(self.cur.y + 1, 0, false);

        self.set_draw_range_each_process(DrawType::After(self.cur.y - 1));

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn backspace(&mut self, ep: &mut Proc) {
        Log::debug_key("back_space");
        // beginning of the line
        if self.cur.x == 0 {
            self.cur.y -= 1;
            self.draw_type = DrawType::After(self.cur.y);
            let (mut cur_x, _) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], 0, true);
            // ' ' is meaningless value
            let c = if cur_x > 0 { self.buf.char(self.cur.y, cur_x - 1) } else { ' ' };
            ep.str = if c == NEW_LINE_CR { NEW_LINE_CRLF.to_string() } else { NEW_LINE_LF.to_string() };
            // Minus for new line code
            cur_x -= 1;

            self.buf.remove_del_bs(KeyCmd::DeletePrevChar, self.cur.y, self.buf.len_line_chars(self.cur.y) - 1);
            self.set_cur_target(self.cur.y, cur_x, false);
            self.scroll();
            self.scroll_horizontal();
        } else {
            self.cur_left();

            if self.box_insert.mode == BoxInsertMode::Normal {
                ep.str = self.buf.char(self.cur.y, self.cur.x).to_string();
                self.buf.remove_del_bs(KeyCmd::DeletePrevChar, self.cur.y, self.cur.x);
                if self.is_enable_syntax_highlight {
                    self.draw_type = DrawType::All;
                } else {
                    self.draw_type = DrawType::Target(self.cur.y, self.cur.y);
                }
                //BoxSelMode::Insert
            } else {
                Log::debug("ep.box_sel_vec", &ep.box_sel_vec);
                for i in 0..=ep.box_sel_vec.len() - 1 {
                    let s = self.buf.line_to_char(ep.box_sel_vec[i].0.sy) + self.cur.x;
                    let e = self.buf.line_to_char(ep.box_sel_vec[i].0.sy) + self.cur.x + 1;
                    let c = self.buf.char_idx(s);
                    ep.box_sel_vec[i].1 = c.to_string().clone();
                    self.buf.remove(s, e);
                    let (_, width) = get_row_width(&self.buf.char_vec_line(ep.box_sel_vec[i].0.sy)[..self.cur.x], 0, false);
                    ep.box_sel_vec[i].0.sx = self.cur.x;
                    ep.box_sel_vec[i].0.s_disp_x = width;
                    ep.box_sel_vec[i].0.ex = self.cur.x + 1;
                    ep.box_sel_vec[i].0.e_disp_x = width + get_char_width(&c, width);
                }
                ep.cur_e = self.cur;
            }
        }
    }

    pub fn delete(&mut self, ep: &mut Proc) {
        Log::debug_key("delete");
        let c = self.buf.char(self.cur.y, self.cur.x);
        ep.str = if c == NEW_LINE_CR { format!("{}{}", c.to_string(), NEW_LINE_LF) } else { c.to_string() };
        self.buf.remove_del_bs(KeyCmd::DeleteNextChar, self.cur.y, self.cur.x);
        let mut draw_type = DrawType::Target(self.cur.y, self.cur.y);
        if is_line_end(c) {
            self.set_cur_target(self.cur.y, self.cur.x, false);
            self.scroll();
            self.scroll_horizontal();
            draw_type = DrawType::After(self.cur.y);
        }
        self.set_draw_range_each_process(draw_type);
    }

    pub fn cur_home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
        self.scroll_horizontal();
    }

    pub fn cur_end(&mut self) {
        self.set_cur_target(self.cur.y, self.buf.len_line_chars(self.cur.y), false);
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

    pub fn cancel_mode(&mut self) {
        self.sel.clear();
        self.sel.mode = SelMode::Normal;
        self.box_insert.mode = BoxInsertMode::Normal;
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
