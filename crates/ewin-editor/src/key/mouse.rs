use crate::{
    ewin_com::{_cfg::key::keys::*, model::*, util::*},
    model::*,
};
use ewin_cfg::{log::*, model::default::*};
use ewin_com::_cfg::key::cmd::CmdType;
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
        Log::debug("self.row_posi + self.row_len", &(self.row_posi + self.row_len));
        Log::debug("self.row_posi", &self.row_posi);
        Log::debug("self.scrl_v.is_show", &self.scrl_v.is_show);
        Log::debug("self.scrl_v.is_enable", &self.scrl_v.is_enable);

        self.sel.mode = match self.cmd.cmd_type {
            CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => SelMode::BoxSelect,
            _ => SelMode::Normal,
        };

        // scrlbar_v
        if self.scrl_v.is_show && self.row_posi <= y && y <= self.row_posi + self.row_len {
            match self.cmd.cmd_type {
                CmdType::MouseDownLeft(y, x) if self.get_rnw_and_margin() + self.col_len <= x => {
                    self.set_scrlbar_v_posi(y);
                }
                CmdType::MouseDragLeftDown(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) if self.scrl_v.is_enable => {
                    if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                        self.set_scrlbar_v_posi(y);
                    }
                }
                _ => self.scrl_v.is_enable = false,
            };
        }

        Log::debug("self.sel 111", &self.sel);

        // scrlbar_h
        let height = Cfg::get().general.editor.scrollbar.horizontal.height;
        match self.cmd.cmd_type {
            CmdType::MouseDownLeft(_, x) if self.scrl_h.row_posi <= y && y < self.scrl_h.row_posi + height => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            CmdType::MouseDragLeftDown(_, x) | CmdType::MouseDragLeftUp(_, x) | CmdType::MouseDragLeftLeft(_, x) | CmdType::MouseDragLeftRight(_, x) if self.scrl_h.is_enable => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            _ => self.scrl_h.is_enable = false,
        };

        Log::debug("self.sel 222", &self.sel);
        Log::debug("self.scrl_v.is_enable", &self.scrl_v.is_enable);
        Log::debug("self.scrl_h.is_enable", &self.scrl_h.is_enable);

        Log::debug("self.offset_y", &self.offset_y);

        if !self.scrl_v.is_enable && !self.scrl_h.is_enable {
            if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) && x < self.get_rnw_and_margin() - 1 {
                self.sel.set_s(y, 0, 0);
                let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(y)[..], 0, true);
                self.sel.set_e(y, cur_x, width);
                self.set_cur_target_by_x(y, 0, false);
            } else {
                // y, range check
                if y == 0 {
                    if self.cur.y > 0 {
                        y = self.cur.y - 1;
                    }
                } else if y + self.offset_y <= self.row_posi {
                    y = 0;
                } else if self.buf.len_rows() < y + self.offset_y - self.row_posi {
                    y = self.buf.len_rows() - 1;
                } else if get_term_size().1 == y {
                    y = self.offset_y + self.row_len;
                } else {
                    if y >= self.row_posi {
                        y -= self.row_posi;
                    }
                    y = min(y + self.offset_y, self.buf.len_rows() - 1)
                }

                x = if x < self.get_rnw_and_margin() { 0 } else { x - self.get_rnw_and_margin() };
                self.cur.y = y;
                let vec = self.buf.char_vec_row(self.cur.y);
                if self.sel.mode == SelMode::BoxSelect && self.offset_x + x > vec.len() - 1 {
                    self.cur.x = x;
                    self.cur.disp_x = x;
                } else {
                    self.set_cur_target_by_disp_x(y, x);
                    self.scroll();
                    self.scroll_horizontal();

                    if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) {
                        self.sel.clear();
                    }
                }
                self.history.set_sel_multi_click(&keys, &mut self.sel, &self.cur, &self.cur_org, &self.buf.char_vec_row(self.cur.y));
            }
        };
    }
}
