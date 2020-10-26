use crate::model::{CopyRange, Editor, Log, StatusBar};
use crate::util::*;
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers};
use std::cmp::{max, min};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

impl Editor {
    // カーソルが画面に映るようにする
    pub fn scroll(&mut self) {
        self.y_offset = min(self.y_offset, self.cur.y);

        if self.cur.y + 1 >= self.disp_row_num {
            self.y_offset = max(self.y_offset, self.cur.y + 1 - self.disp_row_num);
        }
    }

    pub fn scroll_horizontal(&mut self) {
        Log::ep_s("★ scroll_horizontal");
        self.x_offset_y = self.cur.y;

        self.x_offset = self.get_x_offset(self.cur.y, self.cur.x - self.lnw);
        let (_, width) = get_row_width(&self.buf[self.cur.y], 0, self.x_offset);
        self.x_offset_disp = width;
    }

    /// カーソル移動のEventでoffsetの変更有無で再描画範囲を設定設定
    pub fn move_cursor<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar) {
        let y_offset_org: usize = self.y_offset;
        let x_offset_disp_org: usize = self.x_offset_disp;
        match self.curt_evt {
            Key(KeyEvent { code, modifiers: KeyModifiers::SHIFT }) => match code {
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
            _ => {}
        }

        if self.is_all_redraw != true {
            self.is_all_redraw = y_offset_org != self.y_offset || (x_offset_disp_org != self.x_offset_disp || (x_offset_disp_org == self.x_offset_disp && self.x_offset_disp != 0));
        }
        self.draw_cur(out, sbar);
    }
    pub fn get_char_width(&mut self, y: usize, x: usize) -> usize {
        Log::ep("self.buf[y].len()", self.buf[y].len());
        Log::ep("xxx", x);

        if self.buf[y].len() >= x {
            if let Some(c) = self.buf[y].get(x - self.lnw) {
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
            let c = self.buf[y].get(i).unwrap();
            width += c.width().unwrap_or(0);
            if width + self.lnw + 1 > self.disp_col_num {
                break;
            }
            count += 1;
        }
        return x - count;
    }

    pub fn get_copy_range(&mut self) -> Vec<CopyRange> {
        let copy_posi = self.sel.get_range();

        let mut copy_ranges: Vec<CopyRange> = vec![];
        if copy_posi.sy == 0 && copy_posi.ey == 0 && copy_posi.ex == 0 {
            return copy_ranges;
        }

        Log::ep("copy_posi.sy", copy_posi.sy);
        Log::ep("copy_posi.ey", copy_posi.ey);
        Log::ep("copy_posi.sx", copy_posi.sx);
        Log::ep("copy_posi.ex", copy_posi.ex);

        for i in copy_posi.sy..=copy_posi.ey {
            /* if copy_posi.sy != copy_posi.ey && copy_posi.ex == 0 {
                continue;
            }*/
            Log::ep("iii", i);
            // 開始行==終了行
            if copy_posi.sy == copy_posi.ey {
                copy_ranges.push(CopyRange { y: i, sx: copy_posi.sx, ex: copy_posi.ex });
            // 開始行
            } else if i == copy_posi.sy {
                Log::ep("i == copy_posi.sy", i == copy_posi.sy);
                copy_ranges.push(CopyRange { y: i, sx: copy_posi.sx, ex: self.buf[i].len() });
            // 終了行
            } else if i == copy_posi.ey {
                // カーソルが行頭の対応
                copy_ranges.push(CopyRange { y: i, sx: 0, ex: copy_posi.ex });
            // 中間行 全て対象
            } else {
                copy_ranges.push(CopyRange { y: i, sx: 0, ex: self.buf[i].len() });
            }
        }

        return copy_ranges;
    }

    pub fn del_sel_range(&mut self) {
        Log::ep_s("★  del_sel_range");
        // s < e の状態に変換した値を使用
        let sel = self.sel.get_range();
        let (sy, ey, sx, ex, s_disp_x, e_disp_x) = (sel.sy, sel.ey, sel.sx, sel.ex, sel.s_disp_x, sel.e_disp_x);

        eprintln!("sel {:?}", sel);

        for i in 0..self.buf.len() {
            if sy <= i && i <= ey {
                // 1行
                if sy == ey {
                    self.buf[i].drain(sx..ex);
                    self.cur.disp_x = min(s_disp_x, e_disp_x);
                    self.cur.x = min(sx, ex) + self.lnw;
                // 開始行
                } else if sy == i {
                    let (cursorx, _) = get_row_width(&self.buf[sy], sx, self.buf[sy].len());
                    self.buf[i].drain(sx..sx + cursorx);
                    self.cur.disp_x = s_disp_x;
                    self.cur.x = sx + self.lnw;

                // 終了行
                } else if ey == i {
                    self.buf[i].drain(0..ex);

                    let mut rest: Vec<char> = self.buf[i].clone();
                    self.buf[sy].append(&mut rest);
                    self.buf.remove(i);
                }
            }
        }

        // 中間行を纏めて削除
        for i in 0..self.buf.len() {
            if sy < i && i < ey {
                self.buf.remove(i);
            }
        }
        self.cur.y = sy;
    }

    pub fn get_sel_range_str(&mut self) -> Vec<String> {
        let mut all_vec: Vec<String> = vec![];
        let copy_ranges: Vec<CopyRange> = self.get_copy_range();

        for copy_range in copy_ranges {
            let mut vec: Vec<char> = vec![];

            for j in copy_range.sx..copy_range.ex {
                if let Some(c) = self.buf[copy_range.y].get(j) {
                    // Log::ep("ccc", c);
                    vec.push(c.clone());
                }
            }
            all_vec.push(vec.iter().collect::<String>());
        }
        eprintln!("get_sel_range_vec {:?}", all_vec);
        return all_vec;
    }
}
