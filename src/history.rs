use crate::{def::*, model::*};
use crossterm::event::Event;

impl EditHistory {
    pub fn regist(&mut self, evt: Event, ep: EvtProc) {
        Log::ep_s("　　　　　　　History.regist ");
        Log::ep("ep ", &ep);
        // eprintln!("evt {:?}", evt);

        if evt == UNDO {
            Log::ep_s("UNDOUNDOUNDOUNDO");
            if let Some(undo_ep) = self.pop_undo() {
                self.redo_vec.push(undo_ep);
            }
            // eprintln!("self.redo_vec {:?}", self.redo_vec);

            self.history_vec.push(HistoryInfo { ope_type: Opetype::Undo, evt_proc: ep });
        } else if evt == REDO {
            Log::ep_s("REDOREDOREDOREDO");
            if let Some(redo_ep) = self.pop_redo() {
                self.undo_vec.push(redo_ep);
            }
            self.history_vec.push(HistoryInfo { ope_type: Opetype::Redo, evt_proc: ep });
        // Normal
        } else {
            Log::ep_s("NormalNormalNormalNormal");

            self.undo_vec.push(ep.clone());
            self.history_vec.push(HistoryInfo { ope_type: Opetype::Normal, evt_proc: ep });
        }
    }

    pub fn pop_redo(&mut self) -> Option<EvtProc> {
        self.redo_vec.pop()
    }

    pub fn pop_undo(&mut self) -> Option<EvtProc> {
        self.undo_vec.pop()
    }
    pub fn get_history_last(&self) -> &HistoryInfo {
        &self.history_vec[self.history_vec.len() - 1]
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

    pub fn len_history(&mut self) -> usize {
        self.history_vec.len()
    }
    pub fn len_redo(&mut self) -> usize {
        self.redo_vec.len()
    }
}
