use crate::{_cfg::keys::KeyCmd, def::*, log::*, model::*};
use chrono::prelude::Local;

impl History {
    pub fn pop_redo(&mut self) -> Option<EvtProc> {
        self.redo_vec.pop()
    }

    pub fn pop_undo(&mut self) -> Option<EvtProc> {
        self.undo_vec.pop()
    }

    pub fn get_undo_last(&mut self) -> Option<EvtProc> {
        if self.undo_vec.len() == 0 {
            None
        } else {
            Some(self.undo_vec[self.undo_vec.len() - 1].clone())
        }
    }
    pub fn len_undo(&mut self) -> usize {
        self.undo_vec.len()
    }

    pub fn get_redo_last(&mut self) -> Option<EvtProc> {
        if self.redo_vec.len() == 0 {
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

    pub fn len_redo(&mut self) -> usize {
        self.redo_vec.len()
    }

    pub fn count_multi_click(&mut self, keycmd: &KeyCmd) -> usize {
        let mut click_count = 1;

        Log::debug("mouse_click_vec", &self.mouse_click_vec);

        if self.mouse_click_vec.len() > 2 {
            self.mouse_click_vec.pop_front();
        }

        let now = Local::now().naive_local();

        if self.mouse_click_vec.len() > 0 {
            if let Some((one_before, one_before_keycmd)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if keycmd != one_before_keycmd || (now - *one_before).num_milliseconds() > MULTI_CLICK_MILLISECONDS {
                    self.mouse_click_vec.clear();
                }
            }
        }

        if self.mouse_click_vec.len() == 1 {
            if let Some((one_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if (now - *one_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS {
                    Log::debug_key("                             double_click");
                    click_count = 2;
                }
            }
        } else if self.mouse_click_vec.len() == 2 {
            if let Some((two_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 2) {
                if (now - *two_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS * 2 {
                    Log::debug_key("                             triple_click");
                    click_count = 3;
                }
            }
        }
        self.mouse_click_vec.push_back((now, keycmd.clone()));

        return click_count;
    }
}
