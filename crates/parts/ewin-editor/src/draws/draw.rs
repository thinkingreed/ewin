use std::io::Write;

use crate::{editor_gr::*, model::*, window::window::*};
use crossterm::{cursor::*, terminal::*};
use ewin_cfg::{colors::*, log::*, model::general::default::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, view::*},
};
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::char_edit::*;
use ewin_view::menulists::core::*;

impl Editor {
    pub fn draw(str_vec: &mut Vec<String>, draw_cache_vecs: &mut [Vec<EditorDrawCache>], draw_parts: &DrawParts) {
        Log::debug_key("Editor::draw");

        Log::debug("draw_parts", &draw_parts);

        if matches!(draw_parts, DrawParts::TabsAllCacheClear) {
            for vecs in draw_cache_vecs.iter_mut() {
                for draw_cache in vecs.iter_mut() {
                    *draw_cache = EditorDrawCache::default();
                }
            }
        }
        let draw_range = if (matches!(draw_parts, DrawParts::TabsAll) || matches!(draw_parts, DrawParts::TabsAllCacheClear) || matches!(draw_parts, DrawParts::TabsAllMsgBar(_))) {
            E_DrawRange::All
        } else if let DrawParts::Editor(e_draw_range) = draw_parts {
            *e_draw_range
        } else {
            E_DrawRange::All
        };

        // Editor
        match draw_range {
            E_DrawRange::Not => {}
            _ => {
                let curt_row_len = EditorGr::get().curt_ref().get_curt_row_len();
                if curt_row_len > 0 {
                    let curt_win = EditorGr::get().curt_ref().get_curt_ref_win().clone();
                    if draw_range == E_DrawRange::MoveCur {
                        let split_line_v = EditorGr::get().curt_ref().win_mgr.split_line_v;
                        EditorGr::get().curt_ref().draw_move_cur(str_vec, &curt_win, split_line_v);
                        return;
                    }

                    let vec = EditorGr::get().curt_ref().win_mgr.win_list.clone();
                    for (v_idx, vec_v) in vec.iter().enumerate() {
                        for (h_idx, win) in vec_v.iter().enumerate() {
                            if draw_range == E_DrawRange::WinOnlyAll && &curt_win != win {
                                continue;
                            }
                            Editor::draw_cache(&mut draw_cache_vecs[v_idx][h_idx], EditorGr::get().curt_ref(), win, draw_range);
                            EditorGr::get().curt_ref().draw_main(str_vec, &draw_cache_vecs[v_idx][h_idx], win, draw_range);
                            draw_cache_vecs[v_idx][h_idx].cells_from = std::mem::take(&mut draw_cache_vecs[v_idx][h_idx].cells_to);

                            win.draw_scale(str_vec, EditorGr::get().curt_ref().win_mgr.split_line_v);
                            win.scrl_v.draw(str_vec, &win.view, Colors::get_default_bg());
                            win.scrl_h.draw(str_vec, &win.view);
                        }
                    }
                }
                EditorGr::get().curt_ref().draw_window_split_line(str_vec);
                EditorGr::get().curt_mut().change_info.clear();

                str_vec.push(Colors::get_default_fg_bg());
            }
        };
    }

