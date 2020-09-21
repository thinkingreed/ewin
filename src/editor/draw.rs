use crate::model::{Cursor, Editor, SelectRange, StatusBar};
use crate::terminal::Log;
use crate::terminal::*;
use std::fs;
use std::io::{self, Write};
use std::path;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Editor {
    // ファイルを読み込む
    pub fn open(&mut self, path: &path::Path) {
        self.buf = fs::read_to_string(path)
            .ok()
            .map(|s| {
                let buffer: Vec<Vec<char>> = s.lines().map(|line| line.trim_end().chars().collect()).collect();
                if buffer.is_empty() {
                    vec![Vec::new()]
                } else {
                    buffer
                }
            })
            .expect("Error");
        // .unwrap_or_else(|| vec![Vec::new()]);

        self.path = Some(path.into());
        self.lnw = self.buf.len().to_string().len();
        self.cur = Cursor {
            y: 0,
            x: self.lnw,
            disp_x: 0,
            updown_x: 0,
        };
        self.cur.disp_x = self.lnw + self.get_cur_x_width(self.cur.y);
    }
    pub fn draw(&mut self, str_vec: &mut Vec<String>) -> Result<(), io::Error> {
        let (rows, cols) = get_term_disp_size(TermDispType::Editor);

        str_vec.push(clear::All.to_string());
        str_vec.push(cursor::Goto(1, 1).to_string());
        // 画面上の行、列
        let mut y = 0;
        let mut x = 0;
        // let rowlen =
        self.lnw = self.buf.len().to_string().len();
        let sel_range = self.sel.get_range();

        for i in self.y_offset..self.buf.len() {
            self.set_rownum_color(str_vec);
            // 行番号の空白
            if self.x_offset_y == i && self.x_offset_disp > 0 {
                for _ in 0..self.lnw {
                    str_vec.push(">".to_string());
                }
            } else {
                for _ in (i + 1).to_string().len()..self.lnw {
                    str_vec.push(" ".to_string());
                }
                str_vec.push((i + 1).to_string());
            }
            self.set_textarea_color(str_vec);
            for j in 0..=self.buf[i].len() + 1 {
                if self.buf[i].len() == 0 {
                    break;
                }

                // 選択箇所のハイライト
                self.ctl_select_color(str_vec, sel_range, i, j);
                if let Some(c) = self.buf[i].get(j) {
                    let width = c.width().unwrap_or(0);
                    if i == self.x_offset_y && x + width <= self.x_offset_disp {
                        x += width;
                        continue;
                    }
                    let x_w_l = x + width + self.lnw;
                    if i == self.x_offset_y {
                        if x_w_l - self.x_offset_disp > cols {
                            break;
                        }
                    } else {
                        if x_w_l > cols {
                            break;
                        }
                    }
                    x += width;
                    str_vec.push(c.to_string());
                }
            }
            y += 1;
            x = 0;
            if y >= rows {
                break;
            } else {
                str_vec.push("\r\n".to_string());
            }
        }
        // 色をデフォルトに戻す
        //self.set_textarea_color(str_vec);
        for _ in self.buf.len() + 1..rows {
            str_vec.push("\r\n".to_string());
        }
        return Ok(());
    }
    /// 選択箇所のハイライト
    pub fn ctl_select_color(&mut self, str_vec: &mut Vec<String>, ranges: SelectRange, y: usize, _x: usize) {
        if ranges.sy == 0 && ranges.s_disp_x == 0 {
            return;
        }

        // １行または下に複数行選択
        if ranges.sy <= y && y <= ranges.ey {
            let (_, width) = self.get_row_width(y, 0, _x);
            // １行または下に複数行選択
            let x = width + self.lnw + 1;
            // 開始・終了が同じ行
            if ranges.sy == ranges.ey {
                if ranges.s_disp_x <= x && x < ranges.e_disp_x {
                    self.set_select_color(str_vec);
                } else {
                    self.set_textarea_color(str_vec)
                }
            // 開始行
            } else if ranges.sy == y && ranges.s_disp_x <= x {
                self.set_select_color(str_vec);
            // 終了行
            } else if ranges.ey == y && x < ranges.e_disp_x {
                self.set_select_color(str_vec);
            // 中間行
            } else if ranges.sy < y && y < ranges.ey {
                self.set_select_color(str_vec);
            } else {
                self.set_textarea_color(str_vec)
            }
        } else {
            self.set_textarea_color(str_vec)
        }
    }
    pub fn draw_cursor<T: Write>(&mut self, out: &mut T, sber: &mut StatusBar) -> Result<(), io::Error> {
        Log::ep_s("★  draw_cursor");
        Log::ep("disp_x", self.cur.disp_x);

        let str_vec: &mut Vec<String> = &mut vec![];

        sber.draw_statusber(str_vec, self);
        let cur_str = format!("{}", cursor::Goto((self.cur.disp_x - self.x_offset_disp) as u16, (self.cur.y + 1 - self.y_offset) as u16));
        str_vec.push(cur_str);
        write!(out, "{}", str_vec.concat())?;
        out.flush()?;
        return Ok(());
    }
    pub fn set_cur_str(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("★  set_cur_str");
        Log::ep("cur.x", self.cur.x);
        Log::ep("sel.sx", self.sel.sx);
        Log::ep("sel.ex", self.sel.ex);
        Log::ep("sel.s_disp_x", self.sel.s_disp_x);
        Log::ep("sel.e_disp_x", self.sel.e_disp_x);

        Log::ep("disp_x", self.cur.disp_x);
        str_vec.push(cursor::Goto((self.cur.disp_x - self.x_offset_disp) as u16, (self.cur.y + 1 - self.y_offset) as u16).to_string());
    }
}
