use super::parts::input_area::*;
use crate::ewin_com::model::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::_cfg::key::cmd::*;

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
        match proc.cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::Cut => {
                self.set_evtproc(&proc.cur_s);
            }
            CmdType::DelNextChar | CmdType::DelPrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else if proc.cmd.cmd_type == CmdType::DelNextChar {
                    self.set_evtproc(&proc.cur_s);
                } else {
                    self.set_evtproc(&proc.cur_e);
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match proc.cmd.cmd_type {
            CmdType::InsertStr(_) => {
                // Set paste target with sel
                self.sel = proc.sel;
                self.edit_proc(Cmd::to_cmd(CmdType::DelNextChar));
            }
            CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut => {
                self.edit_proc(Cmd::to_cmd(CmdType::InsertStr(proc.str.clone())));
            }
            _ => {}
        }
    }

    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match proc.cmd.cmd_type {
            CmdType::DelNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(&proc.cur_s);
                }
            }
            CmdType::DelPrevChar => {
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
        match proc.cmd.cmd_type {
            CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut => self.sel = proc.sel,
            _ => {}
        }
        match proc.cmd.cmd_type {
            CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut => {
                self.edit_proc(proc.cmd);
            }
            CmdType::InsertStr(_) => {
                self.sel.clear();
                self.edit_proc(Cmd::to_cmd(CmdType::InsertStr(proc.str)));
            }
            _ => {}
        }
    }
}
