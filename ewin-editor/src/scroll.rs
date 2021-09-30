use crate::{
    ewin_core::{_cfg::key::keycmd::*, def::*, log::*, util::*},
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

        if self.e_cmd == E_Cmd::CursorFileHome || self.cur.y == 0 {
            self.offset_y = 0;
        } else if self.e_cmd == E_Cmd::CursorPageUp {
            self.offset_y = if self.offset_y >= self.disp_row_num { self.offset_y - self.disp_row_num } else { 0 };
        } else if self.cur.y >= Editor::UP_DOWN_EXTRA {
            self.offset_y = min(self.offset_y, self.cur.y - Editor::UP_DOWN_EXTRA);
        }

        match &self.e_cmd {
                E_Cmd::GrepResult | E_Cmd::CursorDown |E_Cmd::CursorUp | E_Cmd::CursorDownSelect | E_Cmd::CursorUpSelect | E_Cmd::MouseScrollUp  | E_Cmd::MouseScrollDown | E_Cmd::CursorPageDown | E_Cmd::CursorPageUp | E_Cmd::CursorFileEnd | E_Cmd::InsertStr(_) | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::DelNextChar | E_Cmd::DelPrevChar
            // Prompt confirm
            // | E_Cmd::InsertLine
             => {
                 // When all the display rows fit in the terminal
                if self.buf.len_lines() - self.offset_y + STATUSBAR_ROW_NUM  <= self.disp_row_num {
            
                } else {
                    if self.cur.y + Editor::UP_DOWN_EXTRA >= self.disp_row_num {
                  
                        if self.e_cmd == E_Cmd::CursorPageDown {
                            self.offset_y = if self.buf.len_lines() - 1 > self.offset_y + self.disp_row_num * 2 { self.offset_y + self.disp_row_num } else { self.buf.len_lines() - self.disp_row_num };
                        } else {
                     
                            self.offset_y = max(self.offset_y, self.cur.y + 1 + Editor::UP_DOWN_EXTRA - self.disp_row_num);

                            // offset_y decreases
                            if self.offset_y + self.disp_row_num > self.buf.len_lines() {
                                self.offset_y = self.buf.len_lines() - self.disp_row_num;
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    pub fn add_extra_offset(&mut self, vec: &Vec<char>) {
        let offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;

        if self.cur.disp_x > offset_disp_x + self.disp_col_num - Editor::LEFT_RIGHT_JUDGE_EXTRA {
            self.offset_x += Editor::ADD_EXTRA_END_LINE;
        }
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
            self.add_extra_offset(&vec);
        } else {
            match &self.e_cmd {
                E_Cmd::CursorRowEnd | E_Cmd::CursorRowHomeSelect | E_Cmd::CursorRowEndSelect | E_Cmd::InsertStr(_) | E_Cmd::Undo | E_Cmd::Redo | E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::Null => {
                    self.offset_x = self.get_x_offset(self.cur.y, self.cur.x);

                    match &self.e_cmd {
                        E_Cmd::InsertStr(_) | E_Cmd::CursorRowEnd | E_Cmd::CursorRowEndSelect | E_Cmd::Undo | E_Cmd::Redo => {
                            self.add_extra_offset(&vec);
                        }
                        E_Cmd::FindNext | E_Cmd::FindBack | E_Cmd::Null => {
                            let str_width = get_str_width(&self.search.str);
                            if self.e_cmd == E_Cmd::FindNext || self.e_cmd == E_Cmd::Null {
                                // Offset setting to display a few characters to the right of the search character for easier viewing
                                if self.cur.disp_x + str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA > self.offset_disp_x + self.disp_col_num {
                                    self.offset_x += str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA;
                                }
                            } else if self.e_cmd == E_Cmd::FindBack {
                                // Calc offset_disp_x once to judge the display position
                                let offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;
                                if self.cur.disp_x + str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA > offset_disp_x + self.disp_col_num {
                                    self.offset_x += str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA;
                                }
                            }
                        }

                        _ => {}
                    }
                }
                E_Cmd::CursorRight | E_Cmd::CursorRightSelect => {
                    if self.offset_disp_x + self.disp_col_num < self.cur.disp_x + Editor::LEFT_RIGHT_JUDGE_EXTRA {
                        // Judgment whether the end fits in the width
                        let width = get_row_width(&self.buf.char_vec_line(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.disp_col_num {
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
        //     self.offset_disp_x = get_row_width(&vec[..self.offset_x], self.offset_disp_x, false).1;
        self.offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;
    }
}
