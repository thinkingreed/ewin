use crate::{
    ewin_key::{
        key::{cmd::*, keys::*},
        util::*,
    },
    model::*,
};
use ewin_cfg::{log::*, model::default::*};
use ewin_const::term::*;
use ewin_view::sel_range::*;
use std::cmp::min;

impl Editor {
    pub fn ctrl_mouse(&mut self) {
        Log::debug_key("ctrl_mouse");

        let (mut y, mut x, keys) = match self.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => (y, x, Keys::MouseDownLeft(y as u16, x as u16)),
            CmdType::MouseUpLeft(_, _) => {
                self.state.is_dragging = false;
                return;
            }
            CmdType::MouseDragLeftUp(y, x) | CmdType::MouseDragLeftDown(y, x) | CmdType::MouseDragLeftLeft(y, x) | CmdType::MouseDragLeftRight(y, x) => {
                self.state.is_dragging = true;
                (y, x, Keys::MouseDragLeft(y as u16, x as u16))
            }
            CmdType::MouseDownLeftBox(y, x) => (y, x, Keys::MouseDownLeft(y as u16, x as u16)),
            CmdType::MouseDragLeftBox(y, x) => (y, x, Keys::MouseDragLeft(y as u16, x as u16)),
            _ => return,
        };
        Log::debug("y 111", &y);
        Log::debug("x", &x);
        Log::debug("self.row_posi + self.row_len", &(self.get_curt_row_posi() + self.get_curt_row_len()));
        Log::debug("self.row_posi", &self.get_curt_row_posi());
        Log::debug("self.scrl_v.is_show", &self.win_mgr.curt().scrl_v.is_show);
        Log::debug("self.scrl_v.is_enable", &self.win_mgr.curt().scrl_v.is_enable);

        self.win_mgr.curt().sel.mode = match self.cmd.cmd_type {
            CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => SelMode::BoxSelect,
            _ => SelMode::Normal,
        };

        // scrlbar_v
        if self.win_mgr.curt().scrl_v.is_show && self.get_curt_row_posi() <= y && y <= self.get_curt_row_posi() + self.get_curt_row_len() {
            match self.cmd.cmd_type {
                CmdType::MouseDownLeft(y, x) if self.win_mgr.curt().area_h.1 <= x => {
                    self.set_scrlbar_v_posi(y);
                }
                CmdType::MouseDragLeftDown(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) if self.win_mgr.curt().scrl_v.is_enable => {
                    if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                        self.set_scrlbar_v_posi(y);
                    }
                }
                _ => self.win_mgr.curt().scrl_v.is_enable = false,
            };
        }

        Log::debug("self.sel 111", &self.win_mgr.curt().sel);

        // scrlbar_h
        let height = Cfg::get().general.editor.scrollbar.horizontal.height;
        match self.cmd.cmd_type {
            CmdType::MouseDownLeft(_, x) if self.win_mgr.curt().scrl_h.row_posi <= y && y < self.win_mgr.curt().scrl_h.row_posi + height => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            CmdType::MouseDragLeftDown(_, x) | CmdType::MouseDragLeftUp(_, x) | CmdType::MouseDragLeftLeft(_, x) | CmdType::MouseDragLeftRight(_, x) if self.win_mgr.curt().scrl_h.is_enable => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            _ => self.win_mgr.curt().scrl_h.is_enable = false,
        };

        Log::debug("self.sel 222", &self.win_mgr.curt().sel);
        Log::debug("self.scrl_v.is_enable", &self.win_mgr.curt().scrl_v.is_enable);
        Log::debug("self.scrl_h.is_enable", &self.win_mgr.curt().scrl_h.is_enable);

        Log::debug("self.offset.y", &self.win_mgr.curt().offset.y);

        if !self.win_mgr.curt().scrl_v.is_enable && !self.win_mgr.curt().scrl_h.is_enable {
            if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) && x < self.get_rnw_and_margin() - 1 {
                self.win_mgr.curt().sel.set_s(y, 0, 0);
                let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(y)[..], 0, true);
                self.win_mgr.curt().sel.set_e(y, cur_x, width);
                self.set_cur_target_by_x(y, 0, false);
            } else {
                // y, range check
                if y == 0 {
                    if self.win_mgr.curt().cur.y > 0 {
                        y = self.win_mgr.curt().cur.y - 1;
                    }
                } else if y + self.win_mgr.curt().offset.y <= self.get_curt_row_posi() {
                    y = 0;
                } else if self.buf.len_rows() < y + self.win_mgr.curt().offset.y - self.get_curt_row_posi() {
                    y = self.buf.len_rows() - 1;
                } else if get_term_size().1 == y {
                    y = self.win_mgr.curt().offset.y + self.get_curt_row_len();
                } else {
                    if y >= self.get_curt_row_posi() {
                        y -= self.get_curt_row_posi();
                    }
                    y = min(y + self.win_mgr.curt().offset.y, self.buf.len_rows() - 1)
                }

                Log::debug("self.win_mgr.curt()", &self.win_mgr.curt());

                x = if x < self.get_rnw_and_margin() { 0 } else { x - self.win_mgr.curt().area_h.0 };
                self.win_mgr.curt().cur.y = y;
                let vec = self.buf.char_vec_row(self.win_mgr.curt().cur.y);
                if self.win_mgr.curt().sel.mode == SelMode::BoxSelect && self.win_mgr.curt().offset.x + x > vec.len() - 1 {
                    self.win_mgr.curt().cur.x = x;
                    self.win_mgr.curt().cur.disp_x = x;
                } else {
                    self.set_cur_target_by_disp_x(y, x);
                    self.scroll();
                    self.scroll_horizontal();

                    if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) {
                        self.win_mgr.curt().sel.clear();
                    }
                }
                let cur = &self.win_mgr.curt().cur.clone();
                let cur_org = &self.win_mgr.curt().cur_org.clone();
                let row = &self.buf.char_vec_row(self.win_mgr.curt().cur.y);
                self.history.set_sel_multi_click(&keys, &mut self.win_mgr.curt().sel, cur, cur_org, row);
            }
        };
    }
}
