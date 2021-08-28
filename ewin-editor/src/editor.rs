use crate::{
    ewin_core::{_cfg::keys::*, log::*, model::*, util::*},
    model::*,
};
use std::{
    cmp::{max, min},
    usize,
};

impl Editor {
    pub const UP_DOWN_EXTRA: usize = 1;
    const MOVE_ROW_EXTRA_NUM: usize = 3;
    const LEFT_RIGHT_JUDGE_EXTRA: usize = 3;
    // offset_x Number of characters for switching judgment
    const SEARCH_JUDGE_COLUMN_EXTRA: usize = 5;
    // Number of offset increase / decrease when switching left / right offset
    const ADD_EXTRA_NUM: usize = 10;
    const ADD_EXTRA_END_LINE: usize = 5;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_key("scroll");
        Log::debug("self.keycmd", &self.keycmd);

        if self.keycmd == KeyCmd::CursorFileHome || self.cur.y == 0 {
            self.offset_y = 0;
        } else if self.keycmd == KeyCmd::CursorPageUp {
            self.offset_y = if self.offset_y >= self.disp_row_num { self.offset_y - self.disp_row_num } else { 0 };
        } else if self.cur.y >= Editor::UP_DOWN_EXTRA {
            self.offset_y = min(self.offset_y, self.cur.y - Editor::UP_DOWN_EXTRA);
        }

