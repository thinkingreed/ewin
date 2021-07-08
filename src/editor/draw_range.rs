use crate::{_cfg::keys::KeyCmd, model::*, sel_range::SelMode, tab::Tab};
use std::{
    cmp::{max, min},
    usize,
};
// The Draw range setting is basically done in the initial processing and the final processing,
// but the detailed case is done in each Event processing.

impl Editor {
    pub fn set_draw_range_init(tab: &mut Tab) {
        // judgment redraw
        tab.editor.d_range.draw_type = DrawType::Not;

        match tab.editor.keycmd {
            KeyCmd::Resize => tab.editor.d_range.draw_type = DrawType::None,
            KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => {
                if tab.editor.sel.mode == SelMode::BoxSelect {
                    tab.editor.d_range = DRange::new(min(tab.editor.sel.sy, tab.editor.sel.ey), max(tab.editor.sel.sy, tab.editor.sel.ey), DrawType::Target);

                    // When moving after overlap
                } else if tab.editor.sel.is_selected() {
                    tab.editor.d_range.draw_type = DrawType::All;
                } else {
                    if tab.editor.keycmd == KeyCmd::CursorDown || tab.editor.keycmd == KeyCmd::CursorUp {
                        let (y, y_after) = tab.editor.get_up_down_draw_range();
                        tab.editor.d_range = DRange::new(min(y, y_after), max(y, y_after), DrawType::Target);
                    } else {
                        tab.editor.d_range.draw_type = DrawType::MoveCur;
                    }
                };
            }
            KeyCmd::MouseDragLeft(_, _) => {
                if tab.editor.sel.is_selected() {
                    tab.editor.d_range.draw_type = DrawType::Target;
                }
            }
            KeyCmd::MouseDragBoxLeft(_, _) => {
                if tab.editor.sel.is_selected() {
                    tab.editor.d_range.draw_type = DrawType::All;
                }
            }
            KeyCmd::MouseScrollDown | KeyCmd::MouseScrollUp => {
                if tab.editor.sel.is_selected() {
                    let sel = tab.editor.sel.get_range();
                    tab.editor.d_range = DRange::new(max(sel.sy, tab.editor.offset_y), sel.ey, DrawType::Target);
                } else {
                    let (y, y_after) = tab.editor.get_up_down_draw_range();
                    tab.editor.d_range = DRange::new(min(y, y_after), max(y, y_after), DrawType::Target);
                }
            }
            _ => tab.editor.d_range.draw_type = DrawType::All,
        };
    }

    pub fn set_draw_range_finalize(&mut self) {
        if self.d_range.draw_type != DrawType::All {
            if self.rnw_org != self.get_rnw() {
                self.d_range.draw_type = DrawType::All;
            } else if (self.offset_x > 0 && self.cur_y_org != self.cur.y) || self.offset_x_org != self.offset_x {
                //  self.d_range = DRange::new(min(self.cur_y_org, self.cur.y), max(self.cur_y_org, self.cur.y), DrawType::Target);
                // For undo, redo
                self.d_range.draw_type = DrawType::All;
            } else if self.offset_y_org != self.offset_y {
                match self.keycmd {
                    KeyCmd::CursorUp | KeyCmd::MouseScrollUp => self.set_draw_range_scroll(self.offset_y, DrawType::ScrollUp),
                    KeyCmd::CursorDown | KeyCmd::MouseScrollDown => self.set_draw_range_scroll(self.offset_y + self.disp_row_num - 1, DrawType::ScrollDown),
                    _ => self.d_range.draw_type = DrawType::All,
                }
            }
        }
    }

    pub fn set_draw_range_scroll(&mut self, y: usize, draw_type: DrawType) {
        if draw_type == DrawType::ScrollDown {
            self.d_range = DRange::new(y - Editor::UP_DOWN_EXTRA - 1, y, draw_type);
        } else {
            self.d_range = DRange::new(y, y + Editor::UP_DOWN_EXTRA + 1, draw_type);
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
}
