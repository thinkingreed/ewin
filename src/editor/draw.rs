use crate::{def::*, global::*, model::*};
use permissions::*;
use std::cmp::min;
use std::env;
use std::io::ErrorKind;
use std::path;
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

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        let (rows, cols) = (self.disp_row_num, self.disp_col_num);

        // Rows and columns on the terminal
        let (mut y, mut x) = (0, 0);

        self.rnw = self.buf.len_lines().to_string().len();
        let sel_range = self.sel.get_range();
        let search_ranges = self.search.search_ranges.clone();

        for i in self.draw.y_s..self.draw.y_e {
            self.set_row_num(i, str_vec);

            let row_vec = self.draw.char_vec[i].clone();
            let x_s = if i == self.cur.y { self.offset_x } else { 0 };
            let x_e = min(x_s + cols, row_vec.len().clone());

            for j in x_s..x_e {
                let c = row_vec[j];

                // highlight
                self.ctl_color(str_vec, &row_vec, sel_range, &search_ranges, i, j);

                let mut width = c.width().unwrap_or(0);
                if c == NEW_LINE {
                    width = 1;
                }

                let x_w_l = x + width + self.rnw;
                if x_w_l > cols {
                    break;
                }
                x += width;

                if c == EOF_MARK {
                    self.set_eof(str_vec);
                } else if c == NEW_LINE_CR {
                } else if c == NEW_LINE {
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

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>) {
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
    }
}
