use crate::model::*;
use crate::util::*;
use std::cmp::min;
use std::fs;
use std::io::Write;
use std::path;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Editor {
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
            .unwrap_or_else(|| vec![Vec::new()]);

        self.path = Some(path.into());
        self.lnw = self.buf.len().to_string().len();
        self.cur = Cur { y: 0, x: self.lnw, disp_x: 0 };
        self.cur.disp_x = self.lnw + get_cur_x_width(&self.buf[self.cur.y], self.cur.x - self.lnw);
    }
    pub fn draw<T: Write>(&mut self, out: &mut T) {
        let (rows, cols) = (self.disp_row_num, self.disp_col_num);
        let str_vec: &mut Vec<String> = &mut vec![];

        let mut y_draw_s = self.y_offset;
        let mut y_draw_e = self.y_offset + min(self.disp_row_num, self.buf.len());

        let d_range = self.d_range.get_range();

        if d_range.d_type == DType::Not {
            return;
        } else if d_range.d_type == DType::None || d_range.d_type == DType::All {
            str_vec.push(clear::All.to_string());
            str_vec.push(cursor::Goto(1, 1).to_string());
        } else {
            y_draw_s = d_range.sy;
            if d_range.d_type == DType::Target {
                for i in d_range.sy - self.y_offset..=d_range.ey - self.y_offset {
                    str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                }
                str_vec.push(cursor::Goto(1, (d_range.sy + 1 - self.y_offset) as u16).to_string());
                y_draw_e = d_range.ey + 1;
            } else {
                Log::ep_s("clear::AfterCursor");
                str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.y_offset) as u16), clear::AfterCursor));
            }
        }
        // 画面上の行、列
        let mut y = 0;
        let mut x = 0;
        // let rowlen =
        self.lnw = self.buf.len().to_string().len();
        let sel_range = self.sel.get_range();
        let search_ranges = self.search.search_ranges.clone();

        for i in y_draw_s..y_draw_e {
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

                // 選択箇所のhighlight
                self.ctl_select_color(str_vec, sel_range, i, j);
                &self.ctl_search_color(str_vec, &search_ranges, i, j);

                if let Some(c) = &self.buf[i].get(j) {
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
                    // 検索対象のhighlight
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
        Log::ep("cur.x", self.cur.x);
        Log::ep("cur.disp_x", self.cur.disp_x);
        Log::ep("x_offset", self.x_offset);

        write!(out, "{}", &str_vec.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar) {
        Log::ep_s("★  draw_cursor");

        let str_vec: &mut Vec<String> = &mut vec![];
        sbar.draw_cur(str_vec, self);
        str_vec.push(cursor::Goto((self.cur.disp_x - self.x_offset_disp) as u16, (self.cur.y + 1 - self.y_offset) as u16).to_string());
        write!(out, "{}", str_vec.concat()).unwrap();
        out.flush().unwrap();
    }

    pub fn set_sel_del_d_range(&mut self) {
        let sel = self.sel.get_range();
        self.d_range = DRnage { sy: sel.sy, ey: sel.ey, d_type: DType::After };
    }
}
