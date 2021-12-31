use crate::model::*;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ewin_com::{_cfg::key::keycmd::*, colors::*, def::*, global::*, log::*, model::*, util::*};
use std::{cmp::max, time::Instant};
use unicode_width::UnicodeWidthStr;

impl Editor {
    // Including new line code
    const SCROLL_BAR_H_END_LINE_MARGIN: usize = 4;

    pub fn calc_scrlbar_h_row(&mut self) {
        Log::debug_key("calc_scrlbar_h_row");

        let start = Instant::now();
        self.scrl_h.row_width_vec = vec![0; self.buf.len_rows()];
        self.scrl_h.row_chars_vec = vec![0; self.buf.len_rows()];
        for i in 0..self.buf.len_rows() {
            let width = self.buf.line(i).to_string().width();
            // for new line code and margin
            self.scrl_h.row_width_vec[i] = width + Editor::SCROLL_BAR_H_END_LINE_MARGIN;
            self.scrl_h.row_chars_vec[i] = self.buf.line(i).len_chars() + Editor::SCROLL_BAR_H_END_LINE_MARGIN;
            if i == self.buf.len_rows() - 1 {
                self.scrl_h.row_width_vec[i] -= 1;
                self.scrl_h.row_chars_vec[i] -= 1;
            }

            if self.scrl_h.row_width_vec[i] > self.scrl_h.row_max_width {
                self.scrl_h.max_width_row_idx = i;
                self.scrl_h.row_max_width = self.scrl_h.row_width_vec[i];
                self.scrl_h.row_max_chars = self.scrl_h.row_chars_vec[i];
                if self.scrl_h.row_max_chars > self.scrl_h.row_max_width {
                    self.scrl_h.row_max_width = self.scrl_h.row_max_chars;
                }
            }
        }
        Log::debug("self.scrl_h.row_max_width", &self.scrl_h.row_max_width);
        let end = start.elapsed();
        Log::debug("calc 経過時間", &format!("{}.{:03}", end.as_secs(), end.subsec_nanos() / 1_000_000));
    }

    pub fn recalc_scrlbar_h_row(&mut self, evt_proc: &EvtProc) {
        Log::debug_key("recalc_scrlbar_h_row");

        Log::debug("evt_proc", &evt_proc);
        let start = Instant::now();

        if let Some(sel_proc) = &evt_proc.sel_proc {
            Log::debug("sel_proc self.scrl_h.row_width_vec 111", &self.scrl_h.row_width_vec);
            for i in sel_proc.cur_s.y..sel_proc.cur_e.y {
                Log::debug("iiiii", &i);
                self.scrl_h.row_width_vec.remove(i);
            }
            Log::debug("sel_proc self.scrl_h.row_width_vec 222", &self.scrl_h.row_width_vec);
        };

        if let Some(evt_proc) = &evt_proc.evt_proc {
            Log::debug("evt_proc self.scrl_h.row_width_vec 111", &self.scrl_h.row_width_vec);
            match evt_proc.e_cmd {
                E_Cmd::DelNextChar => {
                    if is_line_end_str(&evt_proc.str) {
                        self.scrl_h.row_width_vec.remove(evt_proc.cur_s.y + 1);
                    } else {
                        self.scrl_h.row_width_vec.remove(evt_proc.cur_s.y);
                    }
                }
                _ => {}
            }
            Log::debug("evt_proc self.scrl_h.row_width_vec 222", &self.scrl_h.row_width_vec);
        };

        self.scrl_h.row_max_width = *self.scrl_h.row_width_vec.iter().max().unwrap();
        self.scrl_h.max_width_row_idx = self.scrl_h.row_width_vec.iter().position(|x| x == &self.scrl_h.row_max_width).unwrap();
        self.scrl_h.row_max_chars = self.buf.char_vec_line(self.scrl_h.max_width_row_idx).len();

        Log::debug("self.scrl_h.row_max_width", &self.scrl_h.row_max_width);
        Log::debug("self.scrl_h.max_width_row_idx", &self.scrl_h.max_width_row_idx);
        Log::debug("self.scrl_h.row_max_chars", &self.scrl_h.row_max_chars);

        let end = start.elapsed();
        Log::debug("recalc 経過時間", &format!("{}.{:03}", end.as_secs(), end.subsec_nanos() / 1_000_000));
    }

