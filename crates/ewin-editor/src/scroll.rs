use ewin_com::def::USIZE_UNDEFINED;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*, util::*},
    model::*,
};
use std::{
    cmp::{max, min},
    usize,
};

impl Editor {
    pub const SCROLL_UP_DOWN_MARGIN: usize = 1;
    const OFFSET_Y_MARGIN: usize = 3;
    const DISP_ROWS_MARGIN: usize = 3;
    const SCROLL_LEFT_RIGHT_JUDGE_MARGIN: usize = 3;
    const SCROLL_MOUSE_DRAG_LEFT_JUDGE_MARGIN: usize = 8;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.cur.y", &self.cur.y);
        Log::debug("self.e_cmd", &self.e_cmd);
        Log::debug("self.buf.len_rows()", &self.buf.len_rows());
        Log::debug("self.offset_y before", &self.offset_y);

        if self.cur.y == 0 && self.is_move_cur_posi_scrolling_enable() {
            self.offset_y = 0
        } else if !self.is_move_cur_posi_scrolling_enable() && (matches!(self.e_cmd, E_Cmd::MouseScrollDown) || matches!(self.e_cmd, E_Cmd::MouseScrollUp)) {
            if matches!(self.e_cmd, E_Cmd::MouseScrollDown) {
                if self.get_disp_rows() > self.row_disp_len {
                    self.offset_y = min(self.offset_y + 1, self.get_disp_rows() - self.row_disp_len);
                }
            } else {
                self.offset_y = if self.offset_y == 0 { 0 } else { self.offset_y - 1 };
            }
        } else if !self.is_move_cur_posi_scrolling_enable() && self.scrl_v.is_enable && (matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _))) {
            self.offset_y = min(self.scrl_v.row_posi * self.scrl_v.move_len, self.get_disp_rows() - self.row_disp_len);

            // When cursor is off the screen
        } else if !self.is_cur_y_in_screen() {
            self.set_offset_move_row();
        } else {
            match &self.e_cmd {
                // When multi rows are deleted
                E_Cmd::CursorFileHome => self.offset_y = 0,
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::ReOpenFile if self.offset_y > self.cur.y => {
                    self.offset_y = if self.row_disp_len >= self.get_disp_rows() - 1 {
                        0
                    } else if self.cur.y < Editor::OFFSET_Y_MARGIN + self.offset_y && self.cur.y >= Editor::OFFSET_Y_MARGIN {
                        self.cur.y - Editor::OFFSET_Y_MARGIN
                    } else {
                        self.offset_y
                    }
                }
                E_Cmd::ReplaceExec(_, _, _) | E_Cmd::GrepResult | E_Cmd::CursorDown | E_Cmd::CursorDownSelect | E_Cmd::MouseScrollDown | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::CursorFileEnd | E_Cmd::InsertStr(_) | E_Cmd::InsertRow | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack => {
                    if self.cur.y + Editor::SCROLL_UP_DOWN_MARGIN >= self.row_disp_len {
                        // "self.buf.len_lines() - self.row_num" is For the last row
                        self.offset_y = max(self.offset_y, min(self.get_disp_rows() - self.row_disp_len, self.cur.y + 1 + Editor::SCROLL_UP_DOWN_MARGIN - self.row_disp_len));
                    }
                }
                E_Cmd::MoveRow => self.set_offset_move_row(),
                E_Cmd::MouseDragLeftLeft(y, _) | E_Cmd::MouseDragLeftRight(y, _) => {
                    if self.sel.is_selected() {
                        if *y >= self.row_posi + self.row_disp_len && self.get_disp_rows() - self.offset_y > self.row_disp_len {
                            self.offset_y += Editor::SCROLL_UP_DOWN_MARGIN;
                        } else if *y < self.row_posi && self.offset_y >= Editor::SCROLL_UP_DOWN_MARGIN {
                            self.offset_y -= Editor::SCROLL_UP_DOWN_MARGIN;
                        }
                    }
                }
                E_Cmd::CursorUp | E_Cmd::MouseScrollUp | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::CursorUpSelect => {
                    // -1 is to maintain consistency with the 0 standard of cur.y
                    if self.offset_y >= Editor::SCROLL_UP_DOWN_MARGIN && self.cur.y < Editor::SCROLL_UP_DOWN_MARGIN + self.offset_y {
                        self.offset_y -= Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                E_Cmd::CursorPageUp => self.offset_y = if self.offset_y >= self.row_disp_len { self.offset_y - self.row_disp_len } else { 0 },
                E_Cmd::CursorPageDown => self.offset_y = if self.get_disp_rows() - 1 > self.offset_y + self.row_disp_len * 2 { self.offset_y + self.row_disp_len } else { self.get_disp_rows() - self.row_disp_len },
                _ => {}
            }
        }
        Log::debug("self.offset_y after", &self.offset_y);
    }
    pub fn get_disp_rows(&self) -> usize {
        if self.scrl_v.bar_len != USIZE_UNDEFINED && self.scrl_v.bar_len > 0 {
            self.row_disp_len + (self.row_disp_len - self.scrl_v.bar_len) * self.scrl_v.move_len
        } else {
            return self.buf.len_rows() + Editor::DISP_ROWS_MARGIN;
        }
    }
    pub fn set_offset_move_row(&mut self) {
        Log::debug_key("set_offset_move_row");

        if self.cur.y >= self.offset_y + self.row_disp_len {
            // last page
            self.offset_y = self.cur.y + Editor::MOVE_ROW_EXTRA_NUM - self.row_disp_len;
            //     self.offset_y = if self.get_disp_rows() - 1 - self.cur.y < self.row_disp_len { self.get_disp_rows() - self.row_disp_len } else { self.cur.y - Editor::MOVE_ROW_EXTRA_NUM }
        } else if self.cur.y < self.offset_y {
            self.offset_y = if self.cur.y > Editor::MOVE_ROW_EXTRA_NUM { self.cur.y - Editor::MOVE_ROW_EXTRA_NUM } else { 0 };
        }
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_key("scroll_horizontal");
        Log::debug("self.cur", &self.cur);
        Log::debug("self.e_cmd", &self.e_cmd);
        Log::debug("self.offset_x 111", &self.offset_x);

        // offset_x Number of characters for switching judgment
        const X_MARGIN: usize = 3;
        //  const SEARCH_JUDGE_COLUMN_MARGIN: usize = 5;
        // Number of offset increase / decrease when switching left / right offset
        const SCROLL_ADD_MARGIN: usize = 10;

        // self.offset_x_org = self.offset_x;
        let vec = &self.buf.char_vec_row(self.cur.y);

        //// Calc offset_x
        // Up・Down・Home ...
        if 0 == self.cur.x {
            self.offset_x = 0;
            self.offset_disp_x = 0;
            return;
        } else if self.sel.mode == SelMode::BoxSelect && self.cur_org.y != self.cur.y {
            self.offset_x = get_until_disp_x(vec, self.offset_disp_x, false).0;
            return;
        } else if self.cur_org.y != self.cur.y {
            if !self.is_cur_disp_x_in_screen() {
                self.offset_x = get_x_offset(&self.buf.char_vec_range(self.cur.y, ..self.cur.x), self.col_len - X_MARGIN);
            }
        } else {
            match &self.e_cmd {
                E_Cmd::ReplaceExec(_, _, _) | E_Cmd::CursorFileEnd | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::InsertStr(_) | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::ReOpenFile | E_Cmd::Null => {
                    self.offset_x = get_x_offset(&self.buf.char_vec_range(self.cur.y, ..self.cur.x), self.col_len - X_MARGIN);
                }
                E_Cmd::CursorRight | E_Cmd::CursorRightSelect | E_Cmd::MouseDragLeftRight(_, _) if self.sel.mode != SelMode::BoxSelect => {
                    let judge_margin = if let E_Cmd::MouseDragLeftRight(_, _) = self.e_cmd { Editor::SCROLL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.offset_disp_x + self.col_len < self.cur.disp_x + judge_margin {
                        // Judgment whether the end fits in the width
                        let width = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.col_len {
                            self.offset_x += SCROLL_ADD_MARGIN;
                        }
                    }
                }
                E_Cmd::CursorLeft | E_Cmd::CursorLeftSelect | E_Cmd::MouseDragLeftLeft(_, _) => {
                    let judge_margin = if let E_Cmd::MouseDragLeftLeft(_, _) = self.e_cmd { Editor::SCROLL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.cur.disp_x - self.offset_disp_x <= judge_margin {
                        self.offset_x = if self.offset_x >= SCROLL_ADD_MARGIN { self.offset_x - SCROLL_ADD_MARGIN } else { 0 };
                    }
                }
                _ => {}
            }
        }
        Log::debug("self.offset_x 222", &self.offset_x);

        if self.offset_x != self.offset_x_org {
            self.set_offset_disp_x();
        }
    }

    pub fn set_offset_disp_x(&mut self) {
        let vec = if self.scrl_h.is_enable { self.buf.char_vec_row(self.scrl_h.row_max_width_idx) } else { self.buf.char_vec_row(self.cur.y) };

        if vec.len() >= self.offset_x {
            self.offset_disp_x = get_row_cur_x_disp_x(&vec[..self.offset_x], 0, false).1;
        }
    }

    /*
    pub fn add_scroll_margin_offset_x(&mut self, vec: &[char]) {
        const ADD_MARGIN_END_LINE: usize = 5;

        let offset_disp_x = get_row_cur_x_disp_x(&vec[..self.offset_x], 0, false).1;

        if self.cur.disp_x > offset_disp_x + self.col_len - Editor::SCROLL_LEFT_RIGHT_JUDGE_MARGIN {
            self.offset_x += ADD_MARGIN_END_LINE;
        }
    }
     */
}

#[cfg(test)]
mod tests {
    use ewin_com::_cfg::model::default::CfgLog;

    use super::*;

    // initial value
    // row_num : 10
    // Cur y:0, x:0, disp_x:0,
    #[test]
    fn test_editor_scroll_base() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        let mut e = Editor::new();
        e.buf.insert_end_multi(&["1\n2\n3\n4\n5\n6\n7\n8\n9\n10"]);

        /*** Downward test ***/
        e.e_cmd = E_Cmd::DelNextChar;

        // cur.y = 0
        e.scroll();
        assert_eq!(e.offset_y, 0);
        // cur.y < offset_y
        e.offset_y = 10;
        e.cur.y = 5;
        e.scroll();
        assert_eq!(e.offset_y, 0);

        // self.cur.y + Editor::UP_DOWN_MARGIN >= self.row_num
        // cur.y is last row
        e.offset_y = 0;
        e.cur.y = 20;
        e.scroll();
        assert_eq!(e.offset_y, 0);
        // cur.y is not  last row

        e.buf.clear();

        e.buf.insert_end_multi(&["1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20"]);

        e.offset_y = 0;
        e.cur.y = 9;
        e.scroll();
        assert_eq!(e.offset_y, 1);

        /*** Upward test ***/
        // self.cur.y == Editor::UP_DOWN_MARGIN + self.offset_y - 1
        e.e_cmd = E_Cmd::CursorUp;
        e.offset_y = 10;
        e.cur.y = 10;
        e.scroll();
        assert_eq!(e.offset_y, 9);
        // e.offset_y = 0;
        e.offset_y = 0;
        e.cur.y = 1;
        e.scroll();
        assert_eq!(e.offset_y, 0);

        /*** Page change test ***/

        e.e_cmd = E_Cmd::CursorPageUp;
        e.offset_y = 10;
        e.cur.y = 20;
        e.scroll();
        assert_eq!(e.offset_y, 0);
        e.offset_y = 5;
        e.cur.y = 15;
        e.scroll();
        assert_eq!(e.offset_y, 0);
        e.offset_y = 0;
        e.cur.y = 5;
        e.scroll();
        assert_eq!(e.offset_y, 0);
        // 1 page
        e.e_cmd = E_Cmd::CursorPageDown;
        e.offset_y = 0;
        e.cur.y = 10;
        e.scroll();
        assert_eq!(e.offset_y, 10);
        // not 1 page
        e.offset_y = 5;
        e.cur.y = 5;
        e.scroll();
        assert_eq!(e.offset_y, 10);
    }
}
