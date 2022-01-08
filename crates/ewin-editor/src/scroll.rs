use ewin_com::model::SelMode;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, global::*, log::*, util::*},
    model::*,
};
use std::{
    cmp::{max, min},
    usize,
};

impl Editor {
    pub const SCROLL_UP_DOWN_MARGIN: usize = 1;
    const OFFSET_Y_MARGIN: usize = 3;
    const SCROLL_LEFT_RIGHT_JUDGE_MARGIN: usize = 3;
    const SCROLL_MOUSE_DRAG_LEFT_JUDGE_MARGIN: usize = 8;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.cur.y", &self.cur.y);
        Log::debug("self.e_cmd", &self.e_cmd);
        Log::debug("self.buf.len_rows()", &self.buf.len_rows());
        Log::debug("self.offset_y before", &self.offset_y);

        if self.cur.y == 0 && CFG.get().unwrap().try_lock().unwrap().general.editor.cursor.move_position_by_scrolling_enable {
            self.offset_y = 0
        } else if !CFG.get().unwrap().try_lock().unwrap().general.editor.cursor.move_position_by_scrolling_enable && (matches!(self.e_cmd, E_Cmd::MouseScrollDown) || matches!(self.e_cmd, E_Cmd::MouseScrollUp)) {
            if matches!(self.e_cmd, E_Cmd::MouseScrollDown) {
                self.offset_y = min(self.offset_y + 1, self.buf.len_rows() - self.row_disp_len);
            } else {
                self.offset_y = if self.offset_y == 0 { 0 } else { self.offset_y - 1 };
            }
        } else if !CFG.get().unwrap().try_lock().unwrap().general.editor.cursor.move_position_by_scrolling_enable && self.scrl_v.is_enable && (matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _))) {
            self.offset_y = self.disp_y;
        } else {
            match &self.e_cmd {
                // When multi rows are deleted
                E_Cmd::CursorFileHome => self.offset_y = 0,
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::ReOpenFile if self.offset_y > self.cur.y => {
                    self.offset_y = if self.row_disp_len >= self.buf.len_rows() - 1 {
                        0
                    } else if self.cur.y < Editor::OFFSET_Y_MARGIN + self.offset_y && self.cur.y >= Editor::OFFSET_Y_MARGIN {
                        self.cur.y - Editor::OFFSET_Y_MARGIN
                    } else {
                        self.offset_y
                    }
                }
                E_Cmd::ReplaceExec(_, _, _, _) | E_Cmd::GrepResult | E_Cmd::CursorDown | E_Cmd::CursorDownSelect | E_Cmd::MouseScrollDown | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::CursorFileEnd | E_Cmd::InsertStr(_) | E_Cmd::InsertRow | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack => {
                    if self.cur.y + Editor::SCROLL_UP_DOWN_MARGIN >= self.row_disp_len {
                        // "self.buf.len_lines() - self.row_num" is For the last row
                        self.offset_y = max(self.offset_y, min(self.buf.len_rows() - self.row_disp_len, self.cur.y + 1 + Editor::SCROLL_UP_DOWN_MARGIN - self.row_disp_len));
                    }
                }
                E_Cmd::MoveRow => self.set_offset_y_move_row(),
                E_Cmd::MouseDragLeftLeft(y, _) | E_Cmd::MouseDragLeftRight(y, _) => {
                    if self.sel.is_selected() {
                        if *y >= self.row_posi + self.row_disp_len && self.buf.len_rows() - self.offset_y > self.row_disp_len {
                            Log::debug("yyyy", &y);
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
                E_Cmd::CursorPageDown => self.offset_y = if self.buf.len_rows() - 1 > self.offset_y + self.row_disp_len * 2 { self.offset_y + self.row_disp_len } else { self.buf.len_rows() - self.row_disp_len },
                _ => {}
            }
        }
        Log::debug("self.offset_y after", &self.offset_y);
    }

    // move to row
    pub fn set_offset_y_move_row(&mut self) {
        if self.get_vertical_val() >= self.offset_y + self.row_disp_len {
            // last page
            self.offset_y = if self.buf.len_rows() - 1 - self.get_vertical_val() < self.row_disp_len { self.buf.len_rows() - self.row_disp_len } else { self.get_vertical_val() - Editor::MOVE_ROW_EXTRA_NUM }
        } else if self.get_vertical_val() < self.offset_y {
            self.offset_y = if self.get_vertical_val() > Editor::MOVE_ROW_EXTRA_NUM { self.get_vertical_val() - Editor::MOVE_ROW_EXTRA_NUM } else { 0 };
        }
    }
    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_key("scroll_horizontal");
        Log::debug("self.offset_x 111", &self.offset_x);
        Log::debug("self.sel.is_selected()", &self.sel.is_selected());
        Log::debug("self.sel", &self.sel);

        // offset_x Number of characters for switching judgment
        const SEARCH_JUDGE_COLUMN_MARGIN: usize = 5;
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
        } else if matches!(self.e_cmd, E_Cmd::CursorFileEnd) {
            self.offset_x = get_x_offset_by_cur_x(&self.buf.char_vec_range(self.cur.y, ..self.cur.x), self.col_len);
        } else if self.cur_y_org != self.cur.y && self.offset_disp_x == self.offset_disp_x_org {
            self.offset_x = get_until_disp_x(vec, self.offset_disp_x).0;
            return;
        } else if self.cur_y_org != self.cur.y && self.offset_disp_x != self.offset_disp_x_org {
        } else {
            match &self.e_cmd {
                E_Cmd::ReplaceExec(_, _, _, _) | E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::InsertStr(_) | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::ReOpenFile | E_Cmd::Null => {
                    self.offset_x = get_x_offset_by_cur_x(&self.buf.char_vec_range(self.cur.y, ..self.cur.x), self.col_len);

                    match &self.e_cmd {
                        E_Cmd::InsertStr(_) | E_Cmd::CursorRowEnd | E_Cmd::CursorRowEndSelect | E_Cmd::Undo | E_Cmd::Redo => {
                            self.add_scroll_margin_offset_x(vec);
                        }
                        E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::Null => {
                            let str_width = get_str_width(&self.search.str);
                            let offset_disp_x = get_row_cur_x_disp_x(&vec[..self.offset_x], 0, false).1;
                            if self.cur.disp_x + str_width + SEARCH_JUDGE_COLUMN_MARGIN > offset_disp_x + self.col_len {
                                self.offset_x += str_width + SEARCH_JUDGE_COLUMN_MARGIN;
                            }
                        }

                        _ => {}
                    }
                }
                E_Cmd::CursorRight | E_Cmd::CursorRightSelect | E_Cmd::MouseDragLeftRight(_, _) if self.sel.mode != SelMode::BoxSelect => {
                    Log::debug_s("1111111111111111111111");
                    Log::debug(" self.offset_x", &self.offset_x);

                    let judge_margin = if let E_Cmd::MouseDragLeftRight(_, _) = self.e_cmd { Editor::SCROLL_MOUSE_DRAG_LEFT_JUDGE_MARGIN } else { Editor::SCROLL_LEFT_RIGHT_JUDGE_MARGIN };

                    if self.offset_disp_x + self.col_len < self.cur.disp_x + judge_margin {
                        // Judgment whether the end fits in the width
                        let width = get_row_cur_x_disp_x(&self.buf.char_vec_row(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.col_len {
                            self.offset_x += SCROLL_ADD_MARGIN;
                        }
                    }
                    Log::debug_s("22222222222222222222");
                    Log::debug(" self.offset_x", &self.offset_x);
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
        let vec = if self.scrl_h.is_enable { self.buf.char_vec_row(self.scrl_h.max_width_row_idx) } else { self.buf.char_vec_row(self.cur.y) };

        Log::debug("vec.len()", &vec.len());
        Log::debug("self.offset_x", &self.offset_x);

        if vec.len() >= self.offset_x {
            self.offset_disp_x = get_row_cur_x_disp_x(&vec[..self.offset_x], 0, false).1;
        }
    }

    pub fn add_scroll_margin_offset_x(&mut self, vec: &[char]) {
        const ADD_MARGIN_END_LINE: usize = 5;

        let offset_disp_x = get_row_cur_x_disp_x(&vec[..self.offset_x], 0, false).1;

        if self.cur.disp_x > offset_disp_x + self.col_len - Editor::SCROLL_LEFT_RIGHT_JUDGE_MARGIN {
            self.offset_x += ADD_MARGIN_END_LINE;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ewin_com::{_cfg::cfg::*, def::*};

    // initial value
    // row_num : 10
    // Cur y:0, x:0, disp_x:0,
    #[test]
    fn test_editor_scroll_base() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        let mut e = Editor::new();
        e.buf.insert_end_multi(&["1\n2\n3\n4\n5\n6\n7\n8\n9\n10", EOF_MARK_STR]);

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

        e.buf.insert_end_multi(&["1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20", EOF_MARK_STR]);

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
