use crate::{colors::*, def::*, global::*, model::*, util::*};
use crossterm::style::{Color as CrosstermColor, SetBackgroundColor};
use crossterm::{cursor::*, terminal::*};
use std::{cmp::min, env, fs::metadata, io::ErrorKind, path};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn open(&mut self, path: &path::Path, mbar: &mut MsgBar) {
        Log::ep_s("           open");

        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                let file_meta = metadata(path).unwrap();
                if file_meta.permissions().readonly() {
                    let msg_1 = &LANG.unable_to_edit.clone();
                    let msg_2 = &LANG.no_write_permission.clone();
                    mbar.set_readonly(&format!("{}({})", msg_1, msg_2));
                }

                // Judge enable syntax_highlight
                if CFG.get().unwrap().syntax.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&self.file.ext) {
                    self.file.is_enable_syntax_highlight = true;
                }

                Log::ep("self.file.is_enable_syntax_highlight", &self.file.is_enable_syntax_highlight);

                self.file.path = Some(path.into());
            } else {
                println!("{}", LANG.file_not_found.clone());
                std::process::exit(1);
            }
        } else {
            let curt_dir = env::current_dir().unwrap();
            let curt_dir = metadata(curt_dir).unwrap();
            if curt_dir.permissions().readonly() {
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
                ErrorKind::NotFound => {
                    self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK);
                    self.file.path = None;
                }
                _ => {
                    println!("{} {:?}", LANG.file_opening_problem, err);
                    std::process::exit(1);
                }
            },
        }

        self.set_cur_default();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::ep_s("draw");

        let (mut y, mut x) = (0, 0);

        let d_range = self.d_range.get_range();
        Log::ep("d_range", &d_range);

        match d_range.draw_type {
            DrawType::Not => {}
            DrawType::None | DrawType::All => {
                if let Some(c) = CFG.get().unwrap().syntax.theme.settings.background {
                    if is_enable_syntax_highlight(&self.file.ext) && CFG.get().unwrap().colors.theme.theme_background_enable {
                        str_vec.push(SetBackgroundColor(CrosstermColor::from(Color::from(c))).to_string());
                    } else {
                        str_vec.push(SetBackgroundColor(CrosstermColor::from(CFG.get().unwrap().colors.editor.bg)).to_string());
                    }
                }
                str_vec.push(format!("{}{}", Clear(ClearType::All), MoveTo(0, 0).to_string()));
            }
            DrawType::Target => {
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, i as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (d_range.sy - self.offset_y) as u16)));
            }
            DrawType::After => str_vec.push(format!("{}{}", MoveTo(0, (d_range.sy - self.offset_y) as u16), Clear(ClearType::FromCursorDown))),
            DrawType::ScrollDown => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.disp_row_num - 1) as u16), Clear(ClearType::CurrentLine))),
            DrawType::ScrollUp => str_vec.push(format!("{}{}", ScrollDown(1), MoveTo(0, 0))),
        }

        for i in self.draw.sy..=self.draw.ey {
            // Log::ep("iii", &i);

            self.set_row_num(i, str_vec);
            let row_region = self.draw.regions[i].clone();
            let (mut sx, mut ex) = (0, row_region.len());

            if self.file.is_enable_syntax_highlight {
                sx = if i == self.cur.y { self.offset_x } else { 0 };
                ex = min(sx + self.disp_col_num, self.buf.len_line_chars(i));
            }

            for (x_idx, j) in (0_usize..).zip(sx..ex) {
                let region = &row_region[j];
                region.draw_style(str_vec, x_idx == 0 && self.offset_x > 0);
                let c = region.c;
                // Log::ep("ccccc", &c);

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
                    EOF_MARK => Colors::set_eof(str_vec),
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
        if self.cur.y == i && self.offset_disp_x > 0 {
            str_vec.push(">".repeat(self.rnw).to_string());
        } else {
            str_vec.push(" ".repeat(self.rnw - (i + 1).to_string().len()).to_string());
            str_vec.push((i + 1).to_string());
        }
        Colors::set_text_color(str_vec);
    }
}
