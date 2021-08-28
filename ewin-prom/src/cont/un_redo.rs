use crate::{
    cont::promptcont::PromptCont,
    ewin_core::{_cfg::keys::*, log::Log, model::*},
};
impl PromptCont {
    pub fn undo(&mut self) {
        Log::debug_s("PromptCont.undo");

        if let Some(evt_proc) = self.history.get_undo_last() {
            if let Some(ep) = evt_proc.evt_proc {
                self.undo_init(&ep);
                self.undo_exec(&ep);
                self.undo_finalize(&ep);
            }
            if let Some(sp) = evt_proc.sel_proc {
                self.undo_init(&sp);
                self.undo_exec(&sp);
                self.undo_finalize(&sp);
            }
            if let Some(undo_ep) = self.history.pop_undo() {
                self.history.redo_vec.push(undo_ep);
            }
        }
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match proc.keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::Cut => self.set_evtproc(&proc.cur_s),
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else if proc.keycmd == KeyCmd::DeleteNextChar {
                    self.set_evtproc(&proc.cur_s);
                } else {
                    self.set_evtproc(&proc.cur_e);
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match proc.keycmd {
            KeyCmd::InsertStr(_) => {
                // Set paste target with sel
                self.sel = proc.sel;
                self.edit_proc(KeyCmd::DeleteNextChar);
            }
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Cut => self.edit_proc(KeyCmd::InsertStr(proc.str.clone())),
            _ => {}
        }
    }

    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match proc.keycmd {
            KeyCmd::DeleteNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(&proc.cur_s);
                }
            }
            KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc.cur_e);
                }
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) {
        Log::debug_s("PromptCont.redo");

        if let Some(evt_proc) = self.history.get_redo_last() {
            if let Some(sp) = evt_proc.sel_proc {
                self.redo_exec(sp);
            }
            if let Some(ep) = evt_proc.evt_proc {
                self.redo_exec(ep);
            }
            if let Some(redo_ep) = self.history.pop_redo() {
                self.history.undo_vec.push(redo_ep);
            }
        }
    }

    pub fn redo_exec(&mut self, proc: Proc) {
        self.set_evtproc(&proc.cur_s);
        match proc.keycmd {
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        match proc.keycmd {
            KeyCmd::DeleteNextChar => self.edit_proc(KeyCmd::DeleteNextChar),
            KeyCmd::DeletePrevChar => self.edit_proc(KeyCmd::DeletePrevChar),
            KeyCmd::Cut => self.edit_proc(KeyCmd::Cut),
            KeyCmd::InsertStr(_) => {
                self.sel.clear();
                self.edit_proc(KeyCmd::InsertStr(proc.str));
            }
            _ => {}
        }
    }
}
