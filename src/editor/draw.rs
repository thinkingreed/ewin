use crate::{def::*, global::*, model::*, util::*};
use permissions::*;
use ropey::iter::Chars;
use std::cmp::min;
use std::env;
use std::io::ErrorKind;
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
        } else {
            let current = env::current_dir().unwrap();
            if !is_writable(current).unwrap() {
                println!("{}", LANG.lock().unwrap().no_write_permission.clone());
                std::process::exit(1);
            }
        }
        // read
        let result = TextBuffer::from_path(&path.to_string_lossy().to_string());
        match result {
            Ok(t_buf) => {
                // search_and_replace(&mut t_buf.text, NEW_LINE_CRLF, NEW_LINE_CR.to_string().as_str());
                self.buf = t_buf;
                self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK);
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    println!("{}", LANG.lock().unwrap().no_read_permission.clone());
                    std::process::exit(1);
                }
                ErrorKind::NotFound => self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK),
                _ => {
                    println!("{} {:?}", LANG.lock().unwrap().file_opening_problem, err);
                    std::process::exit(1);
                }
            },
        }

        self.path = Some(path.into());
        self.set_cur_default();
    }

    pub fn set_cur_default(&mut self) {
        self.rnw = self.buf.len_lines().to_string().len();
        self.cur = Cur { y: 0, x: self.rnw, disp_x: self.rnw + 1 };
        self.scroll();
        self.scroll_horizontal();
    }

    pub fn set_cur_end_x(&mut self, y: usize) {
        self.rnw = self.buf.len_lines().to_string().len();
        let (cur_x, width) = get_row_width(&self.buf.char_vec(y)[..], false);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
        self.scroll();
        self.scroll_horizontal();
    }

    //    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw_vec: Vec<Vec<char>>) {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        let (rows, cols) = (self.disp_row_num, self.disp_col_num);
        Log::ep("rows cols", format!("{} {}", rows, cols));

        let mut y_draw_s = self.offset_y;
        let mut y_draw_e = min(self.buf.len_lines(), self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();

        if d_range.d_type == DType::Not {
            return;
        } else if d_range.d_type == DType::None || d_range.d_type == DType::All {
            str_vec.push(clear::All.to_string());
            str_vec.push(cursor::Goto(1, 1).to_string());
        } else {
            y_draw_s = d_range.sy;
            if d_range.d_type == DType::Target {
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                }
                str_vec.push(cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16).to_string());
                y_draw_e = d_range.ey + 1;
            } else if d_range.d_type == DType::After {
                str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16), clear::AfterCursor));
            }
        }

        // 画面上の行、列
        let mut y = 0;
        let mut x = 0;

        self.rnw = self.buf.len_lines().to_string().len();
        let sel_range = self.sel.get_range();
        let search_ranges = self.search.search_ranges.clone();

        for i in y_draw_s..y_draw_e {
            Colors::set_rownum_color(str_vec);
            // 行番号の空白
            if self.cur.y == i && self.offset_disp_x > 0 {
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

            // let line_vec = &draw_vec[i - y_draw_s];
            let x_draw_s = if i == self.cur.y { self.offset_x } else { 0 };
            let x_draw_e = min(x_draw_s + cols, self.buf.len_line(i));

            let line_idx = self.buf.line_to_char(i);
            let mut line_char_idx = 0;
            for j in x_draw_s..x_draw_e {
                let c = &self.buf.char_idx(line_idx + line_char_idx);

                // let c = &line_vec[j];
                // highlight
                self.ctl_color(str_vec, sel_range, &search_ranges, i, j);

                let mut width = c.width().unwrap_or(0);
                if c == &NEW_LINE {
                    width = 1;
                }

                let x_w_l = x + width + self.rnw;
                if x_w_l > cols {
                    break;
                }
                x += width;
                line_char_idx += 1;

                if c == &EOF_MARK {
                    self.set_eof(str_vec);
                } else if c == &NEW_LINE_CR {
                } else if c == &NEW_LINE {
                    str_vec.push(NEW_LINE_MARK.to_string());
                } else {
                    str_vec.push(c.to_string());
                }
            }
            y += 1;
            x = 0;

            if y >= rows {
                break;
            } else {
                str_vec.push(NEW_LINE_CRLF.to_string());
            }
        }

        Log::ep("cur.y", self.cur.y);
        Log::ep("y_offset", self.offset_y);
        Log::ep("cur.x", self.cur.x);
        Log::ep("cur.disp_x", self.cur.disp_x);
    }

    pub fn set_sel_del_d_range(&mut self) {
        let sel = self.sel.get_range();
        self.d_range = DRnage { sy: sel.sy, ey: sel.sy, d_type: DType::All };
    }
}
