use crate::{_cfg::key::keys::*, def::*, log::*, model::*, util::*};
use chrono::prelude::Local;

impl History {
    pub fn pop_redo(&mut self) -> Option<EvtProc> {
        self.redo_vec.pop()
    }

    pub fn pop_undo(&mut self) -> Option<EvtProc> {
        self.undo_vec.pop()
    }

    pub fn get_undo_last(&mut self) -> Option<EvtProc> {
        if self.undo_vec.is_empty() {
            None
        } else {
            Some(self.undo_vec[self.undo_vec.len() - 1].clone())
        }
    }
    pub fn len_undo(&self) -> usize {
        self.undo_vec.len()
    }

    pub fn get_redo_last(&mut self) -> Option<EvtProc> {
        if self.redo_vec.is_empty() {
            None
        } else {
            Some(self.redo_vec[self.redo_vec.len() - 1].clone())
        }
    }

    pub fn clear_undo_vec(&mut self) {
        self.undo_vec.clear();
    }

    pub fn clear_redo_vec(&mut self) {
        self.redo_vec.clear();
    }

    pub fn len_redo(&self) -> usize {
        self.redo_vec.len()
    }

    pub fn count_multi_click(&mut self, keys: &Keys) -> usize {
        Log::debug_key("History.count_multi_click");
        Log::debug("keys", &keys);

        let mut click_count = 1;

        Log::debug("mouse_click_vec", &self.mouse_click_vec);

        if self.mouse_click_vec.len() > 2 {
            self.mouse_click_vec.pop_front();
        }

        let now = Local::now().naive_local();

        if !self.mouse_click_vec.is_empty() {
            if let Some((one_before, one_before_keys)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if keys != one_before_keys || (now - *one_before).num_milliseconds() > MULTI_CLICK_MILLISECONDS {
                    self.mouse_click_vec.clear();
                }
            }
        }

        if self.mouse_click_vec.len() == 1 {
            if let Some((one_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if (now - *one_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS {
                    Log::debug_s("double_click");
                    click_count = 2;
                }
            }
        } else if self.mouse_click_vec.len() == 2 {
            if let Some((two_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 2) {
                if (now - *two_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS * 2 {
                    Log::debug_s("triple_click");
                    click_count = 3;
                }
            }
        }

        self.mouse_click_vec.push_back((now, *keys));
        click_count
    }

    pub fn set_sel_multi_click(&mut self, mouse_proc: MouseProc, sel: &mut SelRange, cur: &Cur, row: &[char], keys: &Keys) {
        match mouse_proc {
            MouseProc::DownLeft | MouseProc::DownLeftBox => {
                match self.count_multi_click(keys) {
                    1 => {
                        sel.clear();
                        sel.set_s(cur.y, cur.x, cur.disp_x);
                        sel.set_e(cur.y, cur.x, cur.disp_x);
                    }
                    // Delimiter unit
                    2 => {
                        let (sx, ex) = get_delim_x(row, cur.x);
                        sel.set_s(cur.y, sx, get_row_width(&row[..sx], 0, false).1);
                        sel.set_e(cur.y, ex, get_row_width(&row[..ex], 0, false).1);
                    }
                    // One row
                    3 => {
                        sel.set_s(cur.y, 0, 0);
                        let (cur_x, width) = get_row_width(row, 0, true);
                        sel.set_e(cur.y, cur_x, width);
                    }
                    _ => {}
                }
            }
            _ => sel.set_e(cur.y, cur.x, cur.disp_x),
        }
    }
}