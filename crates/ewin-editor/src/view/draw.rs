use crate::{
    ewin_com::{model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*, model::default::*};
use ewin_const::def::*;

impl Editor {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw: &mut EditorDraw) {
        Log::info_key("Editor.draw");

        // let mut str_vec: Vec<String> = vec![];
        let (mut y, mut x_width) = (0, 0);
        let d_range = self.draw_range;
        Log::debug("d_range", &d_range);

        match d_range {
            E_DrawRange::Not => {}
            E_DrawRange::MoveCur => {
                self.draw_scrlbar_h(str_vec);
                self.draw_scrlbar_v(str_vec);
                self.draw_row_num(str_vec);
                return;
            }
            E_DrawRange::TargetRange(sy, ey) => {
                // for e_cmd::AllSelect
                let start_y = if sy >= self.offset_y { sy - self.offset_y } else { self.offset_y };
                let end_y = if ey <= self.offset_y + self.row_len { ey - self.offset_y } else { self.offset_y + self.row_len };

                for i in start_y..=end_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (start_y + self.row_posi) as u16)));
            }
            E_DrawRange::Targetpoint => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::Init | E_DrawRange::All => self.clear_all(str_vec),
            E_DrawRange::After(_) => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (self.row_posi + self.row_len - Editor::SCROLL_UP_DOWN_MARGIN) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, (self.row_posi) as u16), Clear(ClearType::CurrentLine))),
        }

        Log::debug("draw.change_row_vec", &draw.change_row_vec);

        // Judg redraw row_num
        self.draw_row_num(str_vec);

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
                if self.state.mouse == Mouse::Enable {
                    match c {
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", Cfg::get().general.view.tab_characters_as_symbols, " ".repeat(width - 1))),
                        FULL_SPACE => str_vec.push(Cfg::get().general.view.full_width_space_characters_as_symbols.to_string()),

                        _ => str_vec.push(c.to_string()),
                    }
                    // self.state.mouse_mode == MouseMode::Mouse
                } else {
                    match c {
                        NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(" ".repeat(width)),
                        FULL_SPACE => str_vec.push(Cfg::get().general.view.full_width_space_characters_as_symbols.to_string()),
                        _ => str_vec.push(c.to_string()),
                    }
                }
                c_org = c;
            }

            y += 1;
            x_width = 0;

            if y >= self.row_len {
                break;
            }
        }

        if CfgEdit::get().general.editor.scale.is_enable {
            self.draw_scale(str_vec);
        }

        draw.cells_from = std::mem::take(&mut draw.cells_to);
        //  std::mem::swap(&mut draw.cells_from, &mut draw.cells_to);
        // draw.cells_from = draw.cells_to.clone();

        str_vec.push(Colors::get_default_bg());
        self.draw_scrlbar_v(str_vec);
        self.draw_scrlbar_h(str_vec);
    }

    pub fn draw_row_num(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_row_num");
        // If you need to edit the previous row_num
        if CfgEdit::get().general.editor.row_no.is_enable {
            // Correspondence of line number of previous cursor position
            if self.cur_org.y < self.buf.len_rows() && self.is_y_in_screen(self.cur_org.y) && self.cur.y != self.cur_org.y {
                self.move_render_row_num(str_vec, self.cur_org.y);
            }
            if self.is_y_in_screen(self.cur.y) {
                self.move_render_row_num(str_vec, self.cur.y);
            }
        }
    }

    fn move_render_row_num(&mut self, str_vec: &mut Vec<String>, y: usize) {
        str_vec.push(format!("{}", MoveTo(0, (self.row_posi + y - self.offset_y) as u16)));
        self.set_row_num(y, str_vec);
    }

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>) {
        if CfgEdit::get().general.editor.row_no.is_enable {
            if i == self.cur.y {
                str_vec.push(Colors::get_rownum_curt_fg_bg());
            } else {
                // Colors::set_rownum_not_curt_color(str_vec);
                str_vec.push(Colors::get_rownum_not_curt_fg_bg());
            }
            if self.get_rnw() > 0 {
                str_vec.push(" ".repeat(self.get_rnw() - (i + 1).to_string().len()));
            }
            str_vec.push((i + 1).to_string());

            #[allow(clippy::repeat_once)]
            str_vec.push(" ".to_string().repeat(Editor::RNW_MARGIN));
            str_vec.push(Colors::get_default_fg_bg())
        }
    }

    pub fn clear_all(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("Editor.clear_all");

        for i in self.row_posi..=self.row_posi + self.row_len {
            str_vec.push(format!("{}{}", MoveTo(0, i as u16), Clear(ClearType::CurrentLine)));
        }

        str_vec.push(format!("{}", MoveTo(0, self.row_posi as u16)));
    }

    pub fn clear_all_diff(&self, str_vec: &mut Vec<String>, change_row_vec: &[usize]) {
        Log::debug_key("Editor.clear_all_diff");

        for i in change_row_vec {
            str_vec.push(format!("{}{}", MoveTo(0, (*i - self.offset_y + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_row_including_extra() && self.buf_rows_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.buf_rows_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(0, (i + self.row_posi) as u16), Clear(ClearType::CurrentLine)));
            }
        }
    }
}
