use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, util::*},
    model::*,
};
use std::{
    cmp::{max, min},
    usize,
};

impl Editor {
    pub const UP_DOWN_EXTRA: usize = 1;
    const LEFT_RIGHT_JUDGE_EXTRA: usize = 3;
    // offset_x Number of characters for switching judgment
    const SEARCH_JUDGE_COLUMN_EXTRA: usize = 5;
    // Number of offset increase / decrease when switching left / right offset
    const ADD_EXTRA_NUM: usize = 10;
    const ADD_EXTRA_END_LINE: usize = 5;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.cur.y", &self.cur.y);
        Log::debug("self.offset_y before", &self.offset_y);

        if self.cur.y == 0 {
            self.offset_y = 0
        } else {
            match &self.e_cmd {
                // When multi rows are deleted
                E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo if self.offset_y > self.cur.y => self.offset_y = self.cur.y - Editor::UP_DOWN_EXTRA,
                E_Cmd::GrepResult | E_Cmd::CursorDown | E_Cmd::CursorDownSelect | E_Cmd::MouseScrollDown | E_Cmd::CursorFileEnd | E_Cmd::InsertStr(_) | E_Cmd::FindNext | E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Undo | E_Cmd::Redo => {
                    if self.cur.y + Editor::UP_DOWN_EXTRA >= self.row_num {
                        // "self.buf.len_lines() - self.row_num" is For the last row
                        self.offset_y = max(self.offset_y, min(self.buf.len_lines() - self.row_num, self.cur.y + 1 + Editor::UP_DOWN_EXTRA - self.row_num));
                    }
                }
                E_Cmd::CursorUp | E_Cmd::MouseScrollUp | E_Cmd::CursorUpSelect | E_Cmd::FindBack => {
                    // -1 is to maintain consistency with the 0 standard of cur.y
                    if self.offset_y >= Editor::UP_DOWN_EXTRA && self.cur.y == Editor::UP_DOWN_EXTRA + self.offset_y - 1 {
                        self.offset_y -= Editor::UP_DOWN_EXTRA;
                    }
                }
                E_Cmd::CursorPageUp => self.offset_y = if self.offset_y >= self.row_num { self.offset_y - self.row_num } else { 0 },
                E_Cmd::CursorPageDown => self.offset_y = if self.buf.len_lines() - 1 > self.offset_y + self.row_num * 2 { self.offset_y + self.row_num } else { self.buf.len_lines() - self.row_num },
                _ => {}
            }
        }
        Log::debug("self.offset_y after", &self.offset_y);
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_key("scroll_horizontal");

        self.offset_x_org = self.offset_x;
        let vec = &self.buf.char_vec_line(self.cur.y);

        //// Calc offset_x
        // Up・Down・Home ...
        if 0 == self.cur.x {
            self.offset_x = 0;
            self.offset_disp_x = 0;
            return;
        } else if self.cur_y_org != self.cur.y {
            self.offset_x = self.get_x_offset(self.cur.y, self.cur.x);
            self.add_extra_offset_x(&vec);
        } else {
            match &self.e_cmd {
                E_Cmd::CursorRowHome | E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::InsertStr(_) | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::Null => {
                    self.offset_x = self.get_x_offset(self.cur.y, self.cur.x);

                    match &self.e_cmd {
                        E_Cmd::InsertStr(_) | E_Cmd::CursorRowEnd | E_Cmd::CursorRowEndSelect | E_Cmd::Undo | E_Cmd::Redo => {
                            self.add_extra_offset_x(&vec);
                        }
                        E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::Null => {
                            let str_width = get_str_width(&self.search.str);
                            let offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;
                            if self.e_cmd == E_Cmd::FindNext || self.e_cmd == E_Cmd::Null {
                                // Offset setting to display a few characters to the right of the search character for easier viewing
                                if self.cur.disp_x + str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA > offset_disp_x + self.col_num {
                                    self.offset_x += str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA;
                                }
                            } else if self.e_cmd == E_Cmd::FindBack {
                                // Calc offset_disp_x once to judge the display position
                                if self.cur.disp_x + str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA > offset_disp_x + self.col_num {
                                    self.offset_x += str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA;
                                }
                            }
                        }

                        _ => {}
                    }
                }
                E_Cmd::CursorRight | E_Cmd::CursorRightSelect => {
                    if self.offset_disp_x + self.col_num < self.cur.disp_x + Editor::LEFT_RIGHT_JUDGE_EXTRA {
                        // Judgment whether the end fits in the width
                        let width = get_row_width(&self.buf.char_vec_line(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.col_num {
                            self.offset_x += Editor::ADD_EXTRA_NUM;
                        }
                    }
                }
                E_Cmd::CursorLeft | E_Cmd::CursorLeftSelect => {
                    if self.cur.disp_x >= Editor::LEFT_RIGHT_JUDGE_EXTRA && self.offset_disp_x >= self.cur.disp_x - Editor::LEFT_RIGHT_JUDGE_EXTRA {
                        self.offset_x = if self.offset_x >= Editor::ADD_EXTRA_NUM { self.offset_x - Editor::ADD_EXTRA_NUM } else { 0 };
                    }
                }
                _ => {}
            }
        }
        self.offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;
    }

    pub fn add_extra_offset_x(&mut self, vec: &Vec<char>) {
        let offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;

        if self.cur.disp_x > offset_disp_x + self.col_num - Editor::LEFT_RIGHT_JUDGE_EXTRA {
            self.offset_x += Editor::ADD_EXTRA_END_LINE;
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
        assert_eq!(e.offset_y, 4);
        // self.cur.y + Editor::UP_DOWN_EXTRA >= self.row_num
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
        // self.cur.y == Editor::UP_DOWN_EXTRA + self.offset_y - 1
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
