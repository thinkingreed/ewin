use crate::{
    ewin_com::{colors::*, def::*, log::*, model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};

impl Editor {
    pub fn render(&mut self, str_vec: &mut Vec<String>, draw: &mut EditorDraw) {
        Log::info_key("Editor.draw");

        // let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);
        let d_range = self.draw_range;
        Log::debug("d_range", &d_range);

        match d_range {
            E_DrawRange::Not => {}
            E_DrawRange::MoveCur => {
                self.render_scrlbar_h(str_vec);
                self.render_scrlbar_v(str_vec);
                self.render_row_num(str_vec);
                return;
            }
            E_DrawRange::TargetRange(sy, ey) => {
                // for e_cmd::AllSelect
                let start_y = if sy >= self.offset_y { sy - self.offset_y } else { self.offset_y };
                let end_y = if ey <= self.offset_y + self.row_disp_len { ey - self.offset_y } else { self.offset_y + self.row_disp_len };

                for i in start_y..=end_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (start_y + self.row_posi) as u16)));
            }
            E_DrawRange::Targetpoint => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::Init | E_DrawRange::All => self.clear_all(str_vec, self.row_posi - 1),
            E_DrawRange::After(_) => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.row_disp_len - Editor::SCROLL_UP_DOWN_MARGIN - 1) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        Log::debug("draw.change_row_vec", &draw.change_row_vec);

        // Judg redraw row_num
        self.render_row_num(str_vec);

        for i in draw.change_row_vec.iter() {
            str_vec.push(format!("{}", MoveTo(0, (*i - self.offset_y + self.row_posi) as u16)));
            self.set_row_num(*i, str_vec);
            let row_cell = &draw.cells_to[i];

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
            }
            /*
            else {
                str_vec.push(NEW_LINE_CRLF.to_string());
            }
             */
        }
        draw.cells_from = std::mem::take(&mut draw.cells_to);
        //  std::mem::swap(&mut draw.cells_from, &mut draw.cells_to);
        // draw.cells_from = draw.cells_to.clone();

        str_vec.push(Colors::get_default_bg());
        self.render_scrlbar_v(str_vec);
        self.render_scrlbar_h(str_vec);
    }

    pub fn render_row_num(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("render_row_num");
        // If you need to edit the previous row_num
        if self.state.mouse_mode == MouseMode::Normal {
            // Correspondence of line number of previous cursor position
            if self.cur_org.y < self.buf.len_rows() - 1 && self.is_y_in_screen(self.cur_org.y) && self.cur.y != self.cur_org.y {
                self.move_render_row_num(str_vec, self.cur_org.y);
            }
            if self.is_y_in_screen(self.cur.y) {
                self.move_render_row_num(str_vec, self.cur.y);
            }
        }
    }

    fn move_render_row_num(&mut self, str_vec: &mut Vec<String>, y: usize) {
        Log::debug("yyyyyyyyyyyyyyyyyyyyyyyyyyyyy", &y);

        str_vec.push(format!("{}", MoveTo(0, (self.row_posi + y - self.offset_y) as u16)));
        self.set_row_num(y, str_vec);
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

    pub fn clear_all(&self, str_vec: &mut Vec<String>, sy: usize) {
        Log::debug_key("Editor.clear_all");
        for i in sy..=self.row_posi + self.row_disp_len {
            str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
        }
        str_vec.push(format!("{}", MoveTo(0, self.row_posi as u16)));
    }

    pub fn clear_all_diff(&self, str_vec: &mut Vec<String>, change_row_vec: &[usize]) {
        Log::debug_key("Editor.clear_all_diff");

        for i in change_row_vec {
            str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_rows() && self.row_len_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.row_len_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
            }
        }
    }
}
