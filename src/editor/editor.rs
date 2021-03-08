use crate::{def::*, log::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent as M_Event, MouseEventKind as M_Kind};
use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;

impl Editor {
    const SCROLL_UP_EXTRA_NUM: usize = 1;
    const SCROLL_DOWN_EXTRA_NUM: usize = 1;
    const SCROLL_MOVE_EXTRA_NUM: usize = 3;

    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        Log::ep_s("　　　　　　　 scroll");

        if self.evt == CTRL_HOME {
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
                self.offset_y = if self.buf.len_lines() - 1 > self.offset_y + self.disp_row_num * 2 {
                    self.offset_y + self.disp_row_num
                } else {
                    self.buf.len_lines() - self.disp_row_num
                };
            } else {
                self.offset_y = max(self.offset_y, self.cur.y + Editor::SCROLL_DOWN_EXTRA_NUM + 1 - self.disp_row_num);
                // offset_y decreases
                if self.offset_y + self.disp_row_num > self.buf.len_lines() {
                    self.offset_y = self.buf.len_lines() - self.disp_row_num;
                }
            }
        }
        Log::ep("self.offset_y", &self.offset_y);
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
        Log::ep_s("　　　　　　　 scroll_horizontal");

        Log::ep("self.evt", &self.evt);

        // offset_x Number of characters for switching judgment
        let offset_x_extra_num = 3;
        // Number of offset increase / decrease when switching the above offset
        let offset_x_change_num = 10;

        let offset_x_org = self.offset_x;
        let vec = &self.buf.char_vec_line(self.cur.y);

        // Calc offset_x
        // Up・Down・Home ...
        if self.rnw == self.cur.x {
            self.offset_x = 0;
        // KEY_NULL:grep_result initial display
        } else if self.cur_y_org != self.cur.y || self.evt == END || self.evt == SEARCH || self.evt == SEARCH_DESC || self.evt == KEY_NULL {
            self.offset_x = self.get_x_offset(self.cur.y, self.cur.x - self.rnw);

            if self.evt == SEARCH || self.evt == SEARCH_DESC || self.evt == KEY_NULL {
                let str_width = get_str_width(&self.search.str);
                if self.evt == SEARCH || self.evt == KEY_NULL {
                    // Offset setting to display a few characters to the right of the search character for easier viewing
                    if self.cur.disp_x + str_width + 5 > self.offset_disp_x + self.disp_col_num {
                        self.offset_x += str_width + 5;
                    }
                } else if self.evt == SEARCH_DESC {
                    // Calc offset_disp_x once to judge the display position
                    let offset_disp_x = get_row_width(&vec[..self.offset_x], false).1;
                    if self.cur.disp_x + str_width + 5 > offset_disp_x + self.disp_col_num {
                        self.offset_x += str_width + 5;
                    }
                }
            }
        // cur_right
        } else if self.evt == RIGHT && self.offset_disp_x + self.disp_col_num < self.cur.disp_x + offset_x_extra_num {
            let width = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], true).1;
            // +1 for EOF
            if width + 1 > self.disp_col_num - self.rnw {
                self.offset_x += offset_x_change_num;
            }

        // cur_left
        } else if self.evt == LEFT && self.cur.disp_x - 1 >= self.rnw + offset_x_extra_num && self.offset_disp_x >= self.cur.disp_x - 1 - self.rnw - offset_x_extra_num {
            Log::ep_s(" self.x_offset + self.rnw + extra > self.cur.x ");
            self.offset_x = if self.offset_x >= offset_x_change_num { self.offset_x - offset_x_change_num } else { 0 };
        }

        // Calc offset_disp_x
        if self.cur_y_org != self.cur.y {
            Log::ep_s(" self.cur_y_org != self.cur.y ");
            self.offset_disp_x = get_row_width(&vec[..self.offset_x], false).1;
        } else if offset_x_org != self.offset_x {
            if self.offset_x < offset_x_org {
                // Log::ep_s(" self.x_offset < x_offset_org  ");

                let ttt = get_row_width(&vec[self.offset_x..offset_x_org], false).1;
                Log::ep(" ttt", &ttt);
                self.offset_disp_x -= get_row_width(&vec[self.offset_x..offset_x_org], false).1;
            } else {
                // Log::ep_s("else self.x_offset < x_offset_org  ");
                self.offset_disp_x += get_row_width(&vec[offset_x_org..self.offset_x], false).1;
            }
        }
    }

    pub fn get_char_width(&mut self, y: usize, x: usize) -> usize {
        if self.buf.len_line_chars(y) >= x {
            let c = self.buf.char(y, x - self.rnw);
            return c.width().unwrap_or(0);
        }
        return 0;
    }

    /// Get x_offset from the specified y・x
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut count, mut width) = (0, 0);
        for i in (0..x).rev() {
            let c = self.buf.char(y, i);
            // Log::ep("ccccc", c);
            width += c.width().unwrap_or(0);
            // Log::ep("width", width);
            if width + self.rnw + 1 > self.disp_col_num {
                break;
            }
            count += 1;
        }
        return x - count;
    }

    pub fn del_sel_range(&mut self) {
        let sel = self.sel.get_range();
        self.buf.remove_range(sel);
        self.set_cur_target(sel.sy, sel.sx);
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_cur_default(&mut self) {
        self.rnw = self.buf.len_lines().to_string().len();
        self.cur = Cur { y: 0, x: self.rnw, disp_x: self.rnw + 1 };
    }

    pub fn set_cur_target(&mut self, y: usize, x: usize) {
        self.cur.y = y;
        self.rnw = self.buf.len_lines().to_string().len();
        let (cur_x, width) = get_row_width(&self.buf.char_vec_range(y, x), false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
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
    pub fn set_draw_range(&mut self, curt_y_org: usize, offset_y_org: usize, offset_x_org: usize, rnw_org: usize) {
        Log::ep_s("set_draw_range");
        Log::ep("self.d_range", &self.d_range);

        if (self.offset_x > 0 && curt_y_org != self.cur.y) || offset_x_org != self.offset_x {
            self.d_range = DRange::new(min(curt_y_org, self.cur.y), max(curt_y_org, self.cur.y), DrawType::Target);
        }
        if rnw_org != self.rnw {
            self.d_range.draw_type = DrawType::All;
        }
        if offset_y_org != self.offset_y {
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
        Log::ep("self.d_range", &self.d_range);
    }

    pub fn set_draw_range_scroll(&mut self, y: usize, draw_type: DrawType) {
        self.d_range = DRange::new(y, y, draw_type);

        if !self.sel.is_selected() {
            self.d_range = DRange::new(y, y, draw_type);
        }
    }
}
