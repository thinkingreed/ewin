use crate::{
    ewin_core::{colors::*, def::*, global::*, log::*, model::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use unicode_width::*;

impl Editor {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw: &EditorDraw) {
        Log::info_key("Editor.draw");

        // let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);
        let d_range = self.draw_range.clone();

        match d_range {
            EditorDrawRange::Not | EditorDrawRange::MoveCur => {}
            EditorDrawRange::None => {
                self.set_bg_color(str_vec);
                str_vec.push(format!("{}{}", Clear(ClearType::All), MoveTo(0, self.disp_row_posi as u16).to_string()));
            }
            EditorDrawRange::Target(sy, ey) => {
                for i in sy - self.offset_y..=ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.disp_row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (sy - self.offset_y + self.disp_row_posi) as u16)));
            }
            EditorDrawRange::All => {
                if self.disp_row_num > 0 {
                    for i in self.disp_row_posi - 1..=self.disp_row_posi + self.disp_row_num - 2 {
                        str_vec.push(format!("{}{}", MoveTo(0, (i + self.disp_row_posi) as u16), Clear(ClearType::CurrentLine)));
                    }
                    str_vec.push(format!("{}", MoveTo(0, self.disp_row_posi as u16)));
                }
            }
            EditorDrawRange::After(sy) => str_vec.push(format!("{}{}", MoveTo(0, (sy - self.offset_y + self.disp_row_posi) as u16), Clear(ClearType::FromCursorDown))),
            EditorDrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.disp_row_num - Editor::UP_DOWN_EXTRA - 1) as u16), Clear(ClearType::FromCursorDown))),
            EditorDrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.disp_row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        let cfg_tab_width = CFG.get().unwrap().try_lock().unwrap().general.editor.tab.size;

        for i in draw.sy..=draw.ey {
            self.set_row_num(i, str_vec);
            let row_cell = &draw.cells[i];

            let mut c_org = ' ';
            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(str_vec, x_idx == 0 && self.offset_x > 0);
                let c = cell.c;

                let tab_width = if c == TAB_CHAR { cfg_tab_width - ((x_width + if y == self.cur.y { self.offset_disp_x } else { 0 }) % cfg_tab_width) } else { 0 };

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

                if self.state.mouse_mode == MouseMode::Normal {
                    match c {
                        EOF_MARK => {
                            // EOF_STR.len() - 1 is rest 2 char
                            let disp_len = if EOF_STR.len() - 1 + x_width > self.disp_col_num { EOF_STR.len() - (EOF_STR.len() - 1 + x_width - self.disp_col_num) } else { EOF_STR.len() };
                            Colors::set_eof(str_vec, disp_len)
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

        self.draw_range = EditorDrawRange::Not;
        self.sel_org.clear();
    }

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>) {
        if self.state.mouse_mode == MouseMode::Normal {
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
    pub fn set_bg_color(&mut self, str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        if let Some(color) = cfg.syntax.theme.settings.background {
            str_vec.push(if self.is_enable_syntax_highlight && cfg.colors.theme.theme_bg_enable { Colors::bg(Color::from(color)) } else { Colors::bg(cfg.colors.editor.bg) });
        } else {
            str_vec.push(Colors::bg(cfg.colors.editor.bg));
        }
    }
}
