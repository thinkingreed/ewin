use crate::model::*;
use ewin_cfg::{log::*, model::default::*};
use ewin_const::def::*;
use ewin_key::{key::cmd::*, sel_range::*};

use ewin_utils::char_edit::*;
use std::cmp::min;

impl Editor {
    pub fn cur_move_com(&mut self) {
        Log::debug_key("cur_move_com");
        Log::debug("self.sel.mode", &self.win_mgr.curt().sel.mode);
        match self.cmd.cmd_type {
            CmdType::CursorUp | CmdType::MouseScrollUp => self.cur_up(),
            CmdType::CursorDown | CmdType::MouseScrollDown => self.cur_down(),
            CmdType::CursorLeft => self.cur_left(),
            CmdType::CursorRight => self.cur_right(),
            CmdType::CursorRowHome => self.cur_home(),
            CmdType::CursorRowEnd => self.cur_end(),
            CmdType::CursorFileHome => self.ctrl_home(),
            CmdType::CursorFileEnd => self.ctrl_end(),
            CmdType::CursorPageUp => self.page_up(),
            CmdType::CursorPageDown => self.page_down(),
            _ => {}
        }

        self.scroll();
        if self.win_mgr.curt().sel.mode == SelMode::BoxSelect {
            let cur = self.win_mgr.curt().cur;
            self.win_mgr.curt().sel.set_sel_posi(false, cur);
            self.box_insert.vec = self.slice_box_sel().1;
        } else if self.cmd.cmd_type != CmdType::MouseScrollUp && self.cmd.cmd_type != CmdType::MouseScrollDown {
            self.win_mgr.curt().sel.clear();
        }
        Log::debug("self.sel.mode == SelMode::BoxSelect", &(self.win_mgr.curt().sel.mode == SelMode::BoxSelect));
        Log::debug("self.cmd", &self.cmd);
        Log::debug("self.win.curt_ref()", &self.win_mgr.curt_ref());
    }

    pub fn cur_up(&mut self) {
        if !self.is_move_position_by_scrolling_enable_and_cmd() {
            if self.win_mgr.curt().cur.y == 0 {
                return;
            }
            self.win_mgr.curt().cur.y -= 1;
            self.cur_updown_com();
        }
    }

    pub fn cur_down(&mut self) {
        Log::debug_key("Editor.cur_down");
        Log::debug("self.buf.len_rows() ", &self.buf.len_rows());

        if !self.is_move_position_by_scrolling_enable_and_cmd() {
            if self.win_mgr.curt().cur.y == self.buf.len_rows() - 1 {
                return;
            }
            self.win_mgr.curt().cur.y += 1;
            self.cur_updown_com();
        }
    }

    pub fn cur_updown_com(&mut self) {
        Log::debug_s("cur_updown_com");

        if self.win_mgr.curt().updown_x == 0 {
            self.win_mgr.curt().updown_x = self.win_mgr.curt().cur.disp_x;
        }
        // Not set for Left and Right
        if self.win_mgr.curt().sel.mode == SelMode::BoxSelect && get_row_x_opt(&self.buf.char_vec_row(self.win_mgr.curt().cur.y), self.win_mgr.curt().cur.disp_x, true, false).is_none() {
        } else {
            let (cur_x, disp_x) = get_until_disp_x(&self.buf.char_vec_row(self.win_mgr.curt().cur.y), self.win_mgr.curt().updown_x, false);
            self.win_mgr.curt().cur.disp_x = disp_x;
            self.win_mgr.curt().cur.x = cur_x;
            if self.win_mgr.curt().sel.mode != SelMode::BoxSelect {
                self.scroll_horizontal();
            }
        }
    }

