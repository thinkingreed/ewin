use super::parts::input_area::*;
use crate::ewin_com::{_cfg::key::keycmd::*, model::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};

impl PromContInputArea {
    pub fn undo(&mut self) -> ActType {
        Log::debug_s("PromptCont.undo");
        if self.history.len_undo() == 0 {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_undo_operation.to_string()));
        }

        if let Some(evt_proc) = self.history.get_undo_last() {
            if let Some(ep) = evt_proc.proc {
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
        return ActType::Next;
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match proc.p_cmd {
            P_Cmd::InsertStr(_) | P_Cmd::Cut => {
                self.set_evtproc(&proc.cur_s);
            }
            P_Cmd::DelNextChar | P_Cmd::DelPrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else if proc.p_cmd == P_Cmd::DelNextChar {
                    self.set_evtproc(&proc.cur_s);
                } else {
                    self.set_evtproc(&proc.cur_e);
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match proc.p_cmd {
            P_Cmd::InsertStr(_) => {
                // Set paste target with sel
                self.sel = proc.sel;
                self.edit_proc(P_Cmd::DelNextChar);
            }
            P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut => {
                self.edit_proc(P_Cmd::InsertStr(proc.str.clone()));
            }
            _ => {}
        }
    }

    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match proc.p_cmd {
            P_Cmd::DelNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(&proc.cur_s);
                }
            }
            P_Cmd::DelPrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc.cur_e);
                }
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) -> ActType {
        Log::debug_s("PromptCont.redo");
        if self.history.len_redo() == 0 {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_redo_operation.to_string()));
        }

        if let Some(evt_proc) = self.history.get_redo_last() {
            if let Some(sp) = evt_proc.sel_proc {
                self.redo_exec(sp);
            }
            if let Some(ep) = evt_proc.proc {
                self.redo_exec(ep);
            }
            if let Some(redo_ep) = self.history.pop_redo() {
                self.history.undo_vec.push(redo_ep);
            }
        }
        return ActType::Next;
    }

    pub fn redo_exec(&mut self, proc: Proc) {
        self.set_evtproc(&proc.cur_s);
        match proc.p_cmd {
            P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        match proc.p_cmd {
            P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut => {
                self.edit_proc(proc.p_cmd);
            }
            P_Cmd::InsertStr(_) => {
                self.sel.clear();
                self.edit_proc(P_Cmd::InsertStr(proc.str));
            }
            _ => {}
        }
    }
}