    pub fn set_scrlbar_h_posi(&mut self, x: usize) {
        Log::debug_key("set_cur_scrlbar_h");

        Log::debug("xxx", &x);
        Log::debug("self.scrl_h.scrl_range", &self.scrl_h.scrl_range);
        Log::debug("self.col_len", &self.col_len);
        Log::debug("self.scrl_h.row_max_width", &self.scrl_h.row_max_width);
        Log::debug("self.offset_x", &self.offset_x);
        Log::debug("self.scrl_h.bar_len", &self.scrl_h.bar_len);
        Log::debug("self.scrl_h.clm_posi", &self.scrl_h.clm_posi);

        // MouseDownLeft
        if matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) {
            self.calc_scrlbar_h_row();
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
                }
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if matches!(self.e_cmd, E_Cmd::MouseDragLeftLeft(_, _)) {
            if 0 < self.scrl_h.clm_posi {
                self.offset_x = if self.offset_x >= self.scrl_h.move_cur_x { self.offset_x - self.scrl_h.move_cur_x } else { 0 };
                self.scrl_h.clm_posi -= 1;
            } else {
                self.offset_x = 0;
            };
        } else if matches!(self.e_cmd, E_Cmd::MouseDragLeftRight(_, _)) {
            Log::debug("self.scrl_h.clm_posi MouseDragLeftRight 111", &self.scrl_h.clm_posi);
            Log::debug(" self.offset_x 111", &self.offset_x);
            Log::debug(" self.scrl_h.clm_posi", &self.scrl_h.clm_posi);
            Log::debug(" self.scrl_h.scrl_range", &self.scrl_h.scrl_range);
            Log::debug(" self.offset_x", &self.offset_x);
            if self.scrl_h.clm_posi < self.scrl_h.scrl_range {
                // Last move
                if self.scrl_h.clm_posi + 1 == self.scrl_h.scrl_range {
                    // TODO Offset range must be spec
                    if let Some(disp_cur_x) = get_row_x(&self.buf.char_vec_range(self.scrl_h.max_width_row_idx, self.offset_x..), self.col_len, true, true) {
                        Log::debug("disp_cur_x", &disp_cur_x);
                        Log::debug("self.offset_x", &self.offset_x);
                        let move_cur_x = self.scrl_h.row_max_chars - (self.offset_x + disp_cur_x);
                        Log::debug("move_cur_x", &move_cur_x);
                        self.offset_x += move_cur_x;
                    }
                } else {
                    self.offset_x += self.scrl_h.move_cur_x;
                }

                self.scrl_h.clm_posi += 1;
            }
            Log::debug(" self.offset_x 222", &self.offset_x);
            Log::debug("self.scrl_h.clm_posi MouseDragLeftRight 222", &self.scrl_h.clm_posi);
        } else if (matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _))) {
        }
        self.set_offset_disp_x();

        Log::debug("self.offset_x 222", &self.offset_x);
        Log::debug("self.offset_disp_x", &self.offset_disp_x);
    }

    pub fn draw_scrlbar_h(&mut self, str_vec: &mut Vec<String>) {
        if self.scrl_h.is_show {
            Log::debug("self.col_len", &self.col_len);
            Log::debug("self.scrl_h.row_max_chars", &self.scrl_h.row_max_chars);
            Log::debug("self.scrl_h.row_max_width", &self.scrl_h.row_max_width);

            if self.scrl_h.bar_len == USIZE_UNDEFINED || self.scrl_h.row_max_width_org != self.scrl_h.row_max_width {
                Log::debug(" self.col_len as f64 / self.scrl_h.row_max_width as f64 * self.col_len as f64", &(self.col_len as f64 / self.scrl_h.row_max_width as f64 * self.col_len as f64));
                let len = max(2, (self.col_len as f64 / self.scrl_h.row_max_width as f64 * self.col_len as f64).floor() as usize);
                Log::debug(" lenlenlenlen", &len);
                self.scrl_h.bar_len = len;

                let col_len = get_term_size().0 as usize - self.get_rnw_and_margin();
                if self.scrl_h.row_max_width > col_len {
                    self.scrl_h.is_show = true;
                    self.scrl_h.scrl_range = self.col_len - self.scrl_h.bar_len;
                    Log::debug("self.scrl_h.scrl_range", &self.scrl_h.scrl_range);
                    let rate = self.scrl_h.row_max_width as f64 / self.scrl_h.row_max_chars as f64;
                    Log::debug("rate", &rate);
                    let diff = self.scrl_h.row_max_width - col_len;
                    Log::debug("diff", &diff);
                    Log::debug("diff as f64 / rate / self.scrl_h.scrl_range as f64", &(diff as f64 / rate / self.scrl_h.scrl_range as f64));
                    self.scrl_h.move_cur_x = (diff as f64 / rate / self.scrl_h.scrl_range as f64).round() as usize;
                    Log::debug("self.scrl_h.move_cur_x", &self.scrl_h.move_cur_x);
                }
            }
            Log::debug(" self.scrl_h.bar_len", &self.scrl_h.bar_len);

            let height = CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.horizontal.height;

            for i in self.scrl_h.row_posi..self.scrl_h.row_posi + height {
                str_vec.push(format!("{}{}", MoveTo(0, self.scrl_h.row_posi as u16).to_string(), Clear(ClearType::CurrentLine)));
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
