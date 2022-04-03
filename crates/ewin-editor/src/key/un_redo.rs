use ewin_com::_cfg::lang::lang_cfg::Lang;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
    model::*,
};
impl Editor {
    pub fn undo(&mut self) -> ActType {
        Log::debug_key("undo");

        if self.history.len_undo() == 0 {
            return ActType::Render(RParts::MsgBar(Lang::get().no_undo_operation.to_string()));
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
        return ActType::Next;
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match &proc.e_cmd {
            E_Cmd::InsertStr(_) | E_Cmd::InsertRow | E_Cmd::Cut | E_Cmd::ReplaceExec(_, _, _) => self.cur = proc.cur_s,
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {
                self.cur = if proc.sel.is_selected() {
                    if proc.cur_s.x > proc.cur_e.x {
                        proc.cur_e
                    } else {
                        proc.cur_s
                    }
                } else if proc.e_cmd == E_Cmd::DelNextChar {
                    proc.cur_s
                } else {
                    proc.cur_e
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        let _ = match &proc.e_cmd {
            E_Cmd::InsertRow => self.edit_proc(E_Cmd::DelNextChar),
            E_Cmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    // Set paste target with sel
                    self.sel = proc.sel;
                    self.edit_proc(E_Cmd::DelNextChar)
                } else {
                    self.edit_proc(E_Cmd::DelBox(proc.box_sel_vec.clone()))
                }
            }
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar => self.edit_proc(if proc.box_sel_vec.is_empty() { E_Cmd::InsertStr(proc.str.clone()) } else { E_Cmd::InsertBox(proc.box_sel_vec.clone()) }),
            E_Cmd::ReplaceExec(search_str, replace_str, idx_set) => {
                let idx_set = self.get_idx_set(search_str, replace_str, idx_set);

                self.edit_proc(E_Cmd::ReplaceExec(replace_str.clone(), search_str.clone(), idx_set))
            }
            _ => todo!(),
        };
    }
    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match &proc.e_cmd {
            E_Cmd::DelNextChar => {
                self.cur = if proc.sel.is_selected() {
                    if proc.cur_s.x > proc.cur_e.x {
                        proc.cur_s
                    } else {
                        proc.cur_e
                    }
                } else {
                    proc.cur_s
                }
            }
            E_Cmd::DelPrevChar => {
                self.cur = if proc.sel.is_selected() {
                    proc.cur_e
                    // !proc.box_sel_vec.is_empty()
                } else {
                    proc.cur_s
                }
            }
            E_Cmd::ReplaceExec(_, _, _) => {
                // Return cursor position
                self.cur = proc.cur_s;
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) -> ActType {
        Log::debug_key("redo");
        if self.history.len_redo() == 0 {
            return ActType::Render(RParts::MsgBar(Lang::get().no_redo_operation.to_string()));
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
        self.cur = proc.cur_s;
        match &proc.e_cmd {
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        let _ = match &proc.e_cmd {
            E_Cmd::DelNextChar => self.edit_proc(E_Cmd::DelNextChar),
            E_Cmd::DelPrevChar => self.edit_proc(E_Cmd::DelPrevChar),
            E_Cmd::Cut => self.edit_proc(E_Cmd::Cut),
            E_Cmd::InsertRow => self.edit_proc(E_Cmd::InsertRow),
            E_Cmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(E_Cmd::InsertStr(proc.str.clone()))
                } else {
                    self.edit_proc(E_Cmd::InsertBox(proc.box_sel_redo_vec.clone()))
                }
            }
            E_Cmd::ReplaceExec(search_str, replace_str, idx_set) => self.edit_proc(E_Cmd::ReplaceExec(search_str.clone(), replace_str.clone(), idx_set.clone())),
            _ => todo!(),
        };
    }
}
