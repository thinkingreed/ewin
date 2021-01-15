use crate::{def::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::cmp::{max, min};
use unicode_width::UnicodeWidthChar;

impl Editor {
    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        // Log::ep_s("　　　　　　　 scroll");
        self.offset_y = min(self.offset_y, self.cur.y);

        if self.cur.y + 1 >= self.disp_row_num {
            if self.evt == PAGE_DOWN {
                if self.cur.y >= self.offset_y + self.disp_row_num {
                    self.offset_y = self.cur.y;
                }
            } else {
                self.offset_y = max(self.offset_y, self.cur.y + 1 - self.disp_row_num);
                // y_offsetが減少
                if self.offset_y + self.disp_row_num > self.buf.len_lines() {
                    self.offset_y = self.buf.len_lines() - self.disp_row_num;
                }
            }
        }
    }

    // adjusting horizontal posi of cursor
    pub fn scroll_horizontal(&mut self) {
        // Log::ep_s("　　　　　　　 scroll_horizontal");
        // offset_x切替余分文字数(残文字数時にoffset切替)
        let offset_x_extra_num = 3;
        // 上記offset切替時のoffset増減数
        let offset_x_change_num = 10;
        let x_offset_org = self.offset_x;

        // Up・Down
        if self.rnw == self.cur.x {
            self.offset_x = 0;
            self.offset_disp_x = 0;
        } else {
            if self.offset_x == 0 || self.cur_y_org != self.cur.y {
                self.offset_x = self.get_x_offset(self.cur.y, self.cur.x - self.rnw);
            }
        }

        // line disp_x < disp_col_num disable extra
        let mut line_disp_x = 0;
        if self.buf.len_line_chars(self.cur.y) < self.disp_col_num {
            let (_, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], true);
            line_disp_x = width;
        }

        // Right移動
        if line_disp_x > self.disp_col_num && self.offset_disp_x + self.disp_col_num < self.cur.disp_x + offset_x_extra_num {
            // Log::ep_s(" self.cur.x - self.x_offset + extra > self.disp_col_num ");
            self.offset_x += offset_x_change_num;
        //  }
        // Left移動
        } else if self.cur.disp_x - 1 >= self.rnw + offset_x_extra_num && self.offset_disp_x >= self.cur.disp_x - 1 - self.rnw - offset_x_extra_num {
            // Log::ep_s(" self.x_offset + self.rnw + extra > self.cur.x ");
            if self.offset_x >= offset_x_change_num {
                self.offset_x -= offset_x_change_num;
            } else {
                self.offset_x = 0;
            }
        }

        if self.rnw != self.cur.x {
            let vec = &self.buf.char_vec_line(self.cur.y);
            if self.cur_y_org != self.cur.y {
                // Log::ep_s(" self.cur_y_org != self.cur.y ");

                let (_, width) = get_row_width(&vec[..self.offset_x], false);
                self.offset_disp_x = width;

            // offsetに差分
            } else if x_offset_org != self.offset_x {
                if self.offset_x < x_offset_org {
                    // Log::ep_s(" self.x_offset < x_offset_org  ");
                    let (_, width) = get_row_width(&vec[self.offset_x..x_offset_org], false);
                    self.offset_disp_x -= width;
                } else {
                    // Log::ep_s("else self.x_offset < x_offset_org  ");

                    let (_, width) = get_row_width(&vec[x_offset_org..self.offset_x], false);
                    self.offset_disp_x += width;
                }
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

    /// 指定のy・xからx_offsetを取得
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut count, mut width) = (0, 0);
        for i in (0..x).rev() {
            let c = self.buf.char(y, i);
            // Log::ep("ccccc", c);
            width += c.width().unwrap_or(0);
            //Log::ep("width", width);
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
    }

    pub fn set_cur_default(&mut self) {
        self.rnw = self.buf.len_lines().to_string().len();
        self.cur = Cur { y: 0, x: self.rnw, disp_x: self.rnw + 1 };
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_cur_target(&mut self, y: usize, x: usize) {
        self.cur.y = y;
        self.rnw = self.buf.len_lines().to_string().len();
        let (cur_x, width) = get_row_width(&self.buf.char_vec_range(y, x), false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn is_edit_evt(&self, is_incl_unredo: bool) -> bool {
        if is_incl_unredo {
            match self.evt {
                PASTE | UNDO | REDO | DEL | BS | CUT | ENTER | Key(KeyEvent { code: Char(_), modifiers: KeyModifiers::NONE }) | Key(KeyEvent { code: Char(_), modifiers: KeyModifiers::SHIFT }) => return true,
                _ => return false,
            }
        } else {
            match self.evt {
                PASTE | DEL | BS | CUT | ENTER | Key(KeyEvent { code: Char(_), modifiers: KeyModifiers::NONE }) | Key(KeyEvent { code: Char(_), modifiers: KeyModifiers::SHIFT }) => return true,
                _ => return false,
            }
        }
    }
}
