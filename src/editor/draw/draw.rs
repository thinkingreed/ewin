use crate::{def::*, global::*, model::*};
use permissions::*;
use std::cmp::min;
use std::env;
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::path;
use std::path::Path;
use syntect::parsing::SyntaxSet;
use termion::{clear, cursor};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn open(&mut self, path: &path::Path, mbar: &mut MsgBar) {
        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                if !is_writable(path).unwrap() {
                    let msg_1 = &LANG.unable_to_edit.clone();
                    let msg_2 = &LANG.no_write_permission.clone();
                    mbar.set_readonly(&format!("{}({})", msg_1, msg_2));
                }
            } else {
                println!("{}", LANG.file_not_found.clone());
                std::process::exit(1);
            }
        } else {
            let current = env::current_dir().unwrap();
            if !is_writable(current).unwrap() {
                println!("{}", LANG.no_write_permission.clone());
                std::process::exit(1);
            }
        }
        // read
        let result = TextBuffer::from_path(&path.to_string_lossy().to_string());
        match result {
            Ok(t_buf) => {
                self.buf = t_buf;
                self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK);
            }
            Err(err) => match err.kind() {
                ErrorKind::PermissionDenied => {
                    println!("{}", LANG.no_read_permission.clone());
                    std::process::exit(1);
                }
                ErrorKind::NotFound => self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK),
                _ => {
                    println!("{} {:?}", LANG.file_opening_problem, err);
                    std::process::exit(1);
                }
            },
        }
        self.path_str = path.to_string_lossy().to_string();
        self.path = Some(path.into());

        self.syntax.syntax_set = SyntaxSet::load_defaults_newlines();
        let extension = &Path::new(&self.path_str).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
        let tmp = self.syntax.syntax_set.find_syntax_by_extension(extension);
        if let Some(sr) = tmp {
            self.syntax.syntax = sr.clone();
        } else {
            self.syntax.syntax = self.syntax.syntax_set.find_syntax_by_extension("txt").unwrap().clone();
        }
        self.set_cur_default();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("draw");

        let (rows, cols) = (self.disp_row_num, self.disp_col_num);
        // Rows and columns on the terminal
        let (mut y, mut x) = (0, 0);

        let d_range = self.d_range.get_range();
        Log::ep("d_range", d_range);

        if d_range.d_type == DrawType::Not {
            return;
        } else if d_range.d_type == DrawType::None || d_range.d_type == DrawType::All {
            str_vec.push(clear::All.to_string());
            str_vec.push(cursor::Goto(1, 1).to_string());
        } else {
            if d_range.d_type == DrawType::Target {
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                }
                str_vec.push(cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16).to_string());
            } else if d_range.d_type == DrawType::After {
                Log::ep_s(" DrawType::After DrawType::After DrawType::After");
                Log::ep("d_range.sy", d_range.sy);
                Log::ep("self.offset_y", self.offset_y);
                Log::ep("d_range.sy + 1 - self.offset_y", d_range.sy + 1 - self.offset_y);

                str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16), clear::AfterCursor));

                /*
                    for i in d_range.sy - self.offset_y..=rows {
                        str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                    }
                    str_vec.push(cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16).to_string());
                */
            }
        }

        for i in self.draw.sy..=self.draw.ey {
            Log::ep("draw idx", i);

            self.set_row_num(i, str_vec);

            let row_region = self.draw.regions[i].clone();

            let x_s = if i == self.cur.y { self.offset_x } else { 0 };
            let x_e = min(x_s + cols, self.buf.len_line_chars(i));

            for j in x_s..x_e {
                let region = &row_region[j];

                region.draw(str_vec);
                let c = region.c;
                // Log::ep("ccc", c);

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
        Log::ep("self.sel", self.sel);
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