    pub fn cur_left(&mut self) {
        // 0, 0の位置の場合
        if self.win_mgr.curt().cur.y == 0 && self.win_mgr.curt().cur.x == 0 {
            return;
        // 行頭の場合
        } else if self.win_mgr.curt().cur.x == 0 {
            self.cur_up();
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.buf.len_row_chars(self.win_mgr.curt_ref().cur.y), false);
        } else {
            let c = self.buf.char(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur.x - 1);
            if c == TAB_CHAR {
                let cfg_tab_width = Cfg::get().general.editor.tab.size;
                let (_, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.win_mgr.curt().cur.y)[self.win_mgr.curt().offset.x..self.win_mgr.curt().cur.x - 1], self.win_mgr.curt().offset.disp_x, false);
                self.win_mgr.curt().cur.disp_x -= cfg_tab_width - ((self.win_mgr.curt().offset.disp_x + width) % cfg_tab_width);
                self.win_mgr.curt().cur.x -= 1;
            } else {
                self.win_mgr.curt().cur.x -= 1;
                self.win_mgr.curt().cur.disp_x -= get_char_width_not_tab(&c);
                if c == NEW_LINE_CR && (self.cmd.cmd_type == CmdType::CursorLeftSelect || self.cmd.cmd_type == CmdType::CursorLeft) {
                    self.win_mgr.curt().cur.disp_x -= 1;
                    self.win_mgr.curt().cur.x -= 1;
                }
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn cur_right(&mut self) {
        Log::debug_key("Editor.cur_right");

        let mut is_end_of_line = false;
        let char_opt = self.buf.char_opt(self.win_mgr.curt().cur.y, self.win_mgr.curt().cur.x);

        match self.cmd.cmd_type {
            CmdType::CursorRight | CmdType::CursorRightSelect | CmdType::InsertStr(_) if self.win_mgr.curt().sel.mode == SelMode::Normal => {
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
            self.win_mgr.curt().updown_x = 0;
            self.win_mgr.curt().cur.disp_x = 0;
            self.win_mgr.curt().cur.x = 0;
            self.cur_down();
        } else if let Some(c) = char_opt {
            if c == TAB_CHAR {
                let cfg_tab_width = Cfg::get().general.editor.tab.size;
                let tab_width = cfg_tab_width - (self.win_mgr.curt().cur.disp_x % cfg_tab_width);
                self.win_mgr.curt().cur.disp_x += tab_width;
                self.win_mgr.curt().cur.x += 1;
            } else {
                self.win_mgr.curt().cur.disp_x += get_char_width_not_tab(&c);
                self.win_mgr.curt().cur.x = min(self.win_mgr.curt().cur.x + 1, self.buf.len_row_chars(self.win_mgr.curt().cur.y));
                if self.cmd == Cmd::to_cmd(CmdType::CursorRightSelect) && c == NEW_LINE_CR {
                    self.win_mgr.curt().cur.disp_x += 1;
                    self.win_mgr.curt().cur.x += 1;
                }
            }
        } else if self.win_mgr.curt().sel.mode == SelMode::BoxSelect {
            self.win_mgr.curt().cur.disp_x += 1;
            self.win_mgr.curt().cur.x += 1;
        }

        if self.win_mgr.curt().sel.mode == SelMode::Normal {
            // self.scroll();
            self.scroll_horizontal();
        }
        Log::debug("self.win", &self.win_mgr);
    }

    pub fn cur_home(&mut self) {
        self.win_mgr.curt().cur.x = 0;
        self.win_mgr.curt().cur.disp_x = 0;
        self.scroll_horizontal();
    }

    pub fn cur_end(&mut self) {
        self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.buf.len_row_chars(self.win_mgr.curt_ref().cur.y), false);
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn ctrl_home(&mut self) {
        self.win_mgr.curt().updown_x = 0;
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
        if self.win_mgr.curt().updown_x == 0 {
            self.win_mgr.curt().updown_x = self.win_mgr.curt().cur.disp_x;
        }
    }

    pub fn page_down(&mut self) {
        self.win_mgr.curt().cur.y = min(self.win_mgr.curt().cur.y + self.get_curt_row_len(), self.buf.len_rows() - 1);
        self.cur_updown_com();
    }

    pub fn page_up(&mut self) {
        self.win_mgr.curt().cur.y = if self.win_mgr.curt().cur.y > self.get_curt_row_len() { self.win_mgr.curt().cur.y - self.get_curt_row_len() } else { 0 };
        self.cur_updown_com();
    }
    pub fn shift_move_com(&mut self) {
        let cur = self.win_mgr.curt().cur;
        self.win_mgr.curt().sel.set_sel_posi(true, cur);

        match self.cmd.cmd_type {
            CmdType::CursorUpSelect => self.cur_up(),
            CmdType::CursorDownSelect => self.cur_down(),
            CmdType::CursorLeftSelect => self.cur_left(),
            CmdType::CursorRightSelect => self.cur_right(),
            CmdType::CursorRowHomeSelect => self.cur_home(),
            CmdType::CursorRowEndSelect => self.cur_end(),
            _ => {}
        }
        self.scroll();
        let cur = self.win_mgr.curt().cur;
        self.win_mgr.curt().sel.set_sel_posi(false, cur);
        self.win_mgr.curt().sel.check_overlap();
    }
}
