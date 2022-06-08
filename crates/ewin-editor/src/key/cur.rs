use ewin_cfg::{log::*, model::default::*};
use ewin_const::def::*;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*, util::*},
    model::*,
};
use std::cmp::min;

impl Editor {
    pub fn cur_move_com(&mut self) {
        Log::debug_key("cur_move_com");
        Log::debug("self.sel.mode", &self.sel.mode);
        match self.e_cmd {
            E_Cmd::CursorUp | E_Cmd::MouseScrollUp => self.cur_up(),
            E_Cmd::CursorDown | E_Cmd::MouseScrollDown => self.cur_down(),
            E_Cmd::CursorLeft => self.cur_left(),
            E_Cmd::CursorRight => self.cur_right(),
            E_Cmd::CursorRowHome => self.cur_home(),
            E_Cmd::CursorRowEnd => self.cur_end(),
            E_Cmd::CursorFileHome => self.ctrl_home(),
            E_Cmd::CursorFileEnd => self.ctrl_end(),
            E_Cmd::CursorPageUp => self.page_up(),
            E_Cmd::CursorPageDown => self.page_down(),
            _ => {}
        }

        self.scroll();
        if self.sel.mode == SelMode::BoxSelect {
            self.sel.set_sel_posi(false, self.cur);
            self.box_insert.vec = self.slice_box_sel().1;
        } else {
            self.sel.clear();
        }
        Log::debug("self.sel.mode == SelMode::BoxSelect", &(self.sel.mode == SelMode::BoxSelect));
        Log::debug("self.e_cmd", &self.e_cmd);

        /*
        match self.e_cmd {
            E_Cmd::CursorUp | E_Cmd::CursorDown | E_Cmd::CursorLeft | E_Cmd::CursorRight if self.sel.mode == SelMode::BoxSelect => {}
            _ => {
                self.sel.clear();
                self.sel.mode = SelMode::Normal;
            }
        };
         */
    }

    pub fn cur_up(&mut self) {
        if !self.is_move_position_by_scrolling_enable_and_e_cmd() {
            if self.cur.y == 0 {
                return;
            }
            self.cur.y -= 1;
            self.cur_updown_com();
        }
    }

    pub fn cur_down(&mut self) {
        Log::debug_key("Editor.cur_down");
        Log::debug("self.buf.len_rows() ", &self.buf.len_rows());

        if !self.is_move_position_by_scrolling_enable_and_e_cmd() {
            if self.cur.y == self.buf.len_rows() - 1 {
                return;
            }
            self.cur.y += 1;
            self.cur_updown_com();
        }
    }

    pub fn cur_updown_com(&mut self) {
        Log::debug_s("cur_updown_com");

        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
        // Not set for Left and Right
        if self.sel.mode == SelMode::BoxSelect && get_row_x_opt(&self.buf.char_vec_row(self.cur.y), self.cur.disp_x, true, false).is_none() {
        } else {
            let (cur_x, disp_x) = get_until_disp_x(&self.buf.char_vec_row(self.cur.y), self.updown_x, false);
            self.cur.disp_x = disp_x;
            self.cur.x = cur_x;
            if self.sel.mode != SelMode::BoxSelect {
                self.scroll_horizontal();
            }
        }
    }

    pub fn cur_left(&mut self) {
        // 0, 0の位置の場合
        if self.cur.y == 0 && self.cur.x == 0 {
            return;
        // 行頭の場合
        } else if self.cur.x == 0 {
            self.cur_up();
            self.set_cur_target_by_x(self.cur.y, self.buf.len_row_chars(self.cur.y), false);
        } else {
            let c = self.buf.char(self.cur.y, self.cur.x - 1);
            if c == TAB_CHAR {
                let cfg_tab_width = Cfg::get().general.editor.tab.size;
                let (_, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.cur.y)[self.offset_x..self.cur.x - 1], self.offset_disp_x, false);
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

        let char_opt = self.buf.char_opt(self.cur.y, self.cur.x);

        match self.e_cmd {
            E_Cmd::CursorRight | E_Cmd::CursorRightSelect | E_Cmd::InsertStr(_) if self.sel.mode == SelMode::Normal => {
                if let Some(c) = char_opt {
                    if is_nl_char(c) {
                        is_end_of_line = true;
                    }
                }
            }
            _ => {}
        }
        // End of line
        Log::debug("is_end_of_line", &is_end_of_line);
        if is_end_of_line {
            self.updown_x = 0;
            self.cur.disp_x = 0;
            self.cur.x = 0;
            self.cur_down();
        } else if let Some(c) = char_opt {
            if c == TAB_CHAR {
                let cfg_tab_width = Cfg::get().general.editor.tab.size;
                let tab_width = cfg_tab_width - (self.cur.disp_x % cfg_tab_width);
                self.cur.disp_x += tab_width;
                self.cur.x += 1;
            } else {
                self.cur.disp_x += get_char_width_not_tab(&c);
                self.cur.x = min(self.cur.x + 1, self.buf.len_row_chars(self.cur.y));
                if self.e_cmd == E_Cmd::CursorRightSelect && c == NEW_LINE_CR {
                    self.cur.disp_x += 1;
                    self.cur.x += 1;
                }
            }
        } else if self.sel.mode == SelMode::BoxSelect {
            self.cur.disp_x += 1;
            self.cur.x += 1;
        }

        if self.sel.mode == SelMode::Normal {
            self.scroll();
            self.scroll_horizontal();
        }
    }
    pub fn cur_home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
        self.scroll_horizontal();
    }

    pub fn cur_end(&mut self) {
        self.set_cur_target_by_x(self.cur.y, self.buf.len_row_chars(self.cur.y), false);
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
        let y = self.buf.len_rows() - 1;
        let len_line_chars = self.buf.len_row_chars(y);
        self.set_cur_target_by_x(y, len_line_chars, false);
        self.scroll();
        self.scroll_horizontal();
        if self.updown_x == 0 {
            self.updown_x = self.cur.disp_x;
        }
    }

    pub fn page_down(&mut self) {
        self.cur.y = min(self.cur.y + self.row_len, self.buf.len_rows() - 1);
        self.cur_updown_com();
    }

    pub fn page_up(&mut self) {
        self.cur.y = if self.cur.y > self.row_len { self.cur.y - self.row_len } else { 0 };
        self.cur_updown_com();
    }
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur);

        match self.e_cmd {
            E_Cmd::CursorUpSelect => self.cur_up(),
            E_Cmd::CursorDownSelect => self.cur_down(),
            E_Cmd::CursorLeftSelect => self.cur_left(),
            E_Cmd::CursorRightSelect => self.cur_right(),
            E_Cmd::CursorRowHomeSelect => self.cur_home(),
            E_Cmd::CursorRowEndSelect => self.cur_end(),
            _ => {}
        }
        self.scroll();

        self.sel.set_sel_posi(false, self.cur);
        self.sel.check_overlap();
    }
}