        match self.keycmd {
            KeyCmd::CursorDown | KeyCmd::CursorUp | KeyCmd::CursorDownSelect | KeyCmd::CursorUpSelect | KeyCmd::MouseScrollUp | KeyCmd::MouseScrollDown | KeyCmd::CursorPageDown | KeyCmd::CursorPageUp | KeyCmd::CursorFileEnd | KeyCmd::InsertStr(_) | KeyCmd::Find 
            // Prompt confirm
            | KeyCmd::InsertLine => {
                if self.cur.y + Editor::UP_DOWN_EXTRA >= self.disp_row_num {
                    if self.keycmd == KeyCmd::CursorPageDown {
                        self.offset_y = if self.buf.len_lines() - 1 > self.offset_y + self.disp_row_num * 2 { self.offset_y + self.disp_row_num } else { self.buf.len_lines() - self.disp_row_num };
                    } else {
                        Log::debug("self.offset_y 111", &self.offset_y);

                        self.offset_y = max(self.offset_y, self.cur.y + 1 + Editor::UP_DOWN_EXTRA - self.disp_row_num);

                        Log::debug("self.offset_y 222", &self.offset_y);
                        // offset_y decreases
                        if self.offset_y + self.disp_row_num > self.buf.len_lines() {
                            self.offset_y = self.buf.len_lines() - self.disp_row_num;
                        }
                        Log::debug("self.offset_y 333", &self.offset_y);
                    }
                }
            }
            _ => {}
        }
    }

    // move to row
    pub fn move_row(&mut self) {
        if self.cur.y > self.offset_y + self.disp_row_num {
            // last page
            if self.buf.len_lines() - 1 - self.cur.y < self.disp_row_num {
                self.offset_y = self.buf.len_lines() - self.disp_row_num;
            } else {
                self.offset_y = self.cur.y - Editor::MOVE_ROW_EXTRA_NUM;
            }
        } else if self.cur.y < self.offset_y {
            self.offset_y = if self.cur.y > Editor::MOVE_ROW_EXTRA_NUM { self.cur.y - Editor::MOVE_ROW_EXTRA_NUM } else { 0 };
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
            match self.keycmd {
                KeyCmd::CursorRowEnd | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::InsertStr(_) | KeyCmd::Undo | KeyCmd::Redo | KeyCmd::FindNext | KeyCmd::FindBack | KeyCmd::Null => {
                    self.offset_x = self.get_x_offset(self.cur.y, self.cur.x);

                    match self.keycmd {
                        KeyCmd::InsertStr(_) | KeyCmd::CursorRowEnd | KeyCmd::CursorRowEndSelect | KeyCmd::Undo | KeyCmd::Redo => {
                            self.add_extra_offset(&vec);
                        }
                        KeyCmd::FindNext | KeyCmd::FindBack | KeyCmd::Null => {
                            let str_width = get_str_width(&self.search.str);
                            if self.keycmd == KeyCmd::FindNext || self.keycmd == KeyCmd::Null {
                                // Offset setting to display a few characters to the right of the search character for easier viewing
                                if self.cur.disp_x + str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA > self.offset_disp_x + self.disp_col_num {
                                    self.offset_x += str_width + Editor::SEARCH_JUDGE_COLUMN_EXTRA;
                                }
                            } else if self.keycmd == KeyCmd::FindBack {
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
                KeyCmd::CursorRight | KeyCmd::CursorRightSelect => {
                    if self.offset_disp_x + self.disp_col_num < self.cur.disp_x + Editor::LEFT_RIGHT_JUDGE_EXTRA {
                        // Judgment whether the end fits in the width
                        let width = get_row_width(&self.buf.char_vec_line(self.cur.y)[self.offset_x..], self.offset_disp_x, true).1;
                        if width > self.disp_col_num {
                            self.offset_x += Editor::ADD_EXTRA_NUM;
                        }
                    }
                }
                KeyCmd::CursorLeft | KeyCmd::CursorLeftSelect => {
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

    pub fn add_extra_offset(&mut self, vec: &Vec<char>) {
        let offset_disp_x = get_row_width(&vec[..self.offset_x], 0, false).1;

        if self.cur.disp_x > offset_disp_x + self.disp_col_num - Editor::LEFT_RIGHT_JUDGE_EXTRA {
            self.offset_x += Editor::ADD_EXTRA_END_LINE;
        }
    }
    /// Get x_offset from the specified y・x
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut cur_x, mut width) = (0, 0);
        let char_vec = self.buf.char_vec_range(y, x);

        for c in char_vec.iter().rev() {
            width += get_char_width(c, width);
            if width > self.disp_col_num {
                break;
            }
            cur_x += 1;
        }
        return x - cur_x;
    }

    pub fn set_cur_default(&mut self) {
        if self.mouse_mode == MouseMode::Normal {
            self.rnw = self.buf.len_lines().to_string().len();
        } else {
            self.rnw = 0;
        }
        self.cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target(&mut self, y: usize, x: usize, is_ctrlchar_incl: bool) {
        self.cur.y = y;
        let (cur_x, width) = get_row_width(&self.buf.char_vec_range(y, x), 0, is_ctrlchar_incl);
        self.rnw = if self.mouse_mode == MouseMode::Normal { self.buf.len_lines().to_string().len() } else { 0 };
        self.cur.disp_x = width;
        self.cur.x = cur_x;
    }

    pub fn get_rnw(&self) -> usize {
        return self.rnw;
    }

    pub fn get_rnw_and_margin(&self) -> usize {
        return self.rnw + Editor::RNW_MARGIN;
    }

    pub fn set_org_state(&mut self) {
        // let tab = term.tabs.get_mut(term.idx).unwrap();

        self.cur_y_org = self.cur.y;
        self.offset_y_org = self.offset_y;
        self.offset_x_org = self.offset_x;
        self.rnw_org = self.get_rnw();
        self.sel_org = self.sel;
    }

    pub fn set_keys(&mut self, keys: &Keys) {
        self.keys = *keys;
        self.keycmd = Keybind::keys_to_keycmd(keys, KeyWhen::EditorFocus);
    }
}

impl Editor {
    pub const RNW_MARGIN: usize = 1;

    pub fn new() -> Self {
        Editor {
            mouse_mode: MouseMode::Normal,
            buf: TextBuffer::default(),
            buf_cache: vec![],
            cur: Cur::default(),
            offset_y: 0,
            offset_y_org: 0,
            offset_x: 0,
            offset_x_org: 0,
            offset_disp_x: 0,
            cur_y_org: 0,
            is_changed: false,
            updown_x: 0,
            rnw: 0,
            rnw_org: 0,
            //  sel_range: SelRange::default(),
            sel: SelRange::default(),
            sel_org: SelRange::default(),
            keys: Keys::Null,
            keycmd: KeyCmd::Null,
            // for UT set
            disp_row_num: 5,
            disp_row_posi: 1,
            disp_col_num: 5,
            search: Search::default(),
            //  draw: Draw::default(),
            draw_type: DrawType::default(),
            history: History::default(),
            grep_result_vec: vec![],

            // TODO workspace
            key_vec: vec![],
            is_enable_syntax_highlight: false,
            h_file: HeaderFile::default(),
            box_insert: BoxInsert::default(),
        }
    }
    pub fn record_key(&mut self) {
        match self.keycmd {
            // Ctrl
            KeyCmd::Copy | KeyCmd::Cut | KeyCmd::AllSelect | KeyCmd::InsertStr(_) | KeyCmd::CursorFileHome | KeyCmd::CursorFileEnd => self.key_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            // Shift
            KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => self.key_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            KeyCmd::FindBack => self.key_vec.push(KeyMacro { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            // Raw
            KeyCmd::InsertLine | KeyCmd::DeletePrevChar | KeyCmd::DeleteNextChar | KeyCmd::CursorPageUp | KeyCmd::CursorPageDown | KeyCmd::Tab | KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => self.key_vec.push(KeyMacro { keys: self.keys, ..KeyMacro::default() }),
            KeyCmd::FindNext => self.key_vec.push(KeyMacro { keys: self.keys, search: Search { str: self.search.str.clone(), ..Search::default() } }),
            _ => {}
        }
    }
    pub fn box_select_mode(&mut self) {
        Log::debug_key("box_select_mode");
        self.sel.clear();
        self.sel.mode = match self.sel.mode {
            SelMode::Normal => SelMode::BoxSelect,
            SelMode::BoxSelect => SelMode::Normal,
        };
        if self.sel.mode == SelMode::BoxSelect {
            // Initial processing for Box Insert without moving the cursor
            self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);
            self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        }
    }
}
