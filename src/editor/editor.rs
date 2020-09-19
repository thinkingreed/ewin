// 描画処理
use crate::model::{CopyRange, Editor};
use crate::terminal::*;
use clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{Event::Key, Event::Mouse, KeyCode, KeyCode::*, KeyEvent, MouseEvent};
use std::cmp::{max, min};
use std::panic::catch_unwind;

use unicode_width::UnicodeWidthChar;
impl Editor {
    // カーソルが画面に映るようにする
    pub fn scroll(&mut self) {
        let (rows, _) = get_term_disp_size(TermDispType::Editor);

        self.y_offset = min(self.y_offset, self.cur.y);
        if self.cur.y + 1 >= rows {
            self.y_offset = max(self.y_offset, self.cur.y + 1 - rows);
        }
    }
    pub fn scroll_horizontal(&mut self) {
        // Log::ep_s("★ scroll_horizontal");
        self.x_offset_y = self.cur.y;
        let (_, cols) = get_term_disp_size(TermDispType::Editor);
        if self.x_offset_disp + cols < self.cur.disp_x {
            if self.curt_evt == Key(Right.into()) {
                // 次のバイト文字数より大きくx_offset設定
                let (mut sun_w, mut x_offset) = (0, 0);
                let diff = self.cur.disp_x - (self.x_offset_disp + cols);
                for i in self.x_offset..=self.buf[self.x_offset_y].len() {
                    let c = self.buf[self.x_offset_y].get(i).unwrap();
                    let w = c.width().unwrap_or(0);
                    sun_w += w;
                    x_offset += 1;
                    if sun_w >= diff {
                        break;
                    }
                }
                self.x_offset += x_offset;
                self.x_offset_disp += sun_w;
            } else {
                //  Log::ep_s("下の行の1列目からLeftの場合");
                self.x_offset = self.get_x_offset(self.cur.y, self.cur.x - self.lnw);
                let (_, width) = self.get_row_width(self.cur.y, 0, self.x_offset);
                self.x_offset_disp = width;
            }
        // x_offset_dispが減少する場合
        } else if self.cur.disp_x <= self.x_offset_disp + self.lnw {
            if self.curt_evt == Key(Left.into()) {
                //  Log::ep_s("Leftで減少する場合");
                let w = self.get_cur_x_width(self.cur.y);
                self.x_offset_disp -= w;
                self.x_offset -= 1;
            } else {
                //   Log::ep_s("Upで減少する場合");
                self.x_offset = self.get_x_offset(self.cur.y, self.cur.x - self.lnw);
                let (_, width) = self.get_row_width(self.cur.y, 0, self.x_offset);
                self.x_offset_disp = width;
            }
        } else if self.x_offset_disp > 0 {
            match self.curt_evt {
                Key(KeyEvent { code: Up, .. })
                | Key(KeyEvent { code: Down, .. })
                | Mouse(MouseEvent::ScrollUp(_, _, _))
                | Mouse(MouseEvent::ScrollDown(_, _, _)) => {
                    //   Log::ep_s("disp_xの変化が少ない場合");
                    self.x_offset = self.get_x_offset(self.cur.y, self.cur.x - self.lnw);
                    let (_, width) = self.get_row_width(self.cur.y, 0, self.x_offset);
                    self.x_offset_disp = width;
                    // x_offset_disp時の表示の切替の為
                    self.is_all_redraw = true;
                }
                _ => {}
            }
        }
    }

    /// カーソル移動のEventでoffsetの変更有無でエディタ全体、又はカーソルのみの再描画の判定
    pub fn move_cursor(&mut self, key: KeyCode) {
        let y_offset_org: usize = self.y_offset;
        let x_offset_disp_org: usize = self.x_offset_disp;
        match key {
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            _ => {}
        }

        if self.is_all_redraw != true {
            self.is_all_redraw =
                y_offset_org != self.y_offset || x_offset_disp_org != self.x_offset_disp;
        }
    }

