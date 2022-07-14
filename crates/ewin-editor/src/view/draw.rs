use crate::{
    ewin_com::{model::*, util::*},
    model::*,
    window::*,
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

    pub fn clear_init(str_vec: &mut Vec<String>, editor: &Editor, draw: &EditorDraw, win: &Window) -> ActType {
        Log::debug_key("clear_init");
        let curt_win = editor.get_curt_ref_win();

        #[allow(clippy::nonminimal_bool)]
        if !(win.h_idx == 0 && win.v_idx == 0) && !(matches!(curt_win.draw_range, E_DrawRange::TargetRange(_, _)) || matches!(curt_win.draw_range, E_DrawRange::Targetpoint) || matches!(curt_win.draw_range, E_DrawRange::After(_))) {
            return ActType::Next;
        }

        match win.draw_range {
            E_DrawRange::Not | E_DrawRange::MoveCur => return ActType::Cancel,
            E_DrawRange::TargetRange(sy, ey) => editor.clear_target(str_vec, sy, ey, win),
            E_DrawRange::After(_) | E_DrawRange::Targetpoint => editor.clear_all_diff(str_vec, &draw.change_row_vec, win),
            E_DrawRange::Init | E_DrawRange::All => editor.clear_all(str_vec),
            E_DrawRange::ScrollDown(_, _) | E_DrawRange::ScrollUp(_, _) if editor.win_mgr.split_type != WindowSplitType::None => editor.clear_all(str_vec),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (curt_win.area_v.1 - Editor::SCROLL_UP_DOWN_MARGIN) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, curt_win.area_v.0 as u16), Clear(ClearType::CurrentLine))),
        }
        return ActType::Next;
    }

    pub fn draw(&self, str_vec: &mut Vec<String>, draw: &EditorDraw, win: &Window) {
        Log::info_key("Editor.draw");
        Log::debug("win", &win);
        Log::debug("draw.change_row_vec", &draw.change_row_vec);

        let (mut y, mut x_width) = (0, 0);

        // Run only for curt window
        if self.win_mgr.curt_ref() == win {
            self.draw_pre_row_num(str_vec, win);
        }

        for i in draw.change_row_vec.iter() {
            Log::debug("iii", &i);
            Log::debug("win.area_h", &win.area_h);
            Log::debug("win.offset", &win.offset);
            Log::debug("win.area_v", &win.area_v);

            str_vec.push(format!("{}", MoveTo((win.area_h.0 - self.get_rnw_and_margin()) as u16, (*i - win.offset.y + win.area_v.0) as u16)));
            self.set_row_num(*i, str_vec, win);
            let row_cell = &draw.cells_to[i];

            let mut c_org = ' ';
            for (x_idx, cell) in (0_usize..).zip(row_cell) {
                cell.draw_style(str_vec, x_idx == 0 && win.offset.x > 0);
                let width = get_char_width(&cell.c, x_width);
                if x_width + width > win.width() {
                    break;
                }
                x_width += width;
                if self.state.mouse == Mouse::Enable {
                    match cell.c {
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", Cfg::get().general.view.tab_characters_as_symbols, " ".repeat(width - 1))),
                        FULL_SPACE => str_vec.push(Cfg::get().general.view.full_width_space_characters_as_symbols.to_string()),

                        _ => str_vec.push(cell.c.to_string()),
                    }
                } else {
                    match cell.c {
                        NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(" ".repeat(width)),
                        FULL_SPACE => str_vec.push(Cfg::get().general.view.full_width_space_characters_as_symbols.to_string()),
                        _ => str_vec.push(cell.c.to_string()),
                    }
                }
                c_org = cell.c;
            }

            y += 1;
            x_width = 0;

            if y >= win.height() {
                break;
            }
        }
    }

    pub fn draw_pre_row_num(&self, str_vec: &mut Vec<String>, win: &Window) {
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

    fn mod_pre_row_num(&self, str_vec: &mut Vec<String>, y: usize, win: &Window) {
        Log::debug_key("move_render_row_num");
        Log::debug("win.area_all_h.0", &win.area_all_h.0);
        Log::debug("yyy", &y);
        Log::debug("win.offset.y", &win.offset.y);
        str_vec.push(format!("{}", MoveTo(win.area_all_h.0 as u16, (win.area_v.0 + y - win.offset.y) as u16)));
        self.set_row_num(y, str_vec, win);
    }

    fn set_row_num(&self, i: usize, str_vec: &mut Vec<String>, win: &Window) {
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

    pub fn clear_all_diff(&self, str_vec: &mut Vec<String>, change_row_vec: &[usize], win: &Window) {
        Log::debug_key("Editor.clear_all_diff");
        for i in change_row_vec {
            str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, (*i - win.offset.y + win.area_v.0) as u16), " ".repeat(win.width())));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_row_including_extra() && self.buf_len_rows_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.buf_len_rows_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, (i + win.area_v.0) as u16), " ".repeat(win.width())));
            }
        }
    }
    pub fn clear_target(&self, str_vec: &mut Vec<String>, sy: usize, ey: usize, win: &Window) {
        Log::debug_key("Editor.clear_target");
        Log::debug("sy", &sy);
        Log::debug("ey", &ey);
        Log::debug("win", &win);
        // for e_cmd::AllSelect
        if win.offset.y <= sy && ey <= win.offset.y + win.area_v.1 {
            let start_y = if sy >= win.offset.y { sy - win.offset.y } else { win.offset.y };
            let end_y = if ey <= win.offset.y + win.area_v.1 { ey - win.offset.y } else { win.offset.y + win.area_v.1 };
            for i in start_y..=end_y {
                str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, (i + win.area_v.0) as u16), " ".repeat(win.width())));
            }
            str_vec.push(format!("{}", MoveTo(win.area_h.0 as u16, (start_y + win.area_v.0) as u16)));
        }
    }
}
