use crate::{
    ewin_com::{colors::*, def::*, global::*, log::*, model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use std::io::Write;

impl Editor {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw: &EditorDraw) {
        Log::info_key("Editor.draw");

        // let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);
        let d_range = self.draw_range;
        Log::debug("d_range", &d_range);

        match d_range {
            E_DrawRange::Not | E_DrawRange::MoveCur => {}
            E_DrawRange::None => {
                self.set_bg_color(str_vec);
                str_vec.push(format!("{}{}", Clear(ClearType::All), MoveTo(0, self.row_posi as u16).to_string()));
            }
            E_DrawRange::Target(sy, ey) => {
                for i in sy - self.offset_y..=ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (sy - self.offset_y + self.row_posi) as u16)));
            }
            E_DrawRange::All => self.clear_draw_vec(str_vec, self.row_posi - 1),
            E_DrawRange::After(sy) => str_vec.push(format!("{}{}", MoveTo(0, (sy - self.offset_y + self.row_posi) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.row_len - Editor::SCROLL_UP_DOWN_EXTRA - 1) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        // If you need to edit the previous row_num
        if self.offset_y <= self.cur_y_org && self.cur_y_org <= self.offset_y + self.row_len && !(draw.sy <= self.cur_y_org && self.cur_y_org <= draw.ey) {
            str_vec.push(format!("{}", MoveTo(0, (self.cur_y_org - self.offset_y + self.row_posi) as u16)));
            self.set_row_num(self.cur_y_org, str_vec);
            str_vec.push(format!("{}", MoveTo(0, (draw.sy - self.offset_y + self.row_posi) as u16)));
        }

        for i in draw.sy..=draw.ey {
            self.set_row_num(i, str_vec);
            let row_cell = &draw.cells[i];

            let mut c_org = ' ';
            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(str_vec, x_idx == 0 && self.offset_x > 0);
                let c = cell.c;
                let width = get_char_width(&c, x_width);
                if x_width + width > self.col_num {
                    break;
                }
                x_width += width;

                if self.state.mouse_mode == MouseMode::Normal {
                    match c {
                        EOF_MARK => {
                            // EOF_STR.len() - 1 is rest 2 char
                            let disp_len = if EOF_STR.len() - 1 + x_width > self.col_num { EOF_STR.len() - (EOF_STR.len() - 1 + x_width - self.col_num) } else { EOF_STR.len() };
                            Colors::set_eof(str_vec, disp_len)
                        }
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", TAB_MARK, " ".repeat(width - 1))),
                        _ => str_vec.push(c.to_string()),
                    }
                } else {
                    match c {
                        EOF_MARK | NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(" ".repeat(width)),
                        _ => str_vec.push(c.to_string()),
                    }
                }
                c_org = c;
            }
            y += 1;
            x_width = 0;

            if y >= self.row_len {
                break;
            } else {
                str_vec.push(NEW_LINE_CRLF.to_string());
            }
        }
        self.draw_scrlbar_v(str_vec);

        self.draw_range = E_DrawRange::Not;
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
                    str_vec.push(" ".repeat(self.get_rnw() - (i + 1).to_string().len()));
                }
                str_vec.push((i + 1).to_string());
            }
            #[allow(clippy::repeat_once)]
            str_vec.push(" ".to_string().repeat(Editor::RNW_MARGIN));
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
    pub fn clear_draw<T: Write>(&self, out: &mut T, sy: usize) {
        let mut str_vec: Vec<String> = vec![];
        self.clear_draw_vec(&mut str_vec, sy);
        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn clear_draw_vec(&self, str_vec: &mut Vec<String>, sy: usize) {
        if self.row_len > 0 {
            for i in sy..=self.row_posi + self.row_len - 2 {
                str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
            }
            str_vec.push(format!("{}", MoveTo(0, self.row_posi as u16)));
        }
    }
}
