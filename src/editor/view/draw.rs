use crate::{colors::*, def::*, global::*, log::*, model::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn draw<T: Write>(&mut self, out: &mut T) {
        Log::ep_s("draw");

        let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x) = (0, 0);

        let d_range = self.d_range.get_range();
        Log::ep("d_range", &d_range);

        match d_range.draw_type {
            DrawType::Not | DrawType::MoveCur => {}
            DrawType::None => {
                let cfg = CFG.get().unwrap().try_lock().unwrap();
                if let Some(c) = cfg.syntax.theme.settings.background {
                    //  if is_enable_syntax_highlight(&self.file.ext) && cfg.colors.theme.theme_bg_enable {
                    if self.is_enable_syntax_highlight && cfg.colors.theme.theme_bg_enable {
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
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.disp_row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (d_range.sy - self.offset_y + self.disp_row_posi) as u16)));
            }
            DrawType::All => str_vec.push(format!("{}{}", MoveTo(0, self.disp_row_posi as u16), Clear(ClearType::FromCursorDown))),
            DrawType::After => str_vec.push(format!("{}{}", MoveTo(0, (d_range.sy - self.offset_y + self.disp_row_posi) as u16), Clear(ClearType::FromCursorDown))),
            DrawType::ScrollDown => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.disp_row_num - Editor::SCROLL_DOWN_EXTRA_NUM - 1) as u16), Clear(ClearType::FromCursorDown))),
            DrawType::ScrollUp => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.disp_row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        for i in self.draw.sy..=self.draw.ey {
            // Log::ep("iii", &i);

            self.set_row_num(i, &mut str_vec);
            let row_cell = &self.draw.cells[i];

            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(&mut str_vec, x_idx == 0 && self.offset_x > 0);
                let c = cell.c;

                let mut width = c.width().unwrap_or(0);
                if c == NEW_LINE {
                    width = 1;
                }
                let x_w_l = x + width + self.get_rnw() + Editor::RNW_MARGIN;
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
        // if i == self.cur.y - self.offset_y {
        if i == self.cur.y {
            Colors::set_rownum_curt_color(str_vec);
        } else {
            Colors::set_rownum_color(str_vec);
        }
        if self.cur.y == i && self.offset_disp_x > 0 {
            str_vec.push(">".repeat(self.get_rnw()));
        } else {
            if self.get_rnw() > 0 {
                str_vec.push(" ".repeat(self.get_rnw() - (i + 1).to_string().len()).to_string());
            }
            str_vec.push((i + 1).to_string());
        }
        str_vec.push(" ".repeat(Editor::RNW_MARGIN).to_string());
        Colors::set_text_color(str_vec);
    }
}
