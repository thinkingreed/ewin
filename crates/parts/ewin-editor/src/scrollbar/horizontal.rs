use crate::{model::*, window::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_key::key::cmd::*;

use ewin_utils::char_edit::*;
use std::{
    cmp::{max, min},
    collections::BTreeSet,
};
use unicode_width::UnicodeWidthStr;

impl Editor {
    pub fn calc_editor_scrlbar(&mut self) {
        self.calc_editor_scrlbar_h();
        self.calc_editor_scrlbar_v();
    }

    // Including new line code
    const SCROLL_BAR_H_END_LINE_MARGIN: usize = 4;

    pub fn calc_editor_scrlbar_h(&mut self) {
        Log::debug_key("calc_editor_scrlbar_h");

        self.win_mgr.curt().scrl_h.is_show = self.win_mgr.row_max_width > self.get_curt_col_len();

        if self.win_mgr.curt().scrl_h.is_show {
            for vec_v in self.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    if win.scrl_h.bar_len == 0 || self.win_mgr.row_max_width_org != self.win_mgr.row_max_width || win.area_h.1 != win.area_h_org.1 {
                        Log::debug_s("calc_editor_scrlbar_h reset");
                        win.scrl_h.bar_len = max(2, min(win.width() - 1, (win.width() as f64 / self.win_mgr.row_max_width as f64 * win.width() as f64).floor() as usize));

                        if self.win_mgr.row_max_width > win.width() {
                            win.scrl_h.scrl_range = win.width() - win.scrl_h.bar_len;
                            let rate = self.win_mgr.row_max_width as f64 / self.win_mgr.row_max_chars as f64;
                            let move_cur_x = ((self.win_mgr.row_max_width - win.width()) as f64 / win.scrl_h.scrl_range as f64 / rate).ceil() as usize;
                            win.scrl_h.move_char_x = if move_cur_x == 0 { 1 } else { move_cur_x };
                        }
                    }
                    if !win.scrl_h.is_enable {
                        win.scrl_h.clm_posi = min(win.scrl_h.scrl_range, (win.scrl_h.scrl_range as f64 * win.offset.disp_x as f64 / (self.win_mgr.row_max_width - win.width()) as f64).ceil() as usize);
                    }
                }
            }
        }
    }

    pub fn init_editor_scrlbar_h(&mut self) {
        Log::debug_key("calc_scrlbar_h_row");

        self.win_mgr.row_width_chars_vec = vec![(0, 0); self.buf.len_rows()];
        for i in 0..self.buf.len_rows() {
            self.win_mgr.row_width_chars_vec[i] = (self.buf.line(i).to_string().width() + Editor::SCROLL_BAR_H_END_LINE_MARGIN, self.buf.line(i).len_chars() + Editor::SCROLL_BAR_H_END_LINE_MARGIN);

            if self.win_mgr.row_width_chars_vec[i].0 > self.win_mgr.row_max_width {
                self.win_mgr.row_max_width_idx = i;
                self.win_mgr.row_max_width = self.win_mgr.row_width_chars_vec[i].0;
                self.win_mgr.row_max_chars = self.win_mgr.row_width_chars_vec[i].1;
                if self.win_mgr.row_max_chars > self.win_mgr.row_max_width {
                    self.win_mgr.row_max_width = self.win_mgr.row_max_chars;
                }
            }
        }
    }

    pub fn set_row_width_chars_vec(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("recalc_scrlbar_h");
        for i in idxs {
            if self.win_mgr.row_width_chars_vec.get(i).is_some() {
                self.win_mgr.row_width_chars_vec[i] = (self.buf.line(i).to_string().width() + Editor::SCROLL_BAR_H_END_LINE_MARGIN, self.buf.line(i).len_chars() + Editor::SCROLL_BAR_H_END_LINE_MARGIN);
            }
        }
    }

    pub fn calc_editor_row_max(&mut self) {
        if !self.win_mgr.row_width_chars_vec.is_empty() {
            self.win_mgr.row_max_width = self.win_mgr.row_width_chars_vec.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
            self.win_mgr.row_max_width_idx = self.win_mgr.row_width_chars_vec.iter().position(|(x, _)| x == &self.win_mgr.row_max_width).unwrap();
            self.win_mgr.row_max_chars = self.buf.char_vec_row(self.win_mgr.row_max_width_idx).len();
        }
    }

    pub fn set_scrlbar_h_posi(&mut self, x: usize) {
        Log::debug_key("set_scrlbar_h_posi");

        // MouseDownLeft
        if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) {
            // self.calc_scrlbar_h();
            self.win_mgr.curt().scrl_h.is_enable = true;
            // Except on scrl_h
            if self.get_rnw_and_margin() - 1 <= x && x < self.get_rnw_and_margin() + self.get_curt_col_len() {
                // Excluded if within bar range
                if !(self.get_rnw_and_margin() + self.win_mgr.curt().scrl_h.clm_posi <= x && x < self.get_rnw_and_margin() + self.win_mgr.curt().scrl_h.clm_posi + self.win_mgr.curt().scrl_h.bar_len) {
                    self.win_mgr.curt().scrl_h.clm_posi = if x + self.win_mgr.curt().scrl_h.bar_len < self.get_rnw_and_margin() + self.get_curt_col_len() {
                        if x >= self.get_rnw_and_margin() {
                            x - self.get_rnw_and_margin()
                        } else {
                            0
                        }
                    } else {
                        self.win_mgr.curt().scrl_h.scrl_range
                    };
                } else {
                    return;
                }
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) {
            if 0 < self.win_mgr.curt().scrl_h.clm_posi {
                self.win_mgr.curt().offset.x = if self.win_mgr.curt().offset.x >= self.win_mgr.curt().scrl_h.move_char_x { self.win_mgr.curt().offset.x - self.win_mgr.curt().scrl_h.move_char_x } else { 0 };
                self.win_mgr.curt().scrl_h.clm_posi -= 1;
            } else {
                self.win_mgr.curt().offset.x = 0;
            };
        } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) {
            if self.win_mgr.curt().scrl_h.clm_posi < self.win_mgr.curt().scrl_h.scrl_range {
                // Last move
                if self.win_mgr.curt().scrl_h.clm_posi + 1 == self.win_mgr.curt().scrl_h.scrl_range {
                    if self.buf.char_vec_row(self.win_mgr.row_max_width_idx).len() > self.win_mgr.curt().offset.x {
                        if let Some(disp_cur_x) = get_row_x_opt(&self.buf.char_vec_range(self.win_mgr.row_max_width_idx, self.win_mgr.curt().offset.x..), self.get_curt_col_len(), true, true) {
                            let move_cur_x = self.win_mgr.row_max_chars - (self.win_mgr.curt().offset.x + disp_cur_x);
                            self.win_mgr.curt().offset.x += move_cur_x;
                        }
                    }
                } else {
                    self.win_mgr.curt().offset.x += self.win_mgr.curt().scrl_h.move_char_x;
                }
                self.win_mgr.curt().scrl_h.clm_posi += 1;
            }
        } else if (matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _))) {
        }
        self.set_offset_disp_x();
    }

    pub fn draw_scrlbar_h(&self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("draw_scrlbar_h");

        if win.scrl_h.is_show {
            for i in win.scrl_h.row_posi..win.scrl_h.row_posi + win.scrl_h.bar_height {
                str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, win.scrl_h.row_posi as u16), " ".repeat(win.width())));
                str_vec.push(Colors::get_default_bg());
                str_vec.push(MoveTo((win.area_h.0 + win.scrl_h.clm_posi) as u16, i as u16).to_string());
                str_vec.push(Colors::get_scrollbar_h_bg());
                str_vec.push(" ".to_string().repeat(win.scrl_h.bar_len));
                str_vec.push(Colors::get_default_bg());
            }
        }
    }
}

#[cfg(test)]
mod tests {}
