extern crate ropey;
use crate::model::*;
use std::cmp::min;

impl Editor {
    pub fn draw_cache(&mut self) {
        Log::ep_s("　　　　　　　draw_cache");
        // char_vec initialize
        if self.draw.char_vec.len() == 0 {
            let diff: isize = self.buf.len_lines() as isize - self.draw.char_vec.len() as isize;
            if diff > 0 {
                self.draw.char_vec.resize_with(self.buf.len_lines() as usize, || vec![]);
            }
        }
        self.draw.sy = self.offset_y;
        self.draw.ey = min(self.buf.len_lines() - 1, self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();
        if d_range.d_type == DrawType::Target || d_range.d_type == DrawType::After {
            self.draw.sy = d_range.sy;
        }

        if self.history.len_history() > 0 {
            let hist: &HistoryInfo = self.history.get_history_last();
            eprintln!("hist {:?}", hist);
            let ep = hist.evt_proc.clone();
            match ep.evt_type {
                EvtType::Del | EvtType::BS | EvtType::Cut => {
                    if hist.ope_type == Opetype::Undo {
                        self.add_line_char_vec(ep);
                    } else {
                        self.del_line_char_vec(ep);
                    }
                }
                EvtType::InsertChar | EvtType::Enter | EvtType::Paste => {
                    if hist.ope_type == Opetype::Undo {
                        self.del_line_char_vec(ep);
                    } else {
                        self.add_line_char_vec(ep);
                    }
                }
                _ => {}
            }
        }
        // for change offset_y
        for i in self.draw.sy..=self.draw.ey {
            if self.draw.char_vec[i].len() == 0 {
                self.draw.char_vec[i] = self.buf.char_vec_line(i);
            }
        }
        // eprintln!("self.draw.char_vec {:?}", self.draw.char_vec);
    }

    pub fn add_line_char_vec(&mut self, ep: EvtProc) {
        let add_line_num = self.buf.len_lines() - self.draw.char_vec.len();

        // beginning of the line in BS
        let mut sy = ep.cur_s.y;
        if ep.evt_type == EvtType::BS && ep.cur_s.y > ep.cur_e.y {
            sy = ep.cur_e.y;
        }
        for (i, y) in (0_usize..).zip(sy..=sy + add_line_num) {
            if i < add_line_num {
                self.draw.char_vec.insert(y, vec![]);
            }
            self.draw.char_vec[y] = self.buf.char_vec_line(y);
        }
    }

    pub fn del_line_char_vec(&mut self, ep: EvtProc) {
        let mut sy = ep.cur_s.y;
        // beginning of the line in BS
        if ep.evt_type == EvtType::BS && ep.cur_s.y > ep.cur_e.y {
            sy = ep.cur_e.y;
        }
        let del_line_num = self.draw.char_vec.len() - self.buf.len_lines();
        for _ in 0..del_line_num {
            self.draw.char_vec.remove(sy);
        }
        self.draw.char_vec[sy] = self.buf.char_vec_line(sy);
    }
}
