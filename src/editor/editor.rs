use crate::{def::*, log::*, model::*, terminal::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent as M_Event, MouseEventKind as M_Kind};
use std::cmp::{max, min};

impl Editor {
    pub const SCROLL_UP_EXTRA_NUM: usize = 1;
    pub const SCROLL_DOWN_EXTRA_NUM: usize = 1;
    const SCROLL_MOVE_EXTRA_NUM: usize = 3;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::debug_s("　　　　　　　scroll");

        if self.evt == CTRL_HOME || self.cur.y == 0 {
            self.offset_y = 0;
        } else if self.evt == PAGE_UP {
            self.offset_y = if self.offset_y >= self.disp_row_num { self.offset_y - self.disp_row_num } else { 0 };
        } else {
            if self.cur.y >= Editor::SCROLL_UP_EXTRA_NUM {
                self.offset_y = min(self.offset_y, self.cur.y - Editor::SCROLL_UP_EXTRA_NUM);
            }
        }

        if self.cur.y + Editor::SCROLL_DOWN_EXTRA_NUM >= self.disp_row_num {
            if self.evt == PAGE_DOWN {
                self.offset_y = if self.buf.len_lines() - 1 > self.offset_y + self.disp_row_num * 2 { self.offset_y + self.disp_row_num } else { self.buf.len_lines() - self.disp_row_num };
            } else {
                // self.offset_y = max(self.offset_y, self.cur.y + Editor::SCROLL_DOWN_EXTRA_NUM + 1 - self.disp_row_num);
                self.offset_y = max(self.offset_y, self.cur.y + Editor::SCROLL_DOWN_EXTRA_NUM - self.disp_row_num);
                // offset_y decreases
                if self.offset_y + self.disp_row_num > self.buf.len_lines() {
                    self.offset_y = self.buf.len_lines() - self.disp_row_num;
                }
            }
        }
    }

    // move to row
    pub fn scroll_move_row(&mut self) {
        if self.cur.y > self.offset_y + self.disp_row_num {
            // last page
            if self.buf.len_lines() - 1 - self.cur.y < self.disp_row_num {
                self.offset_y = self.buf.len_lines() - self.disp_row_num;
            } else {
                self.offset_y = self.cur.y - Editor::SCROLL_MOVE_EXTRA_NUM;
            }
        } else if self.cur.y < self.offset_y {
            self.offset_y = if self.cur.y > Editor::SCROLL_MOVE_EXTRA_NUM { self.cur.y - Editor::SCROLL_MOVE_EXTRA_NUM } else { 0 };
        }
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        Log::debug_s("　　　　　　　scroll_horizontal");

        // offset_x Number of characters for switching judgment
        let extra_num = 3;
        let extra_num_search = 5;
        // Number of offset increase / decrease when switching the above offset
        let add_width_num = 10;

        self.offset_x_org = self.offset_x;

        let vec = &self.buf.char_vec_line(self.cur.y);

        // Calc offset_x
        // Up・Down・Home ...
        if 0 == self.cur.x {
            self.offset_x = 0;
            self.offset_disp_x = 0;
            return;

        // KEY_NULL:grep_result initial display
        } else if self.cur_y_org != self.cur.y || self.evt == PASTE || self.evt == END || self.evt == SEARCH_ASC || self.evt == SEARCH_DESC || self.evt == KEY_NULL {
            self.offset_x = self.get_x_offset(self.cur.y, self.cur.x);

            if self.evt == END {
                // +3 extra
                if self.cur.disp_x > self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN {
                    self.offset_x += 3;
                }
            }

            if self.evt == SEARCH_ASC || self.evt == SEARCH_DESC || self.evt == KEY_NULL {
                let str_width = get_str_width(&self.search.str);
                if self.evt == SEARCH_ASC || self.evt == KEY_NULL {
                    // Offset setting to display a few characters to the right of the search character for easier viewing
                    if self.cur.disp_x + str_width + extra_num_search > self.offset_disp_x + self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN {
                        self.offset_x += str_width + extra_num_search;
                    }
                } else if self.evt == SEARCH_DESC {
                    // Calc offset_disp_x once to judge the display position
                    let offset_disp_x = get_row_width(&vec[..self.offset_x], self.offset_disp_x, false).1;
                    // +5 extra
                    if self.cur.disp_x + str_width + extra_num_search > offset_disp_x + self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN {
                        self.offset_x += str_width + extra_num_search;
                    }
                }
            }

            // cur_right
        } else if self.evt == RIGHT && self.offset_disp_x + self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN < self.cur.disp_x + extra_num {
            let width = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], self.offset_disp_x, true).1;
            if width > self.disp_col_num - self.get_rnw() - Editor::RNW_MARGIN {
                self.offset_x += add_width_num;
            }

        // cur_left
        } else if self.evt == LEFT && self.cur.disp_x >= extra_num && self.offset_disp_x >= self.cur.disp_x - extra_num {
            self.offset_x = if self.offset_x >= add_width_num { self.offset_x - add_width_num } else { 0 };
        }

        // Calc offset_disp_x
        if self.cur_y_org != self.cur.y {
            self.offset_disp_x = get_row_width(&vec[..self.offset_x], self.offset_disp_x, false).1;
        } else if self.offset_x_org != self.offset_x {
            if self.offset_x < self.offset_x_org {
                self.offset_disp_x -= get_row_width(&vec[self.offset_x..self.offset_x_org], self.offset_disp_x, false).1;
            } else {
                self.offset_disp_x += get_row_width(&vec[self.offset_x_org..self.offset_x], self.offset_disp_x, false).1;
            }
        }
    }

    /// Get x_offset from the specified y・x
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut cur_x, mut width) = (0, 0);
        let char_vec = self.buf.char_vec_range(y, x);
        for c in char_vec.iter().rev() {
            width += get_char_width(c, width);

            let rnw_margin = if self.mode == TermMode::Normal { self.get_rnw() + Editor::RNW_MARGIN + 1 } else { 0 };
            if width > self.disp_col_num - rnw_margin {
                break;
            }
            cur_x += 1;
        }
        return x - cur_x;
    }

    pub fn del_sel_range(&mut self) {
        let sel = self.sel.get_range();
        self.buf.remove_range(sel);
        self.set_cur_target(sel.sy, sel.sx);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_cur_default(&mut self) {
        if self.mode == TermMode::Normal {
            self.rnw = self.buf.len_lines().to_string().len();
        } else {
            self.rnw = 0;
        }
        self.cur = Cur { y: 0, x: 0, disp_x: 0 };
    }

    pub fn set_cur_target(&mut self, y: usize, x: usize) {
        self.cur.y = y;
        let (cur_x, width) = get_row_width(&self.buf.char_vec_range(y, x), self.offset_disp_x, false);

        self.rnw = if self.mode == TermMode::Normal { self.buf.len_lines().to_string().len() } else { 0 };
        self.cur.disp_x = width;
        self.cur.x = cur_x;
    }

    pub fn is_edit_evt(&self, is_incl_unredo: bool) -> bool {
        match self.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Char('x') | Char('v') => return true,
                _ => return false,
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Char(_) => return true,
                _ => return false,
            },
            Key(KeyEvent { code, modifiers: KeyModifiers::NONE }) => match code {
                Delete | Backspace | Enter | Char(_) => return true,
                _ => return false,
            },
            _ => {
                if is_incl_unredo {
                    if self.evt == UNDO || self.evt == REDO {
                        return true;
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    pub fn set_draw_range(&mut self) {
        if self.d_range.draw_type != DrawType::All {
            if (self.offset_x > 0 && self.cur_y_org != self.cur.y) || self.offset_x_org != self.offset_x {
                self.d_range = DRange::new(min(self.cur_y_org, self.cur.y), max(self.cur_y_org, self.cur.y), DrawType::Target);
            }
            if self.rnw_org != self.get_rnw() {
                self.d_range.draw_type = DrawType::All;
            }
            if self.offset_y_org != self.offset_y {
                match self.evt {
                    Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code, .. }) => match code {
                        _ => self.d_range.draw_type = DrawType::All,
                    },
                    Key(KeyEvent { code, .. }) => match code {
                        Down => self.set_draw_range_scroll(self.offset_y + self.disp_row_num - 1, DrawType::ScrollDown),
                        Up => self.set_draw_range_scroll(self.offset_y, DrawType::ScrollUp),
                        _ => self.d_range.draw_type = DrawType::All,
                    },

                    Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => self.set_draw_range_scroll(self.offset_y + self.disp_row_num - 1, DrawType::ScrollDown),
                    Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => self.set_draw_range_scroll(self.offset_y, DrawType::ScrollUp),
                    _ => self.d_range.draw_type = DrawType::All,
                }
            }
        }
    }

    pub fn set_draw_range_scroll(&mut self, y: usize, draw_type: DrawType) {
        if draw_type == DrawType::ScrollDown {
            self.d_range = DRange::new(y - Editor::SCROLL_DOWN_EXTRA_NUM - 1, y, draw_type);
        } else {
            self.d_range = DRange::new(y, y + Editor::SCROLL_UP_EXTRA_NUM + 1, draw_type);
        }
    }
    pub fn get_rnw(&self) -> usize {
        return self.rnw;
    }
    pub fn set_org(term: &mut Terminal) {
        let tab = term.tabs.get_mut(term.idx).unwrap();

        tab.editor.cur_y_org = tab.editor.cur.y;
        tab.editor.offset_y_org = tab.editor.offset_y;
        tab.editor.offset_x_org = tab.editor.offset_x;
        tab.editor.rnw_org = tab.editor.get_rnw();
        tab.editor.sel_org = tab.editor.sel;
    }
}
