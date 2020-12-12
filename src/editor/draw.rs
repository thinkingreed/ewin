use crate::global::*;
use crate::model::*;
use std::cmp::min;
use std::fs;
use std::io::ErrorKind;
use std::io::Write;
use std::path;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn open(&mut self, path: &path::Path, mbar: &mut MsgBar) {
        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                let metadata = fs::metadata(path).unwrap();
                if metadata.permissions().readonly() {
                    mbar.set_readonly(&LANG.lock().unwrap().unable_to_edit.clone());
                }
            } else {
                println!("{}", LANG.lock().unwrap().file_not_found.clone());
                std::process::exit(1);
            }
        }

        let result = fs::read_to_string(path);

        match result {
            Ok(s) => {
                let buffer: Vec<Vec<char>> = s.lines().map(|line| line.trim_end().chars().collect()).collect();
                if buffer.is_empty() {
                    self.buf = vec![Vec::new()]
                } else {
                    self.buf = buffer
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    println!("{}", LANG.lock().unwrap().no_read_permission.clone());
                    std::process::exit(1);
                }
                ErrorKind::NotFound => self.buf = vec![Vec::new()],
                _ => {
                    println!("{} {:?}", LANG.lock().unwrap().file_opening_problem, err);
                    std::process::exit(1);
                }
            },
        }
        self.path = Some(path.into());
        self.rnw = self.buf.len().to_string().len();
        self.set_cur_default();
    }
    pub fn set_cur_default(&mut self) {
        self.cur = Cur { y: 0, x: self.rnw, disp_x: self.rnw + 1 };
        self.scroll();
        self.scroll_horizontal();
    }
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        let (rows, cols) = (self.disp_row_num, self.disp_col_num);
        Log::ep("rows", rows);
        Log::ep("cols", cols);

        let mut y_draw_s = self.y_offset;
        //let mut y_draw_e = self.y_offset + min(self.disp_row_num, self.buf.len());
        let mut y_draw_e = min(self.buf.len(), self.y_offset + min(self.disp_row_num, self.buf.len()));

        let d_range = self.d_range.get_range();
        Log::ep("d_range", d_range);

        if d_range.d_type == DType::Not {
            return;
        } else if d_range.d_type == DType::None || d_range.d_type == DType::All {
            Colors::set_textarea_color(str_vec);
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
                str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.y_offset) as u16), clear::AfterCursor));
            }
        }
        // 画面上の行、列
        let mut y = 0;
        let mut x = 0;
        // let rowlen =
        self.rnw = self.buf.len().to_string().len();
        let sel_range = self.sel.get_range();
        let search_ranges = self.search.search_ranges.clone();

        Log::ep("y_draw_e 222", y_draw_e);

        for i in y_draw_s..y_draw_e {
            Colors::set_rownum_color(str_vec);
            // 行番号の空白
            if self.cur.y == i && self.x_offset_disp > 0 {
                for _ in 0..self.rnw {
                    str_vec.push(">".to_string());
                }
            } else {
                for _ in (i + 1).to_string().len()..self.rnw {
                    str_vec.push(" ".to_string());
                }
                str_vec.push((i + 1).to_string());
            }
            Colors::set_textarea_color(str_vec);

            let mut x_draw_s = 0;

            if i == self.cur.y {
                x_draw_s = self.x_offset;
                Log::ep("x_draw_s", x_draw_s);
            }
            let x_draw_e = self.buf[i].len() + 1;
            for j in x_draw_s..=x_draw_e {
                if self.buf[i].len() == 0 {
                    break;
                }

                // 選択箇所のhighlight
                self.ctl_select_color(str_vec, sel_range, i, j);
                &self.ctl_search_color(str_vec, &search_ranges, i, j);

                if let Some(c) = &self.buf[i].get(j) {
                    let width = c.width().unwrap_or(0);
                    let x_w_l = x + width + self.rnw;
                    // Log::ep("x_w_l", x_w_l);
                    if x_w_l > cols {
                        break;
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
        Log::ep("cur.y", self.cur.y);
        Log::ep("y_offset", self.y_offset);
        Log::ep("cur.x", self.cur.x);
        Log::ep("cur.disp_x", self.cur.disp_x);
        Log::ep("x_offset", self.x_offset);
        Log::ep("x_offset_disp", self.x_offset_disp);
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar) {
        Log::ep_s("　　　　　　　  draw_cursor");

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
