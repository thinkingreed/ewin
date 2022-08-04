use crate::{ewin_key::model::*, model::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::model::*;
use ewin_key::key::cmd::*;

impl Editor {
    pub fn undo(&mut self) -> ActType {
        Log::debug_key("undo");

        if self.history.len_undo() == 0 {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_undo_operation.to_string()));
        }

        if let Some(evt_proc) = self.history.get_undo_last() {
            Log::debug("evt_proc", &evt_proc);

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
            self.scroll();
            self.scroll_horizontal();

            if let Some(undo_ep) = self.history.pop_undo() {
                self.history.redo_vec.push(undo_ep);
            }
        }
        Log::debug("self.history.redo_vec", &self.history.redo_vec);

        return ActType::Next;
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match &proc.cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::InsertRow | CmdType::Cut | CmdType::ReplaceExec(_, _, _) => self.win_mgr.curt().cur = proc.cur_s,
            CmdType::DelNextChar | CmdType::DelPrevChar => {
                self.win_mgr.curt().cur = if proc.sel.is_selected() {
                    if proc.cur_s.x > proc.cur_e.x {
                        proc.cur_e
                    } else {
                        proc.cur_s
                    }
                } else if proc.cmd.cmd_type == CmdType::DelNextChar {
                    proc.cur_s
                } else {
                    proc.cur_e
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match &proc.cmd.cmd_type {
            CmdType::InsertRow => self.edit_proc_cmd_type(CmdType::DelNextChar),
            CmdType::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    // Set paste target with sel
                    self.win_mgr.curt().sel = proc.sel;
                    self.edit_proc_cmd_type(CmdType::DelNextChar)
                } else {
                    self.edit_proc_cmd_type(CmdType::DelBox(proc.box_sel_vec.clone()))
                }
            }
            CmdType::DelNextChar | CmdType::DelPrevChar => self.edit_proc_cmd_type(if proc.box_sel_vec.is_empty() { CmdType::InsertStr(proc.str.clone()) } else { CmdType::InsertBox(proc.box_sel_vec.clone()) }),
            CmdType::ReplaceExec(search_str, replace_str, idx_set) => {
                let idx_set = self.get_idx_set(search_str, replace_str, idx_set);

                self.edit_proc_cmd_type(CmdType::ReplaceExec(replace_str.clone(), search_str.clone(), idx_set))
            }
            _ => todo!(),
        };
    }
    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match &proc.cmd.cmd_type {
            CmdType::DelNextChar => {
                self.win_mgr.curt().cur = if proc.sel.is_selected() {
                    if proc.cur_s.x > proc.cur_e.x {
                        proc.cur_s
                    } else {
                        proc.cur_e
                    }
                } else {
                    proc.cur_s
                }
            }
            CmdType::DelPrevChar => {
                self.win_mgr.curt().cur = if proc.sel.is_selected() {
                    proc.cur_e
                    // !proc.box_sel_vec.is_empty()
                } else {
                    proc.cur_s
                }
            }
            CmdType::ReplaceExec(_, _, _) => {
                // Return cursor position
                self.win_mgr.curt().cur = proc.cur_s;
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) -> ActType {
        Log::debug_key("redo");
        if self.history.len_redo() == 0 {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_redo_operation.to_string()));
        }

        if let Some(evt_proc) = self.history.get_redo_last() {
            Log::debug("evt_proc", &evt_proc);
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
        self.win_mgr.curt().cur = proc.cur_s;
        match &proc.cmd.cmd_type {
            CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut => self.win_mgr.curt().sel = proc.sel,
            _ => {}
        }
        let _ = match &proc.cmd.cmd_type {
            CmdType::DelNextChar => self.edit_proc(proc.cmd),
            CmdType::DelPrevChar => self.edit_proc(proc.cmd),
            CmdType::Cut => self.edit_proc(proc.cmd),
            CmdType::InsertRow => self.edit_proc(proc.cmd),
            CmdType::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(proc.cmd)
                } else {
                    self.edit_proc_cmd_type(CmdType::InsertBox(proc.box_sel_redo_vec.clone()))
                }
            }
            CmdType::ReplaceExec(search_str, replace_str, idx_set) => self.edit_proc_cmd_type(CmdType::ReplaceExec(search_str.clone(), replace_str.clone(), idx_set.clone())),
            _ => todo!(),
        };
    }
}
