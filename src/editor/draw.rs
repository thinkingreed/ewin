use crate::{def::*, global::*, model::*};
use permissions::*;
use std::cmp::min;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn open(&mut self, path: &path::Path, mbar: &mut MsgBar) {
        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                if !is_writable(path).unwrap() {
                    let msg_1 = &LANG.lock().unwrap().unable_to_edit.clone();
                    let msg_2 = &LANG.lock().unwrap().no_write_permission.clone();
                    mbar.set_readonly(&format!("{}({})", msg_1, msg_2));
                }
            } else {
                println!("{}", LANG.lock().unwrap().file_not_found.clone());
                std::process::exit(1);
            }
        }
        // read
        let result = fs::read_to_string(path);
        match result {
            Ok(mut s) => {
                s = s.replace(NEW_LINE_CRLF, NEW_LINE.to_string().as_str());
                let (mut buf, mut vec) = (vec![], vec![]);
                let (chars, chars_len) = (s.chars(), s.chars().count());
                for (i, mut c) in chars.enumerate() {
                    if c == NEW_LINE {
                        c = NEW_LINE_MARK;
                    }
                    vec.push(c);

                    if c == NEW_LINE_MARK || i == chars_len - 1 {
                        if c == NEW_LINE_MARK && i == chars_len - 1 {
                            buf.push(vec);
                            buf.push(vec![EOF]);
                            break;
                        } else if i == chars_len - 1 {
                            vec.push(EOF);
                        }
                        buf.push(vec);
                        vec = vec![];
                    }
                }
                if buf.is_empty() {
                    self.buf[0] = vec![EOF];
                } else {
                    self.buf = buf;
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    println!("{}", LANG.lock().unwrap().no_read_permission.clone());
                    std::process::exit(1);
                }
                ErrorKind::NotFound => self.buf[0] = vec![EOF],
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
            // 改行EOF対応
            if i < self.buf.len() {
                Log::ep("i < self.buf.len() iii ", i);
                eprintln!("self.buf {:?}", self.buf);
                let x_draw_e = self.buf[i].len();
                for j in x_draw_s..x_draw_e {
                    // highlight
                    self.ctl_color(str_vec, sel_range, &search_ranges, i, j);

                    if let Some(c) = self.buf[i].get(j) {
                        let width = c.width().unwrap_or(0);
                        let x_w_l = x + width + self.rnw;
                        // Log::ep("x_w_l", x_w_l);
                        if x_w_l > cols {
                            break;
                        }
                        x += width;

                        if c == &EOF {
                            self.set_eof(str_vec);
                        } else {
                            str_vec.push(c.to_string());
                        }
                    }
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
        Log::ep("sel", self.sel);
    }

    pub fn draw_cur<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar) {
        Log::ep_s("　　　　　　　  draw_cursor");
        Log::ep("cur.x", self.cur.x);
        Log::ep("cur.disp_x", self.cur.disp_x);

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
