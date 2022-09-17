use crate::{model::*, window::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::models::view::*;
use std::collections::BTreeSet;
use unicode_width::UnicodeWidthStr;

impl Editor {
    // Including new line code
    const SCROLL_BAR_H_END_LINE_MARGIN: usize = 4;

    pub fn calc_editor_scrlbar_h(&mut self) {
        Log::debug_key("calc_editor_scrlbar_h");

        self.win_mgr.curt().scrl_h.is_show = self.win_mgr.row_max_width > self.get_curt_col_len();

        if self.win_mgr.curt().scrl_h.is_show {
            for vec_v in self.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    Log::debug("win.width()", &win.width());

                    win.scrl_h.calc_scrlbar_h(win.width(), self.win_mgr.row_max_width, self.win_mgr.row_max_chars, win.offset.disp_x);
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

    pub fn draw_scrlbar_h(&self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("draw_scrlbar_h");

        if win.scrl_h.is_show {
            for i in win.scrl_h.view.y..win.scrl_h.view.y + win.scrl_h.bar_height {
                str_vec.push(format!("{}{}", MoveTo(win.area_h.0 as u16, win.scrl_h.view.y as u16), get_space(win.width())));
                str_vec.push(Colors::get_default_bg());
                str_vec.push(MoveTo((win.area_h.0 + win.scrl_h.view.x) as u16, i as u16).to_string());
                str_vec.push(Colors::get_scrollbar_h_bg());
                str_vec.push(" ".to_string().repeat(win.scrl_h.bar_len));
                str_vec.push(Colors::get_default_bg());
            }
        }
    }
}

#[cfg(test)]
mod tests {}
