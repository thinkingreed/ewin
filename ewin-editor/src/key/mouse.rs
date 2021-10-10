use crate::{
    ewin_core::{_cfg::key::keycmd::*, def::*, log::*, model::*, util::*},
    model::*,
};
use std::cmp::{max, min};

impl Editor {
    pub fn ctrl_mouse(&mut self) {
        Log::debug_key("ctrl_mouse");
        let (mut y, mut x, mouse_proc) = match self.e_cmd {
            E_Cmd::MouseDownLeft(y, x) => (y, x, MouseProc::DownLeft),
            E_Cmd::MouseDragLeft(y, x) => (y, x, MouseProc::DragLeft),
            E_Cmd::MouseDownBoxLeft(y, x) => (y, x, MouseProc::DownLeftBox),
            E_Cmd::MouseDragBoxLeft(y, x) => (y, x, MouseProc::DragLeftBox),
            _ => return,
        };
        if mouse_proc == MouseProc::DownLeftBox || mouse_proc == MouseProc::DragLeftBox {
            self.sel.mode = SelMode::BoxSelect;
        } else if mouse_proc == MouseProc::DownLeft {
            self.sel.mode = SelMode::Normal;
        }
        // y, x range check
        if y < self.row_posi || self.row_num < y || self.buf.len_lines() < y {
            let mut set_y = 0;
            // In case of MouseMode::Mouse, this function is not executed, so ignore it.
            if self.buf.len_lines() < y || self.row_num < y {
                set_y = self.buf.len_lines() - 1;
            } else if y < self.row_posi {
                set_y = 0;
            }
            let set_x = get_until_x(&self.buf.char_vec_line(set_y), if x > self.get_rnw_and_margin() { x - self.get_rnw_and_margin() } else { 0 }).0;
            self.set_cur_target(set_y, set_x, false);
            y = set_y;

            self.draw_range = EditorDrawRange::All;
            if mouse_proc != MouseProc::DragLeft {
                self.sel.clear();
            }
        } else {
            y = y - self.row_posi;
        }

        if mouse_proc == MouseProc::DownLeft && x < self.get_rnw_and_margin() {
            self.sel.set_s(y, 0, 0);
            let (cur_x, width) = get_row_width(&self.buf.char_vec_line(y)[..], 0, true);
            self.sel.set_e(y, cur_x, width);
            self.set_cur_target(y + self.offset_y, 0, false);
            self.draw_range = EditorDrawRange::All;
        } else {
            if x < self.get_rnw_and_margin() {
                x = self.get_rnw_and_margin();
            }
            let x = x - self.get_rnw_and_margin();
            self.cur.y = y + self.offset_y;
            let vec = self.buf.char_vec_line(self.cur.y);

            if self.sel.mode == SelMode::BoxSelect && self.offset_x + x > vec.len() - 1 {
                self.cur.x = x;
                self.cur.disp_x = x;
            } else {
                let (cur_x, width) = get_until_x(&vec, x + self.offset_x);
                self.cur.x = cur_x;
                self.cur.disp_x = width;
                self.scroll_horizontal();
            }
            self.history.set_sel_multi_click(mouse_proc, &mut self.sel, &self.cur, &self.buf.char_vec_line(self.cur.y), &self.keys);

            if self.sel.is_selected() {
                match mouse_proc {
                    MouseProc::DownLeft | MouseProc::DownLeftBox | MouseProc::DragLeftBox => self.draw_range = EditorDrawRange::All,
                    MouseProc::DragLeft => {
                        if self.sel.mode == SelMode::Normal {
                            let sel = self.sel.get_range();
                            let sel_org = self.sel_org.get_range();
                            // Drag from outside the range
                            let sy = min(sel.sy, sel_org.sy);
                            let mut ey = max(sel.ey, if sel_org.ey == USIZE_UNDEFINED { sel.ey } else { sel_org.ey });
                            if ey == USIZE_UNDEFINED {
                                ey = sy;
                            }
                            self.draw_range = EditorDrawRange::Target(sy, ey);
                        } else {
                            self.draw_range = EditorDrawRange::All;
                        }
                    }
                }
            }
        }
    }
}
