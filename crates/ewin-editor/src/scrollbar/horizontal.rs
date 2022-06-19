use crate::model::*;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ewin_cfg::{colors::*, log::*, model::default::*};
use ewin_com::{_cfg::key::cmd::*, util::*};
use ewin_const::def::*;
use std::{
    cmp::{max, min},
    collections::BTreeSet,
};
use unicode_width::UnicodeWidthStr;

impl Editor {
    // Including new line code
    const SCROLL_BAR_H_END_LINE_MARGIN: usize = 4;

    pub fn calc_editor_scrlbar_h(&mut self) {
        Log::debug_key("calc_scrlbar_h_row");

        self.scrl_h.row_width_chars_vec = vec![(0, 0); self.buf.len_rows()];
        for i in 0..self.buf.len_rows() {
            self.scrl_h.row_width_chars_vec[i] = (self.buf.line(i).to_string().width() + Editor::SCROLL_BAR_H_END_LINE_MARGIN, self.buf.line(i).len_chars() + Editor::SCROLL_BAR_H_END_LINE_MARGIN);

            if self.scrl_h.row_width_chars_vec[i].0 > self.scrl_h.row_max_width {
                self.scrl_h.row_max_width_idx = i;
                self.scrl_h.row_max_width = self.scrl_h.row_width_chars_vec[i].0;
                self.scrl_h.row_max_chars = self.scrl_h.row_width_chars_vec[i].1;
                if self.scrl_h.row_max_chars > self.scrl_h.row_max_width {
                    self.scrl_h.row_max_width = self.scrl_h.row_max_chars;
                }
            }
        }
    }

    pub fn recalc_scrlbar_h(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("recalc_scrlbar_h");
        for i in idxs {
            if self.scrl_h.row_width_chars_vec.get(i).is_some() {
                self.scrl_h.row_width_chars_vec[i] = (self.buf.line(i).to_string().width() + Editor::SCROLL_BAR_H_END_LINE_MARGIN, self.buf.line(i).len_chars() + Editor::SCROLL_BAR_H_END_LINE_MARGIN);
            }
        }

        if !self.scrl_h.row_width_chars_vec.is_empty() {
            self.scrl_h.row_max_width = self.scrl_h.row_width_chars_vec.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0;
            self.scrl_h.row_max_width_idx = self.scrl_h.row_width_chars_vec.iter().position(|(x, _)| x == &self.scrl_h.row_max_width).unwrap();
            self.scrl_h.row_max_chars = self.buf.char_vec_row(self.scrl_h.row_max_width_idx).len();
        }
        self.scrl_h.is_show = self.scrl_h.row_max_width > self.col_len;
    }

    pub fn set_scrlbar_h_posi(&mut self, x: usize) {
        Log::debug_key("set_scrlbar_h_posi");

        // MouseDownLeft
        if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) {
            // self.calc_scrlbar_h();
            self.scrl_h.is_enable = true;
            // Except on scrl_h
            if self.get_rnw_and_margin() - 1 <= x && x < self.get_rnw_and_margin() + self.col_len {
                // Excluded if within bar range
                if !(self.get_rnw_and_margin() + self.scrl_h.clm_posi <= x && x < self.get_rnw_and_margin() + self.scrl_h.clm_posi + self.scrl_h.bar_len) {
                    self.scrl_h.clm_posi = if x + self.scrl_h.bar_len < self.get_rnw_and_margin() + self.col_len {
                        if x >= self.get_rnw_and_margin() {
                            x - self.get_rnw_and_margin()
                        } else {
                            0
                        }
                    } else {
                        self.scrl_h.scrl_range
                    };
                } else {
                    return;
                }
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftLeft(_, _)) {
            if 0 < self.scrl_h.clm_posi {
                self.offset_x = if self.offset_x >= self.scrl_h.move_char_x { self.offset_x - self.scrl_h.move_char_x } else { 0 };
                self.scrl_h.clm_posi -= 1;
            } else {
                self.offset_x = 0;
            };
        } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftRight(_, _)) {
            if self.scrl_h.clm_posi < self.scrl_h.scrl_range {
                // Last move
                if self.scrl_h.clm_posi + 1 == self.scrl_h.scrl_range {
                    if self.buf.char_vec_row(self.scrl_h.row_max_width_idx).len() > self.offset_x {
                        if let Some(disp_cur_x) = get_row_x_opt(&self.buf.char_vec_range(self.scrl_h.row_max_width_idx, self.offset_x..), self.col_len, true, true) {
                            let move_cur_x = self.scrl_h.row_max_chars - (self.offset_x + disp_cur_x);
                            self.offset_x += move_cur_x;
                        }
                    }
                } else {
                    self.offset_x += self.scrl_h.move_char_x;
                }
                self.scrl_h.clm_posi += 1;
            }
        } else if (matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) || matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _))) {
        }
        self.set_offset_disp_x();
    }

    pub fn draw_scrlbar_h(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_scrlbar_h");

        if self.scrl_h.is_show {
            if self.scrl_h.bar_len == USIZE_UNDEFINED || self.scrl_h.row_max_width_org != self.scrl_h.row_max_width || self.col_len != self.col_len_org {
                self.scrl_h.bar_len = max(2, min(self.col_len - 1, (self.col_len as f64 / self.scrl_h.row_max_width as f64 * self.col_len as f64).floor() as usize));

                if self.scrl_h.row_max_width > self.col_len {
                    self.scrl_h.is_show = true;
                    self.scrl_h.scrl_range = self.col_len - self.scrl_h.bar_len;
                    let rate = self.scrl_h.row_max_width as f64 / self.scrl_h.row_max_chars as f64;
                    let move_cur_x = ((self.scrl_h.row_max_width - self.col_len) as f64 / self.scrl_h.scrl_range as f64 / rate).ceil() as usize;
                    self.scrl_h.move_char_x = if move_cur_x == 0 { 1 } else { move_cur_x };
                }
            }

            if !self.scrl_h.is_enable {
                Log::debug("self.cur.disp_x", &(self.cur.disp_x));
                Log::debug("self.offset_disp_x", &(self.offset_disp_x));
                Log::debug("self.scrl_h.row_max_width", &self.scrl_h.row_max_width);
                Log::debug("self.scrl_h.bar_len", &self.scrl_h.bar_len);
                Log::debug("self.col_len", &self.col_len);
                Log::debug("self.scrl_h.scrl_range ", &self.scrl_h.scrl_range);
                Log::debug("self.scrl_h.clm_posi 111", &self.scrl_h.clm_posi);
                self.scrl_h.clm_posi = min(self.scrl_h.scrl_range, (self.scrl_h.scrl_range as f64 * self.offset_disp_x as f64 / (self.scrl_h.row_max_width - self.col_len) as f64).ceil() as usize);
                Log::debug("self.scrl_h.clm_posi 222", &self.scrl_h.clm_posi);
            }

            let height = Cfg::get().general.editor.scrollbar.horizontal.height;
            for i in self.scrl_h.row_posi..self.scrl_h.row_posi + height {
                str_vec.push(format!("{}{}", MoveTo(0, self.scrl_h.row_posi as u16), Clear(ClearType::CurrentLine)));
                str_vec.push(Colors::get_default_bg());
                str_vec.push(MoveTo((self.get_rnw_and_margin() + self.scrl_h.clm_posi) as u16, i as u16).to_string());
                str_vec.push(Colors::get_scrollbar_h_bg());
                str_vec.push(" ".to_string().repeat(self.scrl_h.bar_len));
                str_vec.push(Colors::get_default_bg());
            }
        }
    }
}

#[cfg(test)]
mod tests {}
