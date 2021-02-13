use crate::{def::*, log::*, model::*, prompt::prompt::*};
use chrono::prelude::Local;
use crossterm::event::Event;

impl History {
    pub fn regist_edit(&mut self, evt: Event, ep: EvtProc) {
        Log::ep_s("　　　　　　　History.regist ");
        Log::ep("ep ", &ep);

        if evt == UNDO {
            if let Some(undo_ep) = self.pop_undo() {
                self.redo_vec.push(undo_ep);
            }
        } else if evt == REDO {
            if let Some(redo_ep) = self.pop_redo() {
                self.undo_vec.push(redo_ep);
            }
        // Normal
        } else {
            self.undo_vec.push(ep.clone());
        }
    }

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

    pub fn check_multi_click(&mut self, evt: &Event) -> usize {
        Log::ep_s("　　　　　　　regist_mouse_click ");

        let mut click_count = 1;

        if self.mouse_click_vec.len() > 2 {
            self.mouse_click_vec.pop_front();
        }
        let now = Local::now().naive_local();

        Log::ep("self.mouse_click_vec.len()", &self.mouse_click_vec.len());

        if self.mouse_click_vec.len() > 0 {
            if let Some((one_before, one_before_evt)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if evt != one_before_evt || (now - *one_before).num_milliseconds() > MULTI_CLICK_MILLISECONDS {
                    Log::ep_s("mouse_click_vec.clear() ");
                    self.mouse_click_vec.clear();
                }
            }
        }

        if self.mouse_click_vec.len() == 1 {
            if let Some((one_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 1) {
                if (now - *one_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS {
                    Log::ep_s("                                           double_click");
                    click_count = 2;
                }
            }
        } else if self.mouse_click_vec.len() == 2 {
            if let Some((two_before, _)) = self.mouse_click_vec.get(self.mouse_click_vec.len() - 2) {
                if (now - *two_before).num_milliseconds() <= MULTI_CLICK_MILLISECONDS * 2 {
                    Log::ep_s("                                           triple_click");
                    click_count = 3;
                }
            }
        }
        self.mouse_click_vec.push_back((now, *evt));

        return click_count;
    }
}
