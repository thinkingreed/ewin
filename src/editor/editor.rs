use crate::{def::*, model::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent as M_Event, MouseEventKind as M_Kind};
use std::cmp::{max, min};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

impl Editor {
    // adjusting vertical posi of cursor
    pub fn scroll(&mut self) {
        // Log::ep_s("　　　　　　　 scroll");
        self.y_offset = min(self.y_offset, self.cur.y);

        if self.cur.y + 1 >= self.disp_row_num {
            if self.evt == PAGE_DOWN {
                if self.cur.y >= self.y_offset + self.disp_row_num {
                    self.y_offset = self.cur.y;
                }
            } else {
                self.y_offset = max(self.y_offset, self.cur.y + 1 - self.disp_row_num);
                // y_offsetが減少
                if self.y_offset + self.disp_row_num > self.t_buf.lines().len() {
                    self.y_offset = self.t_buf.lines().len() - self.disp_row_num;
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
        let x_offset_org = self.x_offset;

        // Up・Down
        if self.rnw == self.cur.x {
            self.x_offset = 0;
            self.x_offset_disp = 0;
        } else {
            if self.x_offset == 0 || self.cur_y_org != self.cur.y {
                self.x_offset = self.get_x_offset(self.cur.y, self.cur.x - self.rnw);
            }
        }

        // line disp_x < disp_col_num disable extra
        let mut line_disp_x = 0;
        if self.t_buf.line_len(self.cur.y) < self.disp_col_num {
            let (_, width) = get_row_width(&self.t_buf.char_vec(self.cur.y), 0, self.t_buf.line_len(self.cur.y), true);
            line_disp_x = width;
        }

        // Right移動
        if line_disp_x > self.disp_col_num && self.x_offset_disp + self.disp_col_num < self.cur.disp_x + offset_x_extra_num {
            // Log::ep_s(" self.cur.x - self.x_offset + extra > self.disp_col_num ");
            self.x_offset += offset_x_change_num;
        //  }
        // Left移動
        } else if self.cur.disp_x - 1 >= self.rnw + offset_x_extra_num && self.x_offset_disp >= self.cur.disp_x - 1 - self.rnw - offset_x_extra_num {
            // Log::ep_s(" self.x_offset + self.rnw + extra > self.cur.x ");
            if self.x_offset >= offset_x_change_num {
                self.x_offset -= offset_x_change_num;
            } else {
                self.x_offset = 0;
            }
        }

        if self.rnw != self.cur.x {
            let vec = &self.t_buf.char_vec(self.cur.y);
            if self.cur_y_org != self.cur.y {
                // Log::ep_s(" self.cur_y_org != self.cur.y ");

                let (_, width) = get_row_width(vec, 0, self.x_offset, false);
                self.x_offset_disp = width;

            // offsetに差分
            } else if x_offset_org != self.x_offset {
                if self.x_offset < x_offset_org {
                    // Log::ep_s(" self.x_offset < x_offset_org  ");
                    let (_, width) = get_row_width(vec, self.x_offset, x_offset_org, false);
                    self.x_offset_disp -= width;
                } else {
                    // Log::ep_s("else self.x_offset < x_offset_org  ");

                    let (_, width) = get_row_width(vec, x_offset_org, self.x_offset, false);
                    self.x_offset_disp += width;
                }
            }
        }
    }

    /// カーソル移動のEventでoffsetの変更有無で再描画範囲を設定設定
    pub fn move_cursor<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar) {
        let y_offset_org: usize = self.y_offset;
        let x_offset_disp_org: usize = self.x_offset_disp;
        self.cur_y_org = self.cur.y;
        match self.evt {
            Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code }) => match code {
                Home => self.ctrl_home(),
                End => self.ctrl_end(),
                _ => {}
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => self.search_str(false),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Up => self.cur_up(),
                Down => self.cur_down(),
                Left => self.cur_left(),
                Right => self.cur_right(),
                Home => self.home(),
                End => self.end(),
                F(3) => self.search_str(true),
                _ => {}
            },
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => self.cur_up(),
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => self.cur_down(),
            _ => {}
        }

        if self.is_redraw != true {
            // 右記の条件不明な為に一旦コメント x_offset_disp_org == self.x_offset_disp && self.x_offset_disp != 0
            //        self.is_redraw = y_offset_org != self.y_offset || (x_offset_disp_org != self.x_offset_disp || (x_offset_disp_org == self.x_offset_disp && self.x_offset_disp != 0));
            self.is_redraw = y_offset_org != self.y_offset || x_offset_disp_org != self.x_offset_disp;
            if self.is_redraw {
                self.d_range.d_type = DType::All;

                /*
                if x_offset_disp_org != self.x_offset_disp {
                    self.d_range.d_type = DType::Target;
                } else {
                    self.d_range.d_type = DType::All;
                }*/
            }
        }
        self.draw_cur(out, sbar);
    }
    pub fn get_char_width(&mut self, y: usize, x: usize) -> usize {
        Log::ep("xxx", x);

        if self.t_buf.line_len(y) >= x {
            let c = self.t_buf.char(y, x - self.rnw);
            return c.width().unwrap_or(0);
        }
        return 0;
    }

    /// 指定のy・xからx_offsetを取得
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut count, mut width) = (0, 0);
        for i in (0..x).rev() {
            let c = self.t_buf.char(y, i);
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
        Log::ep_s("　　　　　　　  del_sel_range");
        let sel = self.sel.get_range();
        let (sy, ey, sx, ex) = (sel.sy, sel.ey, sel.sx, sel.ex);
        Log::ep("sel", sel);

        for i in 0..self.t_buf.lines().len() {
            if sy <= i && i <= ey {
                // one line
                if sy == ey {
                    if self.buf[i][ex - 1] == NEW_LINE {
                        self.del_end_of_line_new_line(self.cur.y);
                    }
                    self.buf[i].drain(sx..ex);
                // start line
                } else if sy == i {
                    let (cur_x, _) = get_row_width(&self.buf[sy], sx, self.buf[sy].len(), true);
                    self.buf[i].drain(sx..sx + cur_x);

                // end line
                } else if ey == i {
                    self.buf[i].drain(0..ex);
                    let mut rest: Vec<char> = self.buf[i].clone();
                    self.buf[sy].append(&mut rest);
                }
            }
        }
        // delete row
        for i in (0..self.buf.len()).rev() {
            if sy == i && self.buf[sy].len() == 0 || sy < i && i <= ey {
                self.buf.remove(i);
            }
        }
        self.cur.y = sy;
        self.rnw = self.t_buf.lines().len().to_string().len();
        let (cur_x, width) = get_row_width(&self.buf[sy], 0, sx, false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
    }

    pub fn del_end_of_line_new_line(&mut self, remove_y: usize) {
        Log::ep_s("　　　　　　　  del_end_of_line_new_line");
        let mut bottom_line: Vec<char> = self.buf.remove(remove_y);
        self.buf[remove_y - 1].append(&mut bottom_line);
    }
}
