use crate::{colors::*, def::*, global::*, log::*, model::*};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;
use unicode_width::UnicodeWidthChar;

impl Editor {
    pub fn draw<T: Write>(&mut self, out: &mut T, term_mode: MouseMode) {
        Log::info_key("Editor.draw");
        Log::debug("editor.d_range", &self.d_range);

        let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);

        let d_range = self.d_range.clone();

        match d_range.draw_type {
            DrawType::Not | DrawType::MoveCur => {}
            DrawType::None => {
                let cfg = CFG.get().unwrap().try_lock().unwrap();
                if let Some(color) = cfg.syntax.theme.settings.background {
                    str_vec.push(if self.is_enable_syntax_highlight && cfg.colors.theme.theme_bg_enable { Colors::bg(Color::from(color)) } else { Colors::bg(cfg.colors.editor.bg) });
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
            DrawType::ScrollDown => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.disp_row_num - Editor::UP_DOWN_EXTRA - 1) as u16), Clear(ClearType::FromCursorDown))),
            DrawType::ScrollUp => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.disp_row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.width;

        for i in self.draw.sy..=self.draw.ey {
            self.set_row_num(i, &mut str_vec, term_mode);
            let row_cell = &self.draw.cells[i];

            let mut c_org = ' ';
            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(&mut str_vec, x_idx == 0 && self.offset_x > 0);
                let c = cell.c;

                let tab_width = if c == TAB_CHAR { cfg_tab_width - ((x_width + self.offset_disp_x) % cfg_tab_width) } else { 0 };

                let width = match c {
                    TAB_CHAR => tab_width,
                    NEW_LINE_LF => {
                        // NEW_LINE_LF_MARK width
                        if cfg!(target_os = "linux") {
                            1
                        } else {
                            2
                        }
                    }
                    _ => c.width().unwrap_or(0),
                };

                if x_width + width > self.disp_col_num {
                    break;
                }
                x_width += width;

                if term_mode == MouseMode::Normal {
                    match c {
                        EOF_MARK => {
                            // EOF_STR.len() - 1 is rest 2 char
                            let disp_len = if EOF_STR.len() - 1 + x_width > self.disp_col_num { EOF_STR.len() - (EOF_STR.len() - 1 + x_width - self.disp_col_num) } else { EOF_STR.len() };
                            Colors::set_eof(&mut str_vec, disp_len)
                        }
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", TAB_MARK, " ".repeat(tab_width - 1))),
                        _ => str_vec.push(c.to_string()),
                    }
                } else {
                    match c {
                        EOF_MARK | NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(" ".repeat(tab_width)),
                        _ => str_vec.push(c.to_string()),
                    }
                }
                c_org = c;
            }
            y += 1;
            x_width = 0;

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

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>, term_mode: MouseMode) {
        if term_mode == MouseMode::Normal {
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
}
