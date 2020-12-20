use crate::model::*;
use crate::util::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseEvent};
use std::cmp::{max, min};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

impl Editor {
    // カーソルが画面に映るようにする
    pub fn scroll(&mut self) {
        // Log::ep_s("　　　　　　　 scroll");
        self.y_offset = min(self.y_offset, self.cur.y);

        if self.cur.y + 1 >= self.disp_row_num {
            self.y_offset = max(self.y_offset, self.cur.y + 1 - self.disp_row_num);
            // y_offsetが減少
            if self.y_offset + self.disp_row_num > self.buf.len() {
                self.y_offset = self.buf.len() - self.disp_row_num;
            }
        }
    }

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

        // Right移動
        if self.x_offset_disp + self.disp_col_num < self.cur.disp_x + offset_x_extra_num {
            // Log::ep_s(" self.cur.x - self.x_offset + extra > self.disp_col_num ");
            if self.x_offset + self.disp_col_num - self.rnw < self.buf[self.cur.y].len() {
                self.x_offset += offset_x_change_num;
            }
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
            let vec = &self.buf[self.cur.y];
            if self.cur_y_org != self.cur.y {
                let (_, width) = get_row_width(vec, 0, self.x_offset);
                self.x_offset_disp = width;

            // offsetに差分
            } else if x_offset_org != self.x_offset {
                if self.x_offset < x_offset_org {
                    let (_, width) = get_row_width(vec, self.x_offset, x_offset_org);
                    self.x_offset_disp -= width;
                } else {
                    let (_, width) = get_row_width(vec, x_offset_org, self.x_offset);
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
                Home => self.ctl_home(),
                End => self.ctl_end(),
                _ => {}
            },
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                F(4) => self.search_str(false),
                _ => {}
            },
            Key(KeyEvent { code, .. }) => match code {
                Up => self.cursor_up(),
                Down => self.cursor_down(),
                Left => self.cursor_left(),
                Right => self.cursor_right(),
                Home => self.home(),
                End => self.end(),
                F(3) => self.search_str(true),
                _ => {}
            },
            Mouse(MouseEvent::ScrollUp(_, _, _)) => self.cursor_up(),
            Mouse(MouseEvent::ScrollDown(_, _, _)) => self.cursor_down(),
            _ => {}
        }

        if self.is_redraw != true {
            // 右記の条件不明な為に一旦コメント x_offset_disp_org == self.x_offset_disp && self.x_offset_disp != 0
            //        self.is_redraw = y_offset_org != self.y_offset || (x_offset_disp_org != self.x_offset_disp || (x_offset_disp_org == self.x_offset_disp && self.x_offset_disp != 0));
            self.is_redraw = y_offset_org != self.y_offset || (x_offset_disp_org != self.x_offset_disp);
        }
        self.draw_cur(out, sbar);
    }
    pub fn get_char_width(&mut self, y: usize, x: usize) -> usize {
        Log::ep("self.buf[y].len()", self.buf[y].len());
        Log::ep("xxx", x);

        if self.buf[y].len() >= x {
            if let Some(c) = self.buf[y].get(x - self.rnw) {
                return c.width().unwrap_or(0);
            }
        }
        // 最右端の空白対応
        return 1;
    }

    /// 指定のy・xからx_offsetを取得
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut count, mut width) = (0, 0);
        for i in (0..x).rev() {
            if let Some(c) = self.buf[y].get(i) {
                //Log::ep("ccccc", c);
                width += c.width().unwrap_or(0);
                //Log::ep("width", width);
                if width + self.rnw + 1 > self.disp_col_num {
                    break;
                }
                count += 1;
            // 行終端の空白
            } else {
                count += 1;
            }
        }
        return x - count;
    }

    pub fn del_sel_range(&mut self) {
        Log::ep_s("　　　　　　　  del_sel_range");
        let sel = self.sel.get_range();
        let (sy, ey, sx, ex, s_disp_x) = (sel.sy, sel.ey, sel.sx, sel.ex, sel.s_disp_x);
        Log::ep("sel", sel);

        for i in 0..self.buf.len() {
            if sy <= i && i <= ey {
                // one line
                if sy == ey {
                    self.buf[i].drain(sx..ex);
                // start line
                } else if sy == i {
                    let (cursorx, _) = get_row_width(&self.buf[sy], sx, self.buf[sy].len());
                    self.buf[i].drain(sx..sx + cursorx);

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
                Log::ep("sy == i && self.buf[sy].len() == 0 || sy < i && i <= ey", i);
                self.buf.remove(i);
            }
        }

        self.cur.y = sy;
        self.rnw = self.buf.len().to_string().len();
        self.cur.disp_x = s_disp_x;
        self.cur.x = sx + self.rnw;
    }
}
