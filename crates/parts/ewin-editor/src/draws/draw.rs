use std::io::Write;

use crate::{model::*, window::*};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*, model::general::default::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, view::*},
};
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::char_edit::*;

impl Editor {
    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        // Editor
        match self.draw_range {
            E_DrawRange::Not => {}
            _ => {
                if self.get_curt_row_len() > 0 {
                    let curt_win = self.get_curt_ref_win().clone();
                    if self.draw_range == E_DrawRange::MoveCur {
                        self.draw_move_cur(str_vec, &curt_win);
                        return;
                    }

                    let vec = self.win_mgr.win_list.clone();
                    for (v_idx, vec_v) in vec.iter().enumerate() {
                        for (h_idx, win) in vec_v.iter().enumerate() {
                            if self.draw_range == E_DrawRange::WinOnlyAll && &curt_win != win {
                                continue;
                            }
                            self.draw_cache(win);
                            self.draw_main(str_vec, &self.draw_cache[v_idx][h_idx], win);
                            self.draw_cache[v_idx][h_idx].cells_from = std::mem::take(&mut self.draw_cache[v_idx][h_idx].cells_to);
                            self.draw_scale(str_vec, win);
                            self.draw_scrlbar_v(str_vec, win);
                            self.draw_scrlbar_h(str_vec, win);
                        }
                    }
                }
                self.draw_window_split_line(str_vec);

                self.change_info.clear();

                str_vec.push(Colors::get_default_fg_bg());
            }
        };
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    #[allow(clippy::nonminimal_bool)]
    pub fn clear_init(&self, str_vec: &mut Vec<String>, draw: &EditorDraw, win: &Window) -> ActType {
        Log::debug_key("clear_init");

        let curt_win = self.get_curt_ref_win();
        Log::debug("editor.draw_range ", &self.draw_range);

        match self.draw_range {
            E_DrawRange::Not | E_DrawRange::MoveCur => return ActType::Cancel,
            //  E_DrawRange::Init | E_DrawRange::All if win.h_idx == 0 && win.v_idx == 0 && editor.win_mgr.split_type == WindowSplitType::None => editor.clear_all(str_vec),
            E_DrawRange::All => {
                if win.h_idx == 0 && win.v_idx == 0 {
                    self.clear_all(str_vec);
                }
            }
            E_DrawRange::WinOnlyAll => {
                if curt_win == win {
                    win.clear_draw(str_vec);
                }
            }
            E_DrawRange::TargetRange(sy, ey) => {
                if curt_win == win || !Editor::is_disp_state_normal() {
                    self.clear_target(str_vec, sy, ey, win);
                }
            }
            E_DrawRange::After(_) | E_DrawRange::Targetpoint => self.clear_all_diff(str_vec, &draw.change_row_vec, win),
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (curt_win.area_v.1 - Editor::SCROLL_UP_DOWN_MARGIN) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, curt_win.area_v.0 as u16), Clear(ClearType::CurrentLine))),
        };
        return ActType::Next;
    }

    pub fn draw_main(&self, str_vec: &mut Vec<String>, draw: &EditorDraw, win: &Window) {
        Log::info_key("Editor.draw");
        Log::debug("self.draw_range", &self.draw_range);
        Log::debug("win", &win);

        // Clear init
        let act_type = self.clear_init(str_vec, draw, win);
        if act_type != ActType::Next {
            return;
        }
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
                let width = get_char_width(&cell.c, x_width);
                if x_width + width > win.width() {
                    break;
                }
                x_width += width;
                if State::get().curt_state().editor.mouse == Mouse::Enable {
                    match cell.c {
                        NEW_LINE_LF => str_vec.push(if c_org == NEW_LINE_CR { NEW_LINE_CRLF_MARK.to_string() } else { NEW_LINE_LF_MARK.to_string() }),
                        NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(format!("{}{}", Cfg::get().general.view.tab_characters_as_symbols, get_space(width - 1))),
                        FULL_SPACE => str_vec.push(Cfg::get().general.view.full_width_space_characters_as_symbols.to_string()),
                        _ => str_vec.push(cell.c.to_string()),
                    }
                } else {
                    match cell.c {
                        NEW_LINE_LF | NEW_LINE_CR => {}
                        TAB_CHAR => str_vec.push(get_space(width)),
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
        if State::get().curt_state().editor.row_no.is_enable {
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
        Log::debug("win.area_all_h.0", &win.area_h_all.0);
        Log::debug("yyy", &y);
        Log::debug("win.offset.y", &win.offset.y);
        str_vec.push(format!("{}", MoveTo(win.area_h_all.0 as u16, (win.area_v.0 + y - win.offset.y) as u16)));
        self.set_row_num(y, str_vec, win);
    }

    fn set_row_num(&self, i: usize, str_vec: &mut Vec<String>, win: &Window) {
        if State::get().curt_state().editor.row_no.is_enable {
            if i == win.cur.y {
                str_vec.push(Colors::get_rownum_curt_fg_bg());
            } else {
                str_vec.push(Colors::get_rownum_not_curt_fg_bg());
            }
            if self.get_rnw() > 0 {
                str_vec.push(get_space(self.get_rnw() - (i + 1).to_string().len()));
            }
            str_vec.push((i + 1).to_string());

            #[allow(clippy::repeat_once)]
            str_vec.push(" ".to_string().repeat(Editor::RNW_MARGIN));
            str_vec.push(Colors::get_default_fg_bg())
        }
    }

    pub fn draw_move_cur(&self, str_vec: &mut Vec<String>, win: &Window) {
        self.draw_scrlbar_h(str_vec, win);
        self.draw_scrlbar_v(str_vec, win);
        let curt_win = self.get_curt_ref_win().clone();
        self.draw_pre_row_num(str_vec, &curt_win);
        self.draw_scale(str_vec, win);
    }

    pub fn clear_all(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("Editor.clear_all");

        for i in self.view.y..self.view.y + self.view.height - 1 {
            str_vec.push(format!("{}{}", MoveTo(self.view.x as u16, i as u16), get_space(self.view.width)));
        }
        str_vec.push(format!("{}", MoveTo(self.view.x as u16, self.get_curt_row_posi() as u16)));
    }

    pub fn clear_all_diff(&self, str_vec: &mut Vec<String>, change_row_vec: &[usize], win: &Window) {
        Log::debug_key("Editor.clear_all_diff");
        for i in change_row_vec {
            str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, (*i - win.offset.y + win.area_v.0) as u16), get_space(win.width())));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_row_including_extra() && self.buf_len_rows_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.buf_len_rows_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, (i + win.area_v.0) as u16), get_space(win.width())));
            }
        }
    }

    pub fn clear_target(&self, str_vec: &mut Vec<String>, sy: usize, ey: usize, win: &Window) {
        Log::debug_key("Editor.clear_target");
        Log::debug("sy", &sy);
        Log::debug("ey", &ey);
        Log::debug("win", &win);
        // for e_cmd::AllSelect
        if win.offset.y <= sy || ey <= win.offset.y + win.area_v.1 {
            let start_y = if sy >= win.offset.y { sy - win.offset.y } else { win.offset.y };
            let end_y = if ey <= win.offset.y + win.area_v.1 { ey - win.offset.y } else { win.offset.y + win.area_v.1 };
            Log::debug("start_y", &start_y);
            Log::debug("end_y", &end_y);
            for i in start_y..=end_y {
                str_vec.push(format!("{}{}", MoveTo(win.area_h_all.0 as u16, (i + win.area_v.0) as u16), get_space(win.width_all())));
            }
            str_vec.push(format!("{}", MoveTo(win.area_h.0 as u16, (start_y + win.area_v.0) as u16)));
        }
    }
}
