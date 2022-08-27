use crate::model::*;
use ewin_cfg::log::*;
use ewin_key::key::cmd::*;
use ewin_key::sel_range::*;
use ewin_utils::char_edit::*;
use std::{cmp::min, usize};

impl Editor {
    pub const SCROLL_UP_DOWN_MARGIN: usize = 1;
    const OFFSET_Y_MARGIN: usize = 2;
    const DISP_ROWS_MARGIN: usize = 3;
    const SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN: usize = 3;
    const SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN: usize = 8;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.win.curt().cur.y", &self.win_mgr.curt().cur.y);
        Log::debug("self.cmd", &self.cmd);
        Log::debug("self.buf.len_rows()", &self.buf.len_rows());
        Log::debug("self.win_mgr.curt().cur.yd", &self.win_mgr.curt().cur.y);
        Log::debug("self.offset_y before", &self.win_mgr.curt().offset.y);

        if self.win_mgr.curt().cur.y == 0 && self.is_move_cur_posi_scrolling_enable() {
            self.win_mgr.curt().offset.y = 0
        } else if !self.is_move_cur_posi_scrolling_enable() && (matches!(self.cmd.cmd_type, CmdType::MouseScrollDown) || matches!(self.cmd.cmd_type, CmdType::MouseScrollUp)) {
            if matches!(self.cmd.cmd_type, CmdType::MouseScrollDown) {
                if self.get_disp_row_including_extra() > self.get_curt_row_len() {
                    self.win_mgr.curt().offset.y = min(self.win_mgr.curt().offset.y + 1, self.get_disp_row_including_extra() - self.get_curt_row_len());
                }
            } else {
                self.win_mgr.curt().offset.y = if self.win_mgr.curt().offset.y == 0 { 0 } else { self.win_mgr.curt().offset.y - 1 };
            }
        } else if !self.is_move_cur_posi_scrolling_enable() && self.win_mgr.curt().scrl_v.is_enable && (matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _))) {
            self.win_mgr.curt().offset.y = min(self.win_mgr.curt().scrl_v.row_posi * self.win_mgr.curt().scrl_v.move_len, self.get_disp_row_including_extra() - self.get_curt_row_len());
            // When cursor is off the screen
        } else if !self.is_cur_y_in_screen() && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) {
            self.set_offset_move_row();
        } else {
            match &self.cmd.cmd_type {
                // When multi rows are deleted
                CmdType::CursorFileHome => self.win_mgr.curt().offset.y = 0,
                CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack | CmdType::ReOpenFile if self.win_mgr.curt().offset.y > self.win_mgr.curt().cur.y => {
                    self.win_mgr.curt().offset.y = if self.get_curt_row_len() >= self.get_disp_row_including_extra() - 1 {
                        0
                    } else if self.win_mgr.curt().cur.y < Editor::OFFSET_Y_MARGIN + self.win_mgr.curt().offset.y && self.win_mgr.curt().cur.y >= Editor::OFFSET_Y_MARGIN {
                        self.win_mgr.curt().cur.y - Editor::OFFSET_Y_MARGIN
                    } else {
                        self.win_mgr.curt().offset.y
                    }
                }
                CmdType::InsertRow | CmdType::ReplaceExec(_, _, _) | CmdType::GrepingProm(_) | CmdType::GrepResultProm | CmdType::CursorDown | CmdType::CursorDownSelect | CmdType::MouseScrollDown | CmdType::CursorFileEnd | CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack => {
                    if matches!(self.cmd.cmd_type, CmdType::InsertRow) && self.win_mgr.curt().cur.y == self.buf.len_rows() - 1 && self.win_mgr.curt().cur.y - self.win_mgr.curt().offset.y + Editor::OFFSET_Y_MARGIN > self.get_curt_row_len() {
                        self.win_mgr.curt().offset.y += 1;
                    } else if self.win_mgr.curt().cur.y + Editor::SCROLL_UP_DOWN_MARGIN >= self.get_curt_row_len() + self.win_mgr.curt().offset.y {
                        self.win_mgr.curt().offset.y = min(self.get_disp_row_including_extra() - self.get_curt_row_len(), self.win_mgr.curt().cur.y + 1 + Editor::SCROLL_UP_DOWN_MARGIN - self.get_curt_row_len());
                    }
                }
                CmdType::MoveRowProm => self.set_offset_move_row(),
                CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) => {
                    if self.win_mgr.curt().sel.is_selected() {
                        if *y >= self.get_curt_row_posi() + self.get_curt_row_len() && self.get_disp_row_including_extra() - self.win_mgr.curt().offset.y > self.get_curt_row_len() {
                            self.win_mgr.curt().offset.y += Editor::SCROLL_UP_DOWN_MARGIN;
                        } else if *y < self.get_curt_row_posi() && self.win_mgr.curt().offset.y >= Editor::SCROLL_UP_DOWN_MARGIN {
                            self.win_mgr.curt().offset.y -= Editor::SCROLL_UP_DOWN_MARGIN;
                        }
                    }
                }
                CmdType::MouseDragLeftDown(y, _) => {
                    if *y == self.get_curt_row_posi() + self.get_disp_row_including_extra() {
                        self.win_mgr.curt().offset.y += Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::MouseDragLeftUp(y, _) => {
                    if *y == 0 && self.win_mgr.curt().offset.y >= Editor::SCROLL_UP_DOWN_MARGIN {
                        self.win_mgr.curt().offset.y -= Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::CursorUp | CmdType::MouseScrollUp | CmdType::CursorUpSelect => {
                    // -1 is to maintain consistency with the 0 standard of cur.y
                    if self.win_mgr.curt().offset.y >= Editor::SCROLL_UP_DOWN_MARGIN && self.win_mgr.curt().cur.y < Editor::SCROLL_UP_DOWN_MARGIN + self.win_mgr.curt().offset.y {
                        self.win_mgr.curt().offset.y -= Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::CursorPageUp => self.win_mgr.curt().offset.y = if self.win_mgr.curt().offset.y >= self.get_curt_row_len() { self.win_mgr.curt().offset.y - self.get_curt_row_len() } else { 0 },
                CmdType::CursorPageDown => self.win_mgr.curt().offset.y = if self.get_disp_row_including_extra() - 1 > self.win_mgr.curt().offset.y + self.get_curt_row_len() * 2 { self.win_mgr.curt().offset.y + self.get_curt_row_len() } else { self.get_disp_row_including_extra() - self.get_curt_row_len() },
                _ => {}
            }
        }
        Log::debug("self.offset_y after", &self.win_mgr.curt().offset.y);
    }

    pub fn get_disp_row_including_extra(&self) -> usize {
        if self.win_mgr.curt_ref().scrl_v.bar_len != 0 && self.win_mgr.curt_ref().scrl_v.bar_len > 0 {
            self.get_curt_row_len() + (self.get_curt_row_len() - self.win_mgr.curt_ref().scrl_v.bar_len) * self.win_mgr.curt_ref().scrl_v.move_len
        } else {
            return self.buf.len_rows() + Editor::DISP_ROWS_MARGIN;
        }
    }
    pub fn set_offset_move_row(&mut self) {
        Log::debug_key("set_offset_move_row");

        if self.win_mgr.curt().cur.y >= self.win_mgr.curt().offset.y + self.get_curt_row_len() {
            // last page
            Log::debug("self.offset_y 111", &self.win_mgr.curt().offset.y);
            Log::debug("self.buf.len_rows() - 1", &(self.buf.len_rows() - 1));
            Log::debug("self.row_len ", &self.get_curt_row_len());
            Log::debug("Editor::OFFSET_Y_MARGIN ", &Editor::OFFSET_Y_MARGIN);
            self.win_mgr.curt().offset.y = if self.win_mgr.curt().cur.y + self.get_curt_row_len() < self.buf.len_rows() - 1 { self.win_mgr.curt().cur.y - Editor::OFFSET_Y_MARGIN } else { self.buf.len_rows() - 1 - self.get_curt_row_len() + Editor::OFFSET_Y_MARGIN };

            // self.buf.len_rows() - 1 + Editor::MOVE_ROW_EXTRA_NUM - self.row_len;
            Log::debug("self.offset_y 222", &self.win_mgr.curt().offset.y);
        } else if self.win_mgr.curt().cur.y < self.win_mgr.curt().offset.y {
            self.win_mgr.curt().offset.y = if self.win_mgr.curt().cur.y > Editor::OFFSET_Y_MARGIN { self.win_mgr.curt().cur.y - Editor::OFFSET_Y_MARGIN } else { 0 };
        }
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_key("scroll_horizontal");
        Log::debug("self.cur", &self.win_mgr.curt().cur);
        Log::debug("self.cmd", &self.cmd);
        Log::debug("self.offset_x 111", &self.win_mgr.curt().offset.x);

        // offset_x Number of characters for switching judgment
        const X_MARGIN: usize = 3;
        //  const SEARCH_JUDGE_COLUMN_MARGIN: usize = 5;
        // Number of offset increase / decrease when switching left / right offset
        const SCROLL_ADD_MARGIN: usize = 10;

        // self.offset_x_org = self.offset_x;
        let vec = &self.buf.char_vec_row(self.win_mgr.curt().cur.y);

        //// Calc offset_x
        // Up・Down・Home ...
        if 0 == self.win_mgr.curt().cur.x {
            self.win_mgr.curt().offset.x = 0;
            self.win_mgr.curt().offset.disp_x = 0;
            return;
        } else if self.win_mgr.curt().sel.mode == SelMode::BoxSelect && self.win_mgr.curt().cur_org.y != self.win_mgr.curt().cur.y {
            self.win_mgr.curt().offset.x = get_until_disp_x(vec, self.win_mgr.curt().offset.disp_x, false).0;
            return;

            // If the line is changed
        } else if self.win_mgr.curt().cur_org.y != self.win_mgr.curt().cur.y {
            if !self.is_cur_disp_x_in_screen() {
                self.win_mgr.curt().offset.x = get_x_offset(&self.buf.char_vec_range(self.win_mgr.curt().cur.y, ..self.win_mgr.curt().cur.x), self.get_curt_col_len() - X_MARGIN);
            }

            // If there is a permissible margin up to both ends of the range
        } else if (self.win_mgr.curt().cur.disp_x as isize - self.win_mgr.curt().offset.disp_x as isize).unsigned_abs() >= X_MARGIN && (self.win_mgr.curt().cur.disp_x as isize - (self.win_mgr.curt().offset.disp_x + self.get_curt_col_len()) as isize).unsigned_abs() >= X_MARGIN {
            return;
        } else {
            match &self.cmd.cmd_type {
                CmdType::ReplaceExec(_, _, _) | CmdType::CursorFileEnd | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect | CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack | CmdType::ReOpenFile | CmdType::Null => {
                    self.win_mgr.curt().offset.x = get_x_offset(&self.buf.char_vec_range(self.win_mgr.curt().cur.y, ..self.win_mgr.curt().cur.x), self.get_curt_col_len() - X_MARGIN);
                }
                CmdType::CursorRight | CmdType::CursorRightSelect | CmdType::MouseDragLeftRight(_, _) if self.win_mgr.curt().sel.mode != SelMode::BoxSelect => {
                    let judge_margin = if let CmdType::MouseDragLeftRight(_, _) = self.cmd.cmd_type { Editor::SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.win_mgr.curt().offset.disp_x + self.get_curt_col_len() < self.win_mgr.curt().cur.disp_x + judge_margin {
                        // Judgment whether the end fits in the width
                        let width = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.win_mgr.curt().cur.y)[self.win_mgr.curt().offset.x..], self.win_mgr.curt().offset.disp_x, true).1;
                        if width > self.get_curt_col_len() {
                            self.win_mgr.curt().offset.x += SCROLL_ADD_MARGIN;
                        }
                    }
                }
                CmdType::CursorLeft | CmdType::CursorLeftSelect | CmdType::MouseDragLeftLeft(_, _) => {
                    let judge_margin = if let CmdType::MouseDragLeftLeft(_, _) = self.cmd.cmd_type { Editor::SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.win_mgr.curt().cur.disp_x - self.win_mgr.curt().offset.disp_x <= judge_margin {
                        self.win_mgr.curt().offset.x = if self.win_mgr.curt().offset.x >= SCROLL_ADD_MARGIN { self.win_mgr.curt().offset.x - SCROLL_ADD_MARGIN } else { 0 };
                    }
                }
                _ => {}
            }
        }

        if self.win_mgr.curt().offset.x != self.win_mgr.curt().offset.x_org {
            self.set_offset_disp_x();
        }
    }

    pub fn set_offset_disp_x(&mut self) {
        let vec = if self.win_mgr.curt().scrl_h.is_enable { self.buf.char_vec_row(self.win_mgr.row_max_width_idx) } else { self.buf.char_vec_row(self.win_mgr.curt().cur.y) };

        if vec.len() >= self.win_mgr.curt().offset.x {
            self.win_mgr.curt().offset.disp_x = get_row_cur_x_disp_x(&vec[..self.win_mgr.curt().offset.x], 0, false).1;
        }
    }
}

#[cfg(test)]
mod tests {

    use ewin_cfg::model::default::CfgLog;
    use ewin_key::key::cmd::Cmd;

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
        e.cmd = Cmd::to_cmd(CmdType::DelNextChar);

        // cur.y = 0
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);
        // cur.y < offset_y
        e.win_mgr.curt().offset.y = 10;
        e.win_mgr.curt().cur.y = 5;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);

        // self.win.curt().cur.y + Editor::UP_DOWN_MARGIN >= self.row_num
        // cur.y is last row
        e.win_mgr.curt().offset.y = 0;
        e.win_mgr.curt().cur.y = 20;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);
        // cur.y is not  last row

        e.buf.clear();

        e.buf.insert_end_multi(&["1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20"]);

        e.win_mgr.curt().offset.y = 0;
        e.win_mgr.curt().cur.y = 9;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 1);

        /*** Upward test ***/
        // self.win.curt().cur.y == Editor::UP_DOWN_MARGIN + self.offset_y - 1
        e.cmd = Cmd::to_cmd(CmdType::CursorUp);
        e.win_mgr.curt().offset.y = 10;
        e.win_mgr.curt().cur.y = 10;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 9);
        // e.win.curt().offset_y = 0;
        e.win_mgr.curt().offset.y = 0;
        e.win_mgr.curt().cur.y = 1;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);

        /*** Page change test ***/

        e.cmd = Cmd::to_cmd(CmdType::CursorPageUp);
        e.win_mgr.curt().offset.y = 10;
        e.win_mgr.curt().cur.y = 20;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);
        e.win_mgr.curt().offset.y = 5;
        e.win_mgr.curt().cur.y = 15;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);
        e.win_mgr.curt().offset.y = 0;
        e.win_mgr.curt().cur.y = 5;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 0);
        // 1 page
        e.cmd = Cmd::to_cmd(CmdType::CursorPageDown);
        e.win_mgr.curt().offset.y = 0;
        e.win_mgr.curt().cur.y = 10;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 10);
        // not 1 page
        e.win_mgr.curt().offset.y = 5;
        e.win_mgr.curt().cur.y = 5;
        e.scroll();
        assert_eq!(e.win_mgr.curt().offset.y, 10);
    }
}
