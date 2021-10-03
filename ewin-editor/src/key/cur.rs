use crate::{
    ewin_core::{_cfg::key::keycmd::*, def::*, global::*, log::*, model::*, util::*},
    model::*,
};
use std::cmp::min;

impl Editor {
    pub fn cur_move_com(&mut self) {
        Log::debug("self.sel.mode", &self.sel.mode);

        match self.e_cmd {
            E_Cmd::CursorUp | E_Cmd::MouseScrollUp => self.cur_up(),
            E_Cmd::CursorDown | E_Cmd::MouseScrollDown => self.cur_down(),
            E_Cmd::CursorLeft => self.cur_left(),
            E_Cmd::CursorRight => self.cur_right(),
            E_Cmd::CursorRowHome => self.cur_home(),
            E_Cmd::CursorRowEnd => self.cur_end(),
            _ => {}
        }

        if self.sel.mode == SelMode::BoxSelect {
            self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
            self.draw_range = EditorDrawRange::get_type(self.sel.mode, self.cur_y_org, self.cur.y);
            self.box_insert.vec = self.slice_box_sel().1;
        }
    }

    pub fn cur_up(&mut self) {
        if self.cur.y == 0 {
            self.draw_range = EditorDrawRange::Not;
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
            self.draw_range = EditorDrawRange::Not;
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
        if self.e_cmd == E_Cmd::CursorLeft || self.e_cmd == E_Cmd::CursorRight {
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
                if c == NEW_LINE_CR && (self.e_cmd == E_Cmd::CursorLeftSelect || self.e_cmd == E_Cmd::CursorLeft) {
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
        match self.e_cmd {
            E_Cmd::CursorRight | E_Cmd::InsertStr(_) => {
                if self.state.mouse_mode == MouseMode::Normal {
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
            E_Cmd::CursorRightSelect => {
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
                self.draw_range = EditorDrawRange::Target(self.cur.y, self.cur.y + 1);
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
                if self.e_cmd == E_Cmd::CursorRightSelect && c == NEW_LINE_CR {
                    self.cur.disp_x += 1;
                    self.cur.x += 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
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
    pub fn ctrl_home(&mut self) {
        self.updown_x = 0;
        self.set_cur_default();
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn ctrl_end(&mut self) {
        let y = self.buf.len_lines() - 1;
        let len_line_chars = self.buf.len_line_chars(y);
        self.set_cur_target(y, len_line_chars, false);
        self.scroll();
        self.scroll_horizontal();
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
    }

    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.row_num, self.buf.len_lines() - 1);
        self.cur_updown_com();
        self.scroll();
    }

    pub fn page_up(&mut self) {
        self.cur.y = if self.cur.y > self.row_num { self.cur.y - self.row_num } else { 0 };
        self.cur_updown_com();
        self.scroll();
    }
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match self.e_cmd {
            E_Cmd::CursorUpSelect => self.cur_up(),
            E_Cmd::CursorDownSelect => self.cur_down(),
            E_Cmd::CursorLeftSelect => self.cur_left(),
            E_Cmd::CursorRightSelect => self.cur_right(),
            E_Cmd::CursorRowHomeSelect => self.cur_home(),
            E_Cmd::CursorRowEndSelect => self.cur_end(),
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.draw_range = EditorDrawRange::get_type(self.sel.mode, self.cur_y_org, self.cur.y);
        self.sel.check_overlap();
    }
}
