use crate::{bar::msgbar::*, colors::*, def::*, global::*, log::*, model::*, terminal::*, util::*};
use crossterm::{cursor::*, terminal::*};
use std::{
    cmp::min,
    env,
    fs::metadata,
    io::{ErrorKind, Write},
    path,
};
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn open(&mut self, path: &path::Path, mbar: &mut MsgBar) {
        Log::ep_s("           open");

        if path.to_string_lossy().to_string().len() > 0 {
            if path.exists() {
                let file_meta = metadata(path).unwrap();
                if file_meta.permissions().readonly() {
                    mbar.set_readonly(&format!("{}({})", &LANG.unable_to_edit, &LANG.no_write_permission));
                }
                if CFG.get().unwrap().try_lock().unwrap().syntax.syntax_reference.is_some() && file_meta.len() < ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE && is_enable_syntax_highlight(&FILE.get().unwrap().try_lock().unwrap().ext) {
                    FILE.get().unwrap().try_lock().map(|mut file| file.is_enable_syntax_highlight = true).unwrap();
                }
                FILE.get().unwrap().try_lock().map(|mut file| file.path = Some(path.into())).unwrap();
            } else {
                Terminal::exit();
                println!("{}", LANG.file_not_found.clone());
            }
        } else {
            let curt_dir = env::current_dir().unwrap();
            let curt_dir = metadata(curt_dir).unwrap();
            if curt_dir.permissions().readonly() {
                Terminal::exit();
                println!("{}", LANG.no_write_permission.clone());
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
                    Terminal::exit();
                    println!("{}", LANG.no_read_permission.clone());
                }
                ErrorKind::NotFound => {
                    self.buf.text.insert_char(self.buf.text.len_chars(), EOF_MARK);
                    FILE.get().unwrap().try_lock().map(|mut file| file.path = None).unwrap();
                }
                _ => {
                    Terminal::exit();
                    println!("{} {:?}", LANG.file_opening_problem, err);
                }
            },
        }
        self.set_cur_default();
    }

    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("draw");

        let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x) = (0, 0);

        let d_range = self.d_range.get_range();
        Log::ep("d_range", &d_range);

        let is_syntax_highlight = FILE.get().unwrap().try_lock().unwrap().is_enable_syntax_highlight;
        match d_range.draw_type {
            DrawType::Not | DrawType::MoveCur => {}
            DrawType::None => {
                let cfg = CFG.get().unwrap().try_lock().unwrap();
                if let Some(c) = cfg.syntax.theme.settings.background {
                    //  if is_enable_syntax_highlight(&self.file.ext) && cfg.colors.theme.theme_bg_enable {
                    if is_syntax_highlight && cfg.colors.theme.theme_bg_enable {
                        str_vec.push(Colors::bg(Color::from(c)));
                    } else {
                        str_vec.push(Colors::bg(cfg.colors.editor.bg));
                    }
                } else {
                    str_vec.push(Colors::bg(cfg.colors.editor.bg));
                }
                str_vec.push(format!("{}{}", Clear(ClearType::All), MoveTo(0, self.disp_row_posi as u16).to_string()));
            }
            DrawType::Target => {
                Log::ep("self.offset_y", &self.offset_y);
                Log::ep("d_range.sy", &d_range.sy);
                Log::ep("d_range.ey", &d_range.ey);

                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.disp_row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (d_range.sy - self.offset_y + self.disp_row_posi) as u16)));
            }
            DrawType::All => str_vec.push(format!("{}{}", MoveTo(0, self.disp_row_posi as u16), Clear(ClearType::FromCursorDown))),
            DrawType::After => str_vec.push(format!("{}{}", MoveTo(0, (d_range.sy - self.offset_y + self.disp_row_posi) as u16), Clear(ClearType::FromCursorDown))),
            DrawType::ScrollDown => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, self.disp_row_num as u16), Clear(ClearType::CurrentLine))),
            DrawType::ScrollUp => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, self.disp_row_posi as u16), Clear(ClearType::CurrentLine))),
        }

        for i in self.draw.sy..=self.draw.ey {
            // Log::ep("iii", &i);

            self.set_row_num(i, &mut str_vec);
            let row_cell = &self.draw.cells[i];
            let (mut sx, mut ex) = (0, row_cell.len());

            if is_syntax_highlight {
                sx = if i == self.cur.y { self.offset_x } else { 0 };
                ex = min(sx + self.disp_col_num, self.buf.len_line_chars(i));
            }

            for (x_idx, j) in (0_usize..).zip(sx..ex) {
                let cell = &row_cell[j];
                cell.draw_style(&mut str_vec, x_idx == 0 && self.offset_x > 0);
                let c = cell.c;
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
                    EOF_MARK => Colors::set_eof(&mut str_vec),
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

        let _ = out.write(&str_vec.concat().as_bytes());
        out.flush().unwrap();

        self.d_range.clear();
        self.sel_org.clear();
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