    pub fn draw_only<T: Write>(out: &mut T, draw_cache_vecs: &mut Vec<Vec<EditorDrawCache>>, draw_parts: &DrawParts) {
        Log::debug_key("MsgBar.draw_only");

        let mut v: Vec<String> = vec![];
        Editor::draw(&mut v, draw_cache_vecs, draw_parts);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
    pub fn draw_main(&self, str_vec: &mut Vec<String>, draw: &EditorDrawCache, win: &Window, draw_range: E_DrawRange) {
        Log::info_key("Editor.draw");
        Log::debug("win", &win);

        Log::debug("win.sel", &win.sel);

        // Clear init
        let act_type = self.clear_init(str_vec, draw, win, draw_range);
        if act_type != ActType::Next {
            return;
        }
        let (mut y, mut x_width) = (0, 0);

        // Run only for curt window
        if self.win_mgr.curt_ref() == win {
            self.draw_pre_row_num(str_vec, win);
        }

        for i in draw.change_row_vec.iter() {
            str_vec.push(format!("{}", MoveTo((win.view.x - self.get_rnw_and_margin()) as u16, (*i - win.offset.y + win.view.y) as u16)));
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
                if State::get().curt_ref_state().editor.mouse == Mouse::Enable {
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

    #[allow(clippy::nonminimal_bool)]
    pub fn clear_init(&self, str_vec: &mut Vec<String>, draw: &EditorDrawCache, win: &Window, draw_range: E_DrawRange) -> ActType {
        Log::debug_key("clear_init");

        str_vec.push(Colors::get_default_fg_bg());
        let curt_win = self.get_curt_ref_win();

        match draw_range {
            E_DrawRange::Not | E_DrawRange::MoveCur => return ActType::Cancel,
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
            E_DrawRange::ScrollDown(_, _) => str_vec.push(format!("{}{}{}", ScrollUp(1), MoveTo(0, (curt_win.view.y_height() - Editor::SCROLL_UP_DOWN_MARGIN) as u16), Clear(ClearType::FromCursorDown))),
            E_DrawRange::ScrollUp(_, _) => str_vec.push(format!("{}{}{}", ScrollDown(1), MoveTo(0, curt_win.view.y as u16), Clear(ClearType::CurrentLine))),
        };
        return ActType::Next;
    }

    pub fn draw_pre_row_num(&self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("draw_row_num");
        // If you need to edit the previous row_num
        if State::get().curt_ref_state().editor.row_no.is_enable {
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
        Log::debug("win.area_all_h.0", &win.view_all.x);
        Log::debug("yyy", &y);
        Log::debug("win.offset.y", &win.offset.y);
        str_vec.push(format!("{}", MoveTo(win.view_all.x as u16, (win.view.y + y - win.offset.y) as u16)));
        self.set_row_num(y, str_vec, win);
    }

    fn set_row_num(&self, i: usize, str_vec: &mut Vec<String>, win: &Window) {
        if State::get().curt_ref_state().editor.row_no.is_enable {
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

    pub fn draw_move_cur(&self, str_vec: &mut Vec<String>, win: &Window, split_line_v: usize) {
        win.scrl_h.draw(str_vec, &win.view);
        win.scrl_v.draw(str_vec, &win.view, Colors::get_default_bg());

        let curt_win = self.get_curt_ref_win().clone();
        self.draw_pre_row_num(str_vec, &curt_win);
        win.draw_scale(str_vec, split_line_v);
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
            str_vec.push(format!("{}{}", MoveTo(win.view.x as u16, (*i - win.offset.y + win.view.y) as u16), get_space(win.width())));
        }
        // Clear the previously displayed part when the number of lines becomes shorter than the height of the screen
        if self.buf.len_rows() <= self.get_disp_row_including_extra() && self.buf_len_rows_org > self.buf.len_rows() {
            for i in self.buf.len_rows() - 1..=self.buf_len_rows_org - 1 {
                str_vec.push(format!("{}{}", MoveTo(win.view.x as u16, (i + win.view.y) as u16), get_space(win.width())));
            }
        }
    }

    pub fn clear_target(&self, str_vec: &mut Vec<String>, sy: usize, ey: usize, win: &Window) {
        Log::debug_key("Editor.clear_target");
        Log::debug("sy", &sy);
        Log::debug("ey", &ey);
        Log::debug("win", &win);
        // for e_cmd::AllSelect
        if win.offset.y <= sy || ey <= win.offset.y + win.view.y_height() {
            let start_y = if sy >= win.offset.y { sy - win.offset.y } else { win.offset.y };
            let end_y = if ey <= win.offset.y + win.view.y_height() { ey - win.offset.y } else { win.offset.y + win.view.y_height() };
            Log::debug("start_y", &start_y);
            Log::debug("end_y", &end_y);
            for i in start_y..=end_y {
                str_vec.push(format!("{}{}", MoveTo(win.view_all.x as u16, (i + win.view.y) as u16), get_space(win.width_all())));
            }
            str_vec.push(format!("{}", MoveTo(win.view.x as u16, (start_y + win.view.y) as u16)));
        }
    }

    pub fn draw_input_comple(&mut self, str_vec: &mut Vec<String>) {
        self.input_comple.draw(str_vec);
    }
}
