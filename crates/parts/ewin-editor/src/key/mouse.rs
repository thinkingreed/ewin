use crate::{ewin_key::key::cmd::*, model::*};
use ewin_cfg::log::*;
use ewin_const::term::*;
use ewin_key::sel_range::*;
use ewin_state::term::*;
use ewin_utils::char_edit::*;
use std::cmp::min;

impl Editor {
    pub fn ctrl_mouse(&mut self, y: usize, x: usize) {
        Log::debug_key("Editor.ctrl_mouse");

        let (mut y, mut x) = (y, x);

        match self.cmd.cmd_type {
            CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) => {
                State::get().curt_mut_state().editor.is_dragging = true;
            }
            _ => {}
        };

        Log::debug("yyy", &y);
        Log::debug("xxx", &x);
        Log::debug("self.keys", &self.keys);

        self.win_mgr.curt_mut().sel.mode = match self.cmd.cmd_type {
            CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => SelMode::BoxSelect,
            _ => SelMode::Normal,
        };

        if !self.win_mgr.curt_mut().scrl_v.is_enable && !self.win_mgr.curt_mut().scrl_h.is_enable {
            if y == 0 {
                if self.win_mgr.curt_mut().cur.y > 0 {
                    y = self.win_mgr.curt_mut().cur.y - 1;
                }
            } else if y + self.win_mgr.curt_mut().offset.y <= self.get_curt_row_posi() {
                y = 0;
            } else if self.buf.len_rows() < y + self.win_mgr.curt_mut().offset.y - self.get_curt_row_posi() {
                y = self.buf.len_rows() - 1;
            } else if get_term_size().1 == y {
                y = self.win_mgr.curt_mut().offset.y + self.get_curt_row_len();
            } else {
                if y >= self.get_curt_row_posi() {
                    y -= self.get_curt_row_posi();
                }
                y = min(y + self.win_mgr.curt_mut().offset.y, self.buf.len_rows() - 1)
            }

            x = if x < self.win_mgr.curt_mut().view.x { self.win_mgr.curt_mut().view.x } else { x - self.win_mgr.curt_mut().view.x };

            Log::debug("Adjusted yyy", &y);
            Log::debug("Adjusted xxx", &x);

            if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) && self.win_mgr.curt_ref().view_all.x <= x && x < self.win_mgr.curt_ref().view.x + self.get_rnw_and_margin() - 1 {
                self.win_mgr.curt_mut().sel.set_s(y, 0, 0);
                let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(y)[..], 0, true);
                self.win_mgr.curt_mut().sel.set_e(y, cur_x, width);
                self.set_cur_target_by_x(y, 0, false);
            } else {
                self.win_mgr.curt_mut().cur.y = y;
                let vec = self.buf.char_vec_row(self.win_mgr.curt_mut().cur.y);
                if self.win_mgr.curt_mut().sel.mode == SelMode::BoxSelect && self.win_mgr.curt_mut().offset.x + x > vec.len() - 1 {
                    self.win_mgr.curt_mut().cur.x = x;
                    self.win_mgr.curt_mut().cur.disp_x = x;
                } else {
                    self.set_cur_target_by_disp_x(y, x);
                }
                let cur = &self.win_mgr.curt_mut().cur.clone();
                let cur_org = &self.win_mgr.curt_mut().cur_org.clone();
                let row = &self.buf.char_vec_row(self.win_mgr.curt_mut().cur.y);
                self.history.set_sel_multi_click(&self.keys, &mut self.win_mgr.curt_mut().sel, cur, cur_org, row);
            }
        }
        self.scroll();
        self.scroll_horizontal();
    }
}
