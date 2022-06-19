use crate::{
    ewin_com::{model::*, util::*},
    model::*,
};
use ewin_cfg::log::*;
use ewin_com::_cfg::key::cmd::CmdType;
use ewin_const::def::*;
use std::{cmp::min, usize};

impl Editor {
    pub const SCROLL_UP_DOWN_MARGIN: usize = 1;
    const OFFSET_Y_MARGIN: usize = 3;
    const DISP_ROWS_MARGIN: usize = 3;
    const SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN: usize = 3;
    const SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN: usize = 8;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.cur.y", &self.cur.y);
        Log::debug("self.cmd", &self.cmd);
        Log::debug("self.buf.len_rows()", &self.buf.len_rows());
        Log::debug("self.scrl_v.is_enable", &self.scrl_v.is_enable);
        Log::debug("self.offset_y before", &self.offset_y);

        if self.cur.y == 0 && self.is_move_cur_posi_scrolling_enable() {
            self.offset_y = 0
        } else if !self.is_move_cur_posi_scrolling_enable() && (matches!(self.cmd.cmd_type, CmdType::MouseScrollDown) || matches!(self.cmd.cmd_type, CmdType::MouseScrollUp)) {
            Log::debug_s("1111111111111111111111111111111111111111111");
            if matches!(self.cmd.cmd_type, CmdType::MouseScrollDown) {
                if self.get_disp_row_including_extra() > self.row_len {
                    self.offset_y = min(self.offset_y + 1, self.get_disp_row_including_extra() - self.row_len);
                }
            } else {
                self.offset_y = if self.offset_y == 0 { 0 } else { self.offset_y - 1 };
            }
        } else if !self.is_move_cur_posi_scrolling_enable() && self.scrl_v.is_enable && (matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _))) {
            Log::debug_s("222222222222222222222222222222222222");
            self.offset_y = min(self.scrl_v.row_posi * self.scrl_v.move_len, self.get_disp_row_including_extra() - self.row_len);
            // When cursor is off the screen
        } else if !self.is_cur_y_in_screen() && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) && !matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) {
            // && !matches!(self.e_cmd, E_Cmd::MoveRow) {
            Log::debug_s("333333333333333333333333333333333");
            self.set_offset_move_row();
        } else {
            match &self.cmd.cmd_type {
                // When multi rows are deleted
                CmdType::CursorFileHome => self.offset_y = 0,
                CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack | CmdType::ReOpenFile if self.offset_y > self.cur.y => {
                    self.offset_y = if self.row_len >= self.get_disp_row_including_extra() - 1 {
                        0
                    } else if self.cur.y < Editor::OFFSET_Y_MARGIN + self.offset_y && self.cur.y >= Editor::OFFSET_Y_MARGIN {
                        self.cur.y - Editor::OFFSET_Y_MARGIN
                    } else {
                        self.offset_y
                    }
                }
                CmdType::ReplaceExec(_, _, _) | CmdType::GrepResultProm | CmdType::CursorDown | CmdType::CursorDownSelect | CmdType::MouseScrollDown | CmdType::CursorFileEnd | CmdType::InsertStr(_) | CmdType::InsertRow | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack => {
                    if self.cur.y + Editor::SCROLL_UP_DOWN_MARGIN >= self.row_len + self.offset_y {
                        self.offset_y = min(self.get_disp_row_including_extra() - self.row_len, self.cur.y + 1 + Editor::SCROLL_UP_DOWN_MARGIN - self.row_len);
                    }
                }
                CmdType::MoveRowProm => self.set_offset_move_row(),
                CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) => {
                    if self.sel.is_selected() {
                        if *y >= self.row_posi + self.row_len && self.get_disp_row_including_extra() - self.offset_y > self.row_len {
                            self.offset_y += Editor::SCROLL_UP_DOWN_MARGIN;
                        } else if *y < self.row_posi && self.offset_y >= Editor::SCROLL_UP_DOWN_MARGIN {
                            self.offset_y -= Editor::SCROLL_UP_DOWN_MARGIN;
                        }
                    }
                }
                CmdType::MouseDragLeftDown(y, _) => {
                    if *y == self.row_posi + self.get_disp_row_including_extra() {
                        self.offset_y += Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::MouseDragLeftUp(y, _) => {
                    if *y == 0 && self.offset_y >= Editor::SCROLL_UP_DOWN_MARGIN {
                        self.offset_y -= Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::CursorUp | CmdType::MouseScrollUp | CmdType::CursorUpSelect => {
                    // -1 is to maintain consistency with the 0 standard of cur.y
                    if self.offset_y >= Editor::SCROLL_UP_DOWN_MARGIN && self.cur.y < Editor::SCROLL_UP_DOWN_MARGIN + self.offset_y {
                        self.offset_y -= Editor::SCROLL_UP_DOWN_MARGIN;
                    }
                }
                CmdType::CursorPageUp => self.offset_y = if self.offset_y >= self.row_len { self.offset_y - self.row_len } else { 0 },
                CmdType::CursorPageDown => self.offset_y = if self.get_disp_row_including_extra() - 1 > self.offset_y + self.row_len * 2 { self.offset_y + self.row_len } else { self.get_disp_row_including_extra() - self.row_len },
                _ => {}
            }
        }
        Log::debug("self.offset_y after", &self.offset_y);
    }
    pub fn get_disp_row_including_extra(&self) -> usize {
        if self.scrl_v.bar_len != USIZE_UNDEFINED && self.scrl_v.bar_len > 0 {
            self.row_len + (self.row_len - self.scrl_v.bar_len) * self.scrl_v.move_len
        } else {
            return self.buf.len_rows() + Editor::DISP_ROWS_MARGIN;
        }
    }
    pub fn set_offset_move_row(&mut self) {
        Log::debug_key("set_offset_move_row");

        if self.cur.y >= self.offset_y + self.row_len {
            // last page
            Log::debug("self.offset_y 111", &self.offset_y);
            Log::debug("self.buf.len_rows() - 1", &(self.buf.len_rows() - 1));
            Log::debug("self.row_len ", &self.row_len);
            Log::debug("Editor::OFFSET_Y_MARGIN ", &Editor::OFFSET_Y_MARGIN);
            self.offset_y = if self.cur.y + self.row_len < self.buf.len_rows() - 1 { self.cur.y - Editor::OFFSET_Y_MARGIN } else { self.buf.len_rows() - 1 - self.row_len + Editor::OFFSET_Y_MARGIN };

            // self.buf.len_rows() - 1 + Editor::MOVE_ROW_EXTRA_NUM - self.row_len;
            Log::debug("self.offset_y 222", &self.offset_y);
        } else if self.cur.y < self.offset_y {
            self.offset_y = if self.cur.y > Editor::OFFSET_Y_MARGIN { self.cur.y - Editor::OFFSET_Y_MARGIN } else { 0 };
        }
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_key("scroll_horizontal");
        Log::debug("self.cur", &self.cur);
        Log::debug("self.cmd", &self.cmd);
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
            match &self.cmd.cmd_type {
                CmdType::ReplaceExec(_, _, _) | CmdType::CursorFileEnd | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect | CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo | CmdType::FindNext | CmdType::FindBack | CmdType::ReOpenFile | CmdType::Null => {
                    self.offset_x = get_x_offset(&self.buf.char_vec_range(self.cur.y, ..self.cur.x), self.col_len - X_MARGIN);
                }
                CmdType::CursorRight | CmdType::CursorRightSelect | CmdType::MouseDragLeftRight(_, _) if self.sel.mode != SelMode::BoxSelect => {
                    let judge_margin = if let CmdType::MouseDragLeftRight(_, _) = self.cmd.cmd_type { Editor::SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.offset_disp_x + self.col_len < self.cur.disp_x + judge_margin {
                        // Judgment whether the end fits in the width
                        let width = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.col_len {
                            self.offset_x += SCROLL_ADD_MARGIN;
                        }
                    }
                }
                CmdType::CursorLeft | CmdType::CursorLeftSelect | CmdType::MouseDragLeftLeft(_, _) => {
                    let judge_margin = if let CmdType::MouseDragLeftLeft(_, _) = self.cmd.cmd_type { Editor::SCROLL_HORIZONTAL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_HORIZONTAL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.cur.disp_x - self.offset_disp_x <= judge_margin {
                        self.offset_x = if self.offset_x >= SCROLL_ADD_MARGIN { self.offset_x - SCROLL_ADD_MARGIN } else { 0 };
                    }
                }
                _ => {}
            }
        }

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
}

#[cfg(test)]
mod tests {

    use ewin_cfg::model::default::CfgLog;
    use ewin_com::_cfg::key::cmd::Cmd;

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
        e.cmd = Cmd::to_cmd(CmdType::CursorUp);
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

        e.cmd = Cmd::to_cmd(CmdType::CursorPageUp);
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
        e.cmd = Cmd::to_cmd(CmdType::CursorPageDown);
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
