extern crate ropey;
use crate::model::*;
use std::cmp::min;

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");

        // char_vec initialize
        let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
        if diff > 0 {
            self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
        }

        self.draw.sy = self.offset_y;
        self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();
        if d_range.d_type == DrawType::Target {
            self.draw.sy = d_range.sy;
            self.draw.ey = d_range.ey;
        } else if d_range.d_type == DrawType::After {
            self.draw.sy = d_range.sy;
        }

        if self.history.len_history() > 0 {
            let hist: &HistoryInfo = self.history.get_history_last();
            let ep = hist.evt_proc.clone();
            match ep.d_range.d_type {
                DrawType::Target | DrawType::After | DrawType::All | DrawType::None => {
                    if self.is_edit_evt(true) {
                        Log::ep_s("refresh refresh refresh refresh refresh");
                        for i in self.draw.sy..=self.draw.ey {
                            self.draw.char_vec[i] = self.buf.char_vec_line(i);
                        }
                    }
                }
                DrawType::Not => {}
            }
        }

        Log::ep("self.draw.sy", self.draw.sy);
        Log::ep("self.draw.ey", self.draw.ey);
        // Initial display line
        for i in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[i].len() == 0 {
                Log::ep_s("all refresh all refresh all refresh all refresh all refresh");
                self.draw.char_vec[i] = self.buf.char_vec_line(i);
            }
        }
    }
}
