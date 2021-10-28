use crate::{
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*, util::*},
    model::*,
};
use std::cmp::{max, min};

impl Editor {
    pub fn ctrl_mouse(&mut self) {
        Log::debug_key("ctrl_mouse");

        let (mut y, mut x, mouse_proc) = match self.e_cmd {
            E_Cmd::MouseDownLeft(y, x) => (y, x, MouseProc::DownLeft),
            E_Cmd::MouseDragLeftUp(y, x) | E_Cmd::MouseDragLeftDown(y, x) | E_Cmd::MouseDragLeftLeft(y, x) | E_Cmd::MouseDragLeftRight(y, x) => (y, x, MouseProc::DragLeft),
            E_Cmd::MouseDownBoxLeft(y, x) => (y, x, MouseProc::DownLeftBox),
            E_Cmd::MouseDragBoxLeft(y, x) => (y, x, MouseProc::DragLeftBox),
            _ => return,
        };
        Log::debug("y", &y);
        Log::debug("x", &x);

        self.sel.mode = match mouse_proc {
            MouseProc::DownLeftBox | MouseProc::DragLeftBox => SelMode::BoxSelect,
            MouseProc::DownLeft | MouseProc::DragLeft => SelMode::Normal,
        };

        // y, range check
        if self.buf.len_lines() < y || HEADERBAR_ROW_NUM + self.row_num - 1 + STATUSBAR_ROW_NUM == y {
            // In case of MouseMode::Mouse, this function is not executed, so ignore it.
            if self.buf.len_lines() < y {
                y = self.buf.len_lines() - 1;
            } else if HEADERBAR_ROW_NUM + self.row_num - 1 + STATUSBAR_ROW_NUM == y {
                y = self.offset_y + self.row_num - 1;
            }
        } else {
            if y >= HEADERBAR_ROW_NUM {
                y -= HEADERBAR_ROW_NUM;
            }
            y += self.offset_y
        }

        if mouse_proc == MouseProc::DownLeft && x < self.get_rnw_and_margin() {
            self.sel.set_s(y, 0, 0);
            let (cur_x, width) = get_row_width(&self.buf.char_vec_line(y)[..], 0, true);
            self.sel.set_e(y, cur_x, width);
            self.set_cur_target(y, 0, false);
        } else {
            x = if x < self.get_rnw_and_margin() { 0 } else { x - self.get_rnw_and_margin() };
            self.cur.y = y;
            let vec = self.buf.char_vec_line(self.cur.y);

            if self.sel.mode == SelMode::BoxSelect && self.offset_x + x > vec.len() - 1 {
                self.cur.x = x;
                self.cur.disp_x = x;
            } else {
                let (cur_x, width) = get_until_x(&vec, x + self.offset_disp_x);
                self.cur.x = cur_x;
                self.cur.disp_x = width;
                self.scroll();
                self.scroll_horizontal();
            }
            self.history.set_sel_multi_click(mouse_proc, &mut self.sel, &self.cur, &self.buf.char_vec_line(self.cur.y), &self.keys);

            if self.sel.is_selected() {
                match mouse_proc {
                    MouseProc::DownLeft | MouseProc::DownLeftBox | MouseProc::DragLeftBox => self.draw_range = EditorDrawRange::All,
                    MouseProc::DragLeft => {
                        if self.sel.mode == SelMode::Normal {
                            self.draw_range = EditorDrawRange::Target(min(self.cur.y, self.cur_y_org), max(self.cur.y, self.cur_y_org));
                        } else {
                            self.draw_range = EditorDrawRange::All;
                        }
                    }
                }
            }
        }
    }
}
