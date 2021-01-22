use crate::{def::*, global, global::*, model::*};
use permissions::*;
use std::cmp::min;
use std::env;
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::path;
use std::path::Path;
use syntect::parsing::SyntaxSet;
use termion::{clear, cursor, scroll};
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
        self.extension = Path::new(&self.path_str).extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string();
        let tmp = self.syntax.syntax_set.find_syntax_by_extension(&self.extension);
        if let Some(sr) = tmp {
            self.syntax.syntax = sr.clone();
        } else {
            self.syntax.syntax = self.syntax.syntax_set.find_syntax_by_extension("txt").unwrap().clone();
        }
        self.syntax.theme = self.syntax.theme_set.themes[global::CFG.lock().unwrap()["THEME"]].clone();
        self.set_cur_default();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("draw");

        let (mut y, mut x) = (0, 0);

        let d_range = self.d_range.get_range();
        Log::ep("d_range", d_range);

        match d_range.d_type {
            DrawType::Not => {}
            DrawType::None | DrawType::All => {
                Colors::set_textarea_color(str_vec);
                str_vec.push(format!("{}{}", clear::All.to_string(), cursor::Goto(1, 1).to_string()));
            }
            DrawType::Target => {
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                }
                str_vec.push(format!("{}", cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16)));
            }
            DrawType::After => str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16), clear::AfterCursor)),
            DrawType::ScrollDown => str_vec.push(format!("{}{}{}", scroll::Up(1), clear::CurrentLine, cursor::Goto(1, self.disp_row_num as u16))),
            DrawType::ScrollUp => str_vec.push(format!("{}{}", scroll::Down(1), cursor::Goto(1, 1))),
        }

        for i in self.draw.sy..=self.draw.ey {
            Log::ep("iii", i);

            self.set_row_num(i, str_vec);

            let row_region = self.draw.regions[i].clone();
            let x_s = if i == self.cur.y { self.offset_x } else { 0 };
            let x_e = min(x_s + self.disp_col_num, self.buf.len_line_chars(i));

            for (x_idx, j) in (0_usize..).zip(x_s..x_e) {
                let region = &row_region[j];
                region.draw_style(str_vec, x_idx == 0 && self.offset_x > 0);
                let c = region.c;
                let mut width = c.width().unwrap_or(0);
                if c == NEW_LINE {
                    width = 1;
                }
                let x_w_l = x + width + self.rnw;
                if x_w_l > self.disp_col_num {
                    break;
                }
                x += width;

                match c {
                    EOF_MARK => self.set_eof(str_vec),
                    NEW_LINE => str_vec.push(NEW_LINE_MARK.to_string()),
                    NEW_LINE_CR => {}
                    _ => str_vec.push(c.to_string()),
                }
            }
            y += 1;
            x = 0;

            if y >= self.disp_row_num {
                break;
            } else {
                str_vec.push(NEW_LINE_CRLF.to_string());
            }
        }

        self.d_range.clear();
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
