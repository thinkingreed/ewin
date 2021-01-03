extern crate ropey;
use crate::model::*;
use std::cmp::min;
use termion::{clear, cursor};

impl Editor {
    pub fn draw_cache(&mut self, str_vec: &mut Vec<String>) {
        self.draw.clear();
        if self.draw.char_vec.len() == 0 {
            Log::ep_s("draw.char_vec.len() == 0 ");

            let mut vec: Vec<Vec<char>> = vec![];
            let len = self.buf.len_lines();
            // Log::ep("lenlenlenlen", len);

            for _ in 0..len {
                vec.push(vec![]);
            }
            self.draw.char_vec = vec;
        }

        self.draw.y_s = self.offset_y;
        self.draw.y_e = min(self.buf.len_lines(), self.offset_y + self.disp_row_num);

        let d_range = self.d_range.get_range();

        if d_range.d_type == DType::Not {
            return;
        } else if d_range.d_type == DType::None || d_range.d_type == DType::All {
            str_vec.push(clear::All.to_string());
            str_vec.push(cursor::Goto(1, 1).to_string());
        } else {
            self.draw.y_s = d_range.sy;
            if d_range.d_type == DType::Target {
                for i in d_range.sy - self.offset_y..=d_range.ey - self.offset_y {
                    str_vec.push(format!("{}{}", cursor::Goto(1, (i + 1) as u16), clear::CurrentLine));
                }
                str_vec.push(cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16).to_string());
                self.draw.y_e = d_range.ey + 1;
            } else if d_range.d_type == DType::After {
                str_vec.push(format!("{}{}", cursor::Goto(1, (d_range.sy + 1 - self.offset_y) as u16), clear::AfterCursor));
            }
        }

        for i in self.draw.y_s..self.draw.y_e {
            if self.draw.char_vec[i].len() == 0 {
                let vec = self.buf.char_vec(i);
                Log::ep("changed char_vec i ", i);
                self.draw.char_vec[i] = vec;
            } else {
                let proc: &EvtProc = &self.undo_vec[self.undo_vec.len() - 1];
                let y_s = proc.cur_s.y;
                let y_e = proc.cur_e.y;
                match proc.do_type {
                    DoType::Del => {}
                    DoType::BS => {}
                    DoType::Enter => {}
                    DoType::Cut => {}
                    DoType::InsertChar => {}
                    DoType::Paste => {}
                    _ => {}
                }
                Log::ep("y_s ", y_s);
                Log::ep("y_e ", y_e);
            }
        }
        //  eprintln!("draw.char_vec 222 {:?}", self.draw.char_vec.clone());
    }
}
