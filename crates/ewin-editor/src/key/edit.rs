use crate::{
    ewin_com::{_cfg::key::keycmd::*, clipboard::*, def::*, log::*, model::*, util::*},
    model::*,
};

impl Editor {
    pub fn insert_str(&mut self, ep: &mut Proc) {
        Log::debug_key("insert_str");
        Log::debug("ep", &ep);

        self.draw_range = if self.is_enable_syntax_highlight || self.box_insert.mode == BoxInsertMode::Insert { E_DrawRange::All } else { E_DrawRange::After(self.cur.y) };

        ep.draw_type = self.draw_range;
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
        let mut clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());

        change_nl(&mut clipboard, &self.h_file.nl);

        if self.box_insert.mode == BoxInsertMode::Normal {
            // Paste of the string copied in box insert mode
            if self.box_insert.get_str(&NL::get_nl(&self.h_file.nl)) == clipboard {
                ep.box_sel_vec = self.box_insert.vec.clone();
                ep.str = self.box_insert.get_str(&NL::get_nl(&self.h_file.nl));
            } else {
                self.set_clipboard_and_clear_box_sel(ep, clipboard);
            }
        } else if clipboard.split(&NL::get_nl(&self.h_file.nl)).count() == 1 {
            for i in 0..ep.box_sel_vec.len() {
                ep.box_sel_vec[i].1 = clipboard.clone();
            }
        } else {
            self.set_clipboard_and_clear_box_sel(ep, clipboard);
            return true;
        }
        false
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
            if sy + i <= self.buf.len_rows() - 1 {
                // If there are characters up to the column to insert
                if let Some(cur_x) = get_row_x(&self.buf.char_vec_line(sy + i)[..], s_disp_x, false) {
                    self.buf.insert(sy + i, cur_x, sel_str);
                    let sel = SelRange { sy: sy + i, sx: cur_x, ex: cur_x + sel_str.chars().count(), s_disp_x, e_disp_x: s_disp_x + get_str_width(sel_str), ..SelRange::default() };
                    box_sel_undo_vec.push((sel, sel_str.clone()));
                    box_sel_redo_vec.push((sel, sel_str.clone()));
                    if i == 0 {
                        ex = cur_x + sel_str.chars().count();
                    }
                } else {
                    let (cur_x, width) = get_row_x_disp_x(&self.buf.char_vec_line(sy + i)[..], 0, false);

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
                let end_idx = self.buf.len_line_chars(self.buf.len_rows() - 1);
                let (_, width) = get_row_x_disp_x(&self.buf.char_vec_line(self.buf.len_rows() - 1)[..], 0, false);

                self.buf.insert_end(nl_code);
                box_sel_undo_vec.push((SelRange { sy: sy + i - 1, sx: end_idx, ex: end_idx + nl_code.chars().count(), s_disp_x: width, e_disp_x: width + get_str_width(nl_code), ..SelRange::default() }, "".to_string()));

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

                e_disp_x = get_row_x_disp_x(&self.buf.char_vec_line(sy)[..ex], 0, false).1;
                self.set_cur_target(sy, sx, false);

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

    pub fn undo_del_box(&mut self, box_sel_vec: &[(SelRange, String)]) {
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

        let last_str_len = insert_strs.last().unwrap().chars().count();
        let x = if insert_strs.len() == 1 { self.cur.x + last_str_len } else { last_str_len };
        self.set_cur_target(self.cur.y, x, false);
    }

    pub fn insert_row(&mut self) {
        self.buf.insert(self.cur.y, self.cur.x, &NL::get_nl(&self.h_file.nl));
        self.set_cur_target(self.cur.y + 1, 0, false);

        self.scroll();
        self.scroll_horizontal();
    }

    pub fn backspace(&mut self, ep: &mut Proc) {
        Log::debug_key("back_space");
        // beginning of the line
        if self.cur.x == 0 {
            self.cur.y -= 1;
            let (mut cur_x, _) = get_row_x_disp_x(&self.buf.char_vec_line(self.cur.y)[..], 0, true);
            // ' ' is meaningless value
            let c = if cur_x > 0 { self.buf.char(self.cur.y, cur_x - 1) } else { ' ' };
            ep.str = if c == NEW_LINE_CR { NEW_LINE_CRLF.to_string() } else { NEW_LINE_LF.to_string() };
            // Minus for new line code
            cur_x -= 1;

            self.buf.remove_del_bs(KeyCmd::Edit(E_Cmd::DelPrevChar), self.cur.y, self.buf.len_line_chars(self.cur.y) - 1);
            self.set_cur_target(self.cur.y, cur_x, false);
            self.scroll();
            self.scroll_horizontal();
        } else {
            self.cur_left();

            if self.box_insert.mode == BoxInsertMode::Normal {
                ep.str = self.buf.char(self.cur.y, self.cur.x).to_string();
                self.buf.remove_del_bs(KeyCmd::Edit(E_Cmd::DelPrevChar), self.cur.y, self.cur.x);

                //BoxSelMode::Insert
            } else {
                Log::debug("ep.box_sel_vec", &ep.box_sel_vec);
                for i in 0..=ep.box_sel_vec.len() - 1 {
                    let s = self.buf.line_to_char(ep.box_sel_vec[i].0.sy) + self.cur.x;
                    let e = self.buf.line_to_char(ep.box_sel_vec[i].0.sy) + self.cur.x + 1;
                    let c = self.buf.char_idx(s);
                    ep.box_sel_vec[i].1 = c.to_string().clone();
                    self.buf.remove(s, e);
                    let (_, width) = get_row_x_disp_x(&self.buf.char_vec_line(ep.box_sel_vec[i].0.sy)[..self.cur.x], 0, false);
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
        self.buf.remove_del_bs(KeyCmd::Edit(E_Cmd::DelNextChar), self.cur.y, self.cur.x);
        if is_line_end(c) {
            self.set_cur_target(self.cur.y, self.cur.x, false);
            self.scroll();
            self.scroll_horizontal();
        }
    }

    pub fn cancel_mode_and_search_result(&mut self) {
        self.sel.clear();
        self.sel.mode = SelMode::Normal;
        self.box_insert.mode = BoxInsertMode::Normal;
        self.search.clear();
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
