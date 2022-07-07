use crate::{
    ewin_com::{model::*, util::*},
    model::*,
};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*, model::default::*};
use ewin_const::def::*;

impl Editor {
    pub fn draw_move_cur(&mut self, str_vec: &mut Vec<String>, win: &Window) {
        self.draw_scrlbar_h(str_vec, win);

        self.draw_scrlbar_v(str_vec, win);
        let curt_win = self.get_curt_ref_win().clone();
        self.draw_pre_row_num(str_vec, &curt_win);
        self.draw_scale(str_vec, win);
    }

    pub fn clear_init(&mut self, str_vec: &mut Vec<String>, draw: &mut EditorDraw, win: &Window) -> ActType {
        Log::debug_key("clear_init");
        let curt_win = self.get_curt_ref_win();

        Log::debug("win", &win);
        Log::debug("curt_win", &curt_win);

        match curt_win.draw_range {
            E_DrawRange::Not | E_DrawRange::MoveCur => return ActType::Cancel,
            E_DrawRange::TargetRange(sy, ey) => {
                // for e_cmd::AllSelect
                let start_y = if sy >= win.offset.y { sy - win.offset.y } else { win.offset.y };

                Log::debug("ey", &ey);

                let end_y = if ey <= win.offset.y + win.area_v.1 { ey - win.offset.y } else { win.offset.y + win.area_v.1 };
                for i in start_y..=end_y {
                    str_vec.push(format!("{}{}", MoveTo(0, (i + win.area_v.0) as u16), Clear(ClearType::CurrentLine)));
                }
                str_vec.push(format!("{}", MoveTo(0, (start_y + win.area_v.0) as u16)));
            }
            E_DrawRange::Targetpoint => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::After(_) => self.clear_all_diff(str_vec, &draw.change_row_vec),
            E_DrawRange::Init | E_DrawRange::All => self.clear_all(str_vec),
            E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) if self.win_mgr.split_type != WindowSplitType::None => self.clear_all(str_vec),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (win.area_v.1 - Editor::SCROLL_UP_DOWN_MARGIN) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, win.area_v.0 as u16), Clear(ClearType::CurrentLine))),
        }

        return ActType::Next;
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>, draw: &mut EditorDraw, win: &Window) {
        Log::info_key("Editor.draw");

        let (mut y, mut x_width) = (0, 0);

        // Run only for curt window
        if self.win_mgr.curt_ref() == win {
            self.draw_pre_row_num(str_vec, win);
        }

        for i in draw.change_row_vec.iter() {
            str_vec.push(format!("{}", MoveTo((win.area_h.0 - self.get_rnw_and_margin()) as u16, (*i - win.offset.y + win.area_v.0) as u16)));
            self.set_row_num(*i, str_vec, win);
            let row_cell = &draw.cells_to[i];

            let mut c_org = ' ';
            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(str_vec, x_idx == 0 && win.offset.x > 0);
                let c = cell.c;
                let width = get_char_width(&c, x_width);
                if x_width + width > self.get_curt_col_len() {
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

            if y >= self.get_curt_row_len() {
                break;
            }
        }
        draw.cells_from = std::mem::take(&mut draw.cells_to);
    }

    pub fn draw_pre_row_num(&mut self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("draw_row_num");
        // If you need to edit the previous row_num
        if CfgEdit::get().general.editor.row_no.is_enable {
            // Correspondence of line number of previous cursor position
            if win.cur_org.y < self.buf.len_rows() && self.is_y_in_screen(win.cur_org.y) && win.cur.y != win.cur_org.y {
                self.mod_pre_row_num(str_vec, win.cur_org.y, win);
            }
            if self.is_y_in_screen(win.cur.y) {
                self.mod_pre_row_num(str_vec, win.cur.y, win);
            }
        }
    }

    fn mod_pre_row_num(&mut self, str_vec: &mut Vec<String>, y: usize, win: &Window) {
        Log::debug_key("move_render_row_num");
        Log::debug("win.area_all_h.0", &win.area_all_h.0);
        Log::debug("yyy", &y);
        Log::debug("win.offset.y", &win.offset.y);
        str_vec.push(format!("{}", MoveTo(win.area_all_h.0 as u16, (win.area_v.0 + y - win.offset.y) as u16)));
        self.set_row_num(y, str_vec, win);
    }

    fn set_row_num(&mut self, i: usize, str_vec: &mut Vec<String>, win: &Window) {
        if CfgEdit::get().general.editor.row_no.is_enable {
            if i == win.cur.y {
                str_vec.push(Colors::get_rownum_curt_fg_bg());
            } else {
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
        for i in self.row_posi..=self.row_posi + self.row_num {
            str_vec.push(format!("{}{}", MoveTo(0, i as u16), Clear(ClearType::CurrentLine)));
        }
        str_vec.push(format!("{}", MoveTo(0, self.get_curt_row_posi() as u16)));
    }

    pub fn clear_all_diff(&self, str_vec: &mut Vec<String>, change_row_vec: &[usize]) {
        Log::debug_key("Editor.clear_all_diff");

        for i in change_row_vec {
            str_vec.push(format!("{}{}", MoveTo(0, (*i - self.win_mgr.curt_ref().offset.y + self.get_curt_row_posi()) as u16), Clear(ClearType::CurrentLine)));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_row_including_extra() && self.buf_len_rows_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.buf_len_rows_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(0, (i + self.get_curt_row_posi()) as u16), Clear(ClearType::CurrentLine)));
            }
        }
    }
}
