use crate::{ewin_core::_cfg::keys::KeyCmd, ewin_core::model::*, model::*};
use std::{
    cmp::{max, min},
    usize,
};
// The Draw range setting is basically done in the initial processing and the final processing,
// but the detailed case is done in each Event processing.

impl Editor {
    pub fn set_draw_range_init(&mut self) {
        // judgment redraw
        self.draw_type = DrawType::Not;

        match self.keycmd {
            KeyCmd::Resize => self.draw_type = DrawType::None,
            KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => {
                if self.sel.mode == SelMode::BoxSelect {
                    self.draw_type = DrawType::Target(min(self.sel.sy, self.sel.ey), max(self.sel.sy, self.sel.ey));

                    // When moving after overlap
                } else if self.sel.is_selected() {
                    self.draw_type = DrawType::All;
                } else if self.keycmd == KeyCmd::CursorDown || self.keycmd == KeyCmd::CursorUp {
                    let (y, y_after) = self.get_up_down_draw_range();
                    self.draw_type = DrawType::Target(min(y, y_after), max(y, y_after));
                } else {
                    self.draw_type = DrawType::MoveCur;
                }
            }
            KeyCmd::MouseDragLeft(y, _) => {
                if self.sel.is_selected() {
                    self.draw_type = DrawType::Target(y, y);
                }
            }
            KeyCmd::MouseDragBoxLeft(_, _) => {
                if self.sel.is_selected() {
                    self.draw_type = DrawType::All;
                }
            }
            KeyCmd::MouseScrollDown | KeyCmd::MouseScrollUp => {
                if self.sel.is_selected() {
                    let sel = self.sel.get_range();
                    self.draw_type = DrawType::Target(max(sel.sy, self.offset_y), sel.ey);
                } else {
                    let (y, y_after) = self.get_up_down_draw_range();
                    self.draw_type = DrawType::Target(min(y, y_after), max(y, y_after));
                }
            }
            KeyCmd::MouseDownRight(_, _) | KeyCmd::MouseDragRight(_, _) => self.draw_type = DrawType::All,
            _ => self.draw_type = DrawType::All,
        };
    }

    pub fn set_draw_range_finalize(&mut self) {
        if self.draw_type != DrawType::All {
            if self.rnw_org != self.get_rnw() {
                self.draw_type = DrawType::All;
            } else if (self.offset_x > 0 && self.cur_y_org != self.cur.y) || self.offset_x_org != self.offset_x {
                //  self.d_range = DRange::new(min(self.cur_y_org, self.cur.y), max(self.cur_y_org, self.cur.y), DrawType::Target);
                // For undo, redo
                self.draw_type = DrawType::All;
            } else if self.offset_y_org != self.offset_y {
                match self.keycmd {
                    KeyCmd::CursorUp | KeyCmd::MouseScrollUp => self.draw_type = DrawType::ScrollUp(self.offset_y, self.offset_y + Editor::UP_DOWN_EXTRA + 1),
                    KeyCmd::CursorDown | KeyCmd::MouseScrollDown => {
                        let y = self.offset_y + self.disp_row_num - 1;
                        self.draw_type = DrawType::ScrollDown(y - Editor::UP_DOWN_EXTRA - 1, y);
                    }
                    _ => self.draw_type = DrawType::All,
                }
            }
        }
    }

    pub fn get_up_down_draw_range(&mut self) -> (usize, usize) {
        let y = self.cur.y;

        let y_after = match self.keycmd {
            KeyCmd::CursorDown | KeyCmd::MouseScrollDown => min(y + 1, self.buf.len_lines() - 1),
            // UP・ScrollUp
            _ => {
                if y == 0 {
                    0
                } else {
                    y - 1
                }
            }
        };

        return (y, y_after);
    }

    pub fn set_draw_range_each_process(&mut self, draw_type: DrawType) {
        if self.is_enable_syntax_highlight {
            self.draw_type = DrawType::All;
        } else if self.sel.is_selected() {
            let sel = self.sel.get_range();
            self.draw_type = DrawType::After(sel.sy);
        } else {
            self.draw_type = draw_type;
        }
    }
}
