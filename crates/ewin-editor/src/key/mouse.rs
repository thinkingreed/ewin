use ewin_com::_cfg::model::default::Cfg;

use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        def::*,
        log::*,
        model::*,
        util::*,
    },
    model::*,
};

impl Editor {
    pub fn ctrl_mouse(&mut self) {
        Log::debug_key("ctrl_mouse");

        let (mut y, mut x, keys) = match self.e_cmd {
            E_Cmd::MouseDownLeft(y, x) => (y, x, Keys::MouseDownLeft(y as u16, x as u16)),
            E_Cmd::MouseUpLeft(_, _) => {
                self.state.is_dragging = false;
                return;
            }
            E_Cmd::MouseDragLeftUp(y, x) | E_Cmd::MouseDragLeftDown(y, x) | E_Cmd::MouseDragLeftLeft(y, x) | E_Cmd::MouseDragLeftRight(y, x) => {
                self.state.is_dragging = true;
                (y, x, Keys::MouseDragLeft(y as u16, x as u16))
            }
            E_Cmd::MouseDownLeftBox(y, x) => (y, x, Keys::MouseDownLeft(y as u16, x as u16)),
            E_Cmd::MouseDragLeftBox(y, x) => (y, x, Keys::MouseDragLeft(y as u16, x as u16)),
            _ => return,
        };
        Log::debug("y", &y);
        Log::debug("x", &x);
        Log::debug("self.sel 000", &self.sel);

        self.sel.mode = match self.e_cmd {
            E_Cmd::MouseDownLeftBox(_, _) | E_Cmd::MouseDragLeftBox(_, _) => SelMode::BoxSelect,
            _ => SelMode::Normal,
        };

        // scrlbar_v
        if self.scrl_v.is_show && self.row_posi <= y && y <= self.row_posi + self.row_disp_len {
            match self.e_cmd {
                E_Cmd::MouseDownLeft(y, x) if self.get_rnw_and_margin() + self.col_len <= x => {
                    self.set_scrlbar_v_posi(y);
                }
                E_Cmd::MouseDragLeftDown(y, _) | E_Cmd::MouseDragLeftUp(y, _) | E_Cmd::MouseDragLeftLeft(y, _) | E_Cmd::MouseDragLeftRight(y, _) if self.scrl_v.is_enable => {
                    if matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) {
                        self.set_scrlbar_v_posi(y);
                    }
                }
                _ => self.scrl_v.is_enable = false,
            };
        }

        Log::debug("self.sel 111", &self.sel);

        // scrlbar_h
        let height = Cfg::get().general.editor.scrollbar.horizontal.height;
        match self.e_cmd {
            E_Cmd::MouseDownLeft(_, x) if self.scrl_h.row_posi <= y && y < self.scrl_h.row_posi + height => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            E_Cmd::MouseDragLeftDown(_, x) | E_Cmd::MouseDragLeftUp(_, x) | E_Cmd::MouseDragLeftLeft(_, x) | E_Cmd::MouseDragLeftRight(_, x) if self.scrl_h.is_enable => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            _ => self.scrl_h.is_enable = false,
        };

        Log::debug("self.sel 222", &self.sel);

        if !self.scrl_v.is_enable && !self.scrl_h.is_enable {
            // y, range check
            if self.buf.len_rows() < y || HEADERBAR_ROW_NUM + self.row_disp_len - 1 + STATUSBAR_ROW_NUM == y {
                // In case of MouseMode::Mouse, this function is not executed, so ignore it.
                if self.buf.len_rows() < y {
                    y = self.buf.len_rows() - 1;
                } else if HEADERBAR_ROW_NUM + self.row_disp_len - 1 + STATUSBAR_ROW_NUM == y {
                    y = self.offset_y + self.row_disp_len - 1;
                }
            } else {
                if y >= HEADERBAR_ROW_NUM {
                    y -= HEADERBAR_ROW_NUM;
                }
                y += self.offset_y
            }

            if matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) && x < self.get_rnw_and_margin() - 1 {
                self.sel.set_s(y, 0, 0);
                let (cur_x, width) = get_row_cur_x_disp_x(&self.buf.char_vec_row(y)[..], 0, true);
                self.sel.set_e(y, cur_x, width);
                self.set_cur_target_by_x(y, 0, false);
            } else if self.buf.len_rows() > y {
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

                    if matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) {
                        self.sel.clear();
                    }
                }
                self.history.set_sel_multi_click(&keys, &mut self.sel, &self.cur, &self.cur_org, &self.buf.char_vec_row(self.cur.y));
            }
        }
    }
}