    /// updown_xまでのwidthを加算してdisp_xとcursorx算出
    pub fn get_until_updown_x(&mut self) -> (usize, usize) {
        let (mut cursorx, mut width) = (self.lnw, self.lnw);
        for i in 0..self.buf[self.cur.y].len() + 1 {
            if let Some(c) = self.buf[self.cur.y].get(i) {
                let mut c_len = c.width().unwrap_or(0);
                if width + c_len >= self.cur.updown_x {
                    if c_len > 1 {
                        c_len = 1;
                    }
                    width += c_len;
                    break;
                } else {
                    width += c_len;
                }
                cursorx += 1;
            // 最終端の空白の場合
            } else {
                width += 1;
            }
        }
        return (cursorx, width);
    }
    pub fn get_cur_x_width(&mut self, y: usize) -> usize {
        if let Some(c) = self.buf[y].get(self.cur.x - self.lnw) {
            // Log::ep("ccc", c);
            return c.width().unwrap_or(0);
        }
        // 最右端の空白対応
        return 1;
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
    pub fn get_row_width(&mut self, y: usize, sx: usize, ex: usize) -> (usize, usize) {
        let (mut cur_x, mut width) = (0, 0);
        for i in sx..ex {
            if let Some(c) = self.buf[y].get(i) {
                let c_len = c.width().unwrap_or(0);

                width += c_len;
                cur_x += 1;
            } else {
                // 最終端の空白対応
                width += 1;
            }
        }
        return (cur_x, width);
    }

    /// 指定のy・xからx_offsetを取得
    pub fn get_x_offset(&mut self, y: usize, x: usize) -> usize {
        let (mut count, mut width) = (0, 0);
        let (_, cols) = get_term_disp_size(TermDispType::Editor);
        for i in (0..x).rev() {
            let c = self.buf[y].get(i).unwrap();
            width += c.width().unwrap_or(0);
            if width + self.lnw + 1 > cols {
                break;
            }
            count += 1;
        }
        return x - count;
    }

    pub fn get_copy_range(&mut self) -> Vec<CopyRange> {
        let copy_posi = self.sel.get_range();

        let mut copy_ranges: Vec<CopyRange> = vec![];

        Log::ep("copy_posi.sy", copy_posi.sy);
        Log::ep("copy_posi.ey", copy_posi.ey);
        Log::ep("copy_posi.sx", copy_posi.sx);
        Log::ep("copy_posi.ex", copy_posi.ex);
        Log::ep("copy_posi.s_disp_x", copy_posi.s_disp_x);
        Log::ep("copy_posi.e_disp_x", copy_posi.e_disp_x);

        if copy_posi.sy != copy_posi.ey && copy_posi.ex == 0 {}

        for ii in copy_posi.sy..=copy_posi.ey {
            let i = ii as usize;
            // 開始行==終了行
            if copy_posi.sy == copy_posi.ey {
                copy_ranges.push(CopyRange {
                    y: i,
                    sx: copy_posi.sx as usize,
                    ex: copy_posi.ex as usize,
                });
            // 開始行
            } else if i == copy_posi.sy as usize {
                copy_ranges.push(CopyRange {
                    y: i,
                    sx: copy_posi.sx as usize,
                    ex: self.buf[i].len(),
                });
            // 終了行
            } else if i == copy_posi.ey as usize {
                // カーソルが行頭の対応
                copy_ranges.push(CopyRange {
                    y: i,
                    sx: 0,
                    ex: copy_posi.ex,
                });
            // 中間行 全て対象
            } else {
                copy_ranges.push(CopyRange {
                    y: i,
                    sx: 0,
                    ex: self.buf[i].len(),
                });
            }
        }

        return copy_ranges;
    }
    pub fn get_clipboard(&mut self) -> String {
        // WSL環境でpanic発生の回避
        let clipboard_process = catch_unwind(move || {
            let _ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        });
        if clipboard_process.is_ok() {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            return ctx.get_contents().unwrap_or("".to_string());
        } else {
            return self.clipboard.clone();
        }
    }

    pub fn set_clipboard(&mut self, copy_string: String) {
        // WSL環境でpanic発生の回避
        let copy_string___ = copy_string.clone();
        let clipboard_process = catch_unwind(move || {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(copy_string.clone()).unwrap();
            //            Log::ep_s("ClipboardContext");
        });

        if clipboard_process.is_err() {
            self.clipboard = copy_string___.clone();
            //             Log::ep("clipboard", copy_string___);
        }
    }
}
