use crate::{
    ewin_com::{colors::*, def::*, log::*, model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use ewin_com::_cfg::cfg::{Cfg, CfgSyntax};
use std::io::Write;

impl Editor {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw: &EditorDraw) {
        Log::info_key("Editor.draw");

        // let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);
        let d_range = self.draw_range;
        Log::debug("d_range", &d_range);

        match d_range {
            E_DrawRange::Not => {}
            E_DrawRange::MoveCur => {}
            E_DrawRange::None => {
                self.set_bg_color(str_vec);
                str_vec.push(format!("{}{}", Clear(ClearType::All), MoveTo(0, self.row_posi as u16)));
            }
            E_DrawRange::Target(sy, ey) => {
                for i in sy - self.offset_y..=ey - self.offset_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (sy - self.offset_y + self.row_posi) as u16)));
            }
            E_DrawRange::All => self.clear_all(str_vec, self.row_posi - 1),
            E_DrawRange::After(sy) => str_vec.push(format!("{}{}", MoveTo(0, (sy - self.offset_y + self.row_posi) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.row_disp_len - Editor::SCROLL_UP_DOWN_MARGIN - 1) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        // If you need to edit the previous row_num
        if self.offset_y == self.offset_y_org && !(draw.sy <= self.cur_y_org && self.cur_y_org <= draw.ey) && self.buf.len_rows() - 1 > self.cur_y_org && self.is_move_cur_posi_scrolling_enable() {
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
                if x_width + width > self.col_len {
                    break;
                }
                x_width += width;
                if self.state.mouse_mode == MouseMode::Normal {
                    match c {
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", TAB_MARK, " ".repeat(width - 1))),
                        _ => str_vec.push(c.to_string()),
                    }
                    // self.state.mouse_mode == MouseMode::Mouse
                } else {
                    match c {
                        NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(" ".repeat(width)),
                        _ => str_vec.push(c.to_string()),
                    }
                }
                c_org = c;
            }

            y += 1;
            x_width = 0;

            if y >= self.row_disp_len {
                break;
            } else {
                str_vec.push(NEW_LINE_CRLF.to_string());
            }
        }
        str_vec.push(Colors::get_default_bg());
        self.draw_scrlbar_v(str_vec);
        self.draw_scrlbar_h(str_vec);

        self.draw_range = E_DrawRange::Not;
    }

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>) {
        if self.state.mouse_mode == MouseMode::Normal {
            if i == self.cur.y {
                Colors::set_rownum_curt_color(str_vec);
            } else {
                Colors::set_rownum_color(str_vec);
            }

            if self.get_rnw() > 0 {
                str_vec.push(" ".repeat(self.get_rnw() - (i + 1).to_string().len()));
            }
            str_vec.push((i + 1).to_string());

            #[allow(clippy::repeat_once)]
            str_vec.push(" ".to_string().repeat(Editor::RNW_MARGIN));
            Colors::set_text_color(str_vec);
        }
    }

    pub fn set_bg_color(&mut self, str_vec: &mut Vec<String>) {
        if let Some(color) = CfgSyntax::get().syntax.theme.settings.background {
            str_vec.push(if self.is_enable_syntax_highlight && Cfg::get().colors.theme.theme_bg_enable { Colors::bg(Color::from(color)) } else { Colors::bg(Cfg::get().colors.editor.bg) });
        } else {
            str_vec.push(Colors::bg(Cfg::get().colors.editor.bg));
        }
    }
    pub fn clear_draw<T: Write>(&self, out: &mut T, sy: usize) {
        let mut str_vec: Vec<String> = vec![];
        self.clear_all(&mut str_vec, sy);
        let _ = out.write(str_vec.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn clear_all(&self, str_vec: &mut Vec<String>, sy: usize) {
        Log::debug_key("clear_render_vec");
        if self.row_disp_len > 0 {
            for i in sy..=self.row_posi + self.row_disp_len - 2 {
                str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
            }
            str_vec.push(format!("{}", MoveTo(0, self.row_posi as u16)));
        }
    }
}
