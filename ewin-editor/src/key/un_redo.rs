use crate::{
    ewin_core::{_cfg::keys::*, log::*, model::*},
    model::*,
};
use std::collections::BTreeMap;

impl Editor {
    pub fn undo(&mut self) {
        Log::debug_key("undo");

        if let Some(evt_proc) = self.history.get_undo_last() {
            Log::debug("evt_proc", &evt_proc);

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
            self.scroll();
            self.scroll_horizontal();

            if let Some(undo_ep) = self.history.pop_undo() {
                self.history.redo_vec.push(undo_ep);
            }
        }
    }
    // initial cursor posi set
    pub fn undo_init(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::InsertLine | KeyCmd::Cut | KeyCmd::ReplaceExec(_, _, _) => self.set_evtproc(&proc, &proc.cur_s),
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else if proc.keycmd == KeyCmd::DeleteNextChar {
                    self.set_evtproc(&proc, &proc.cur_s);
                } else {
                    self.set_evtproc(&proc, &proc.cur_e);
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::InsertLine => self.edit_proc(KeyCmd::DeleteNextChar),
            KeyCmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    // Set paste target with sel
                    self.sel = proc.sel;
                    self.edit_proc(KeyCmd::DeleteNextChar);
                } else {
                    self.edit_proc(KeyCmd::DelBox(proc.box_sel_vec.clone()));
                }
            }
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(KeyCmd::InsertStr(proc.str.clone()));
                } else {
                    self.edit_proc(KeyCmd::InsertBox(proc.box_sel_vec.clone()));
                }
            }
            KeyCmd::ReplaceExec(is_regex, replace_str, search_map) => {
                let replace_map = self.get_replace_map(*is_regex, replace_str, &search_map);

                if *is_regex {
                    for ((s, e), org_str) in replace_map {
                        let mut map = BTreeMap::new();
                        map.insert((s, e), "".to_string());
                        self.edit_proc(KeyCmd::ReplaceExec(*is_regex, org_str.clone(), map));
                    }
                } else {
                    let search_str = search_map.iter().min().unwrap().1;
                    self.edit_proc(KeyCmd::ReplaceExec(*is_regex, search_str.clone(), replace_map.clone()));
                }
            }

            _ => {}
        }
    }
    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match &proc.keycmd {
            KeyCmd::DeleteNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(&proc, &proc.cur_s);
                }
            }
            KeyCmd::DeletePrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(&proc, &proc.cur_e);
                } else if !proc.box_sel_vec.is_empty() {
                    self.set_evtproc(&proc, &proc.cur_s);
                }
            }
            KeyCmd::ReplaceExec(_, _, _) => {
                // Return cursor position
                self.set_evtproc(&proc, &proc.cur_s);
            }
            _ => {}
        }
    }

    pub fn redo(&mut self) {
        Log::debug_key("　redo");

        if let Some(evt_proc) = self.history.get_redo_last() {
            Log::debug("evt_proc", &evt_proc);
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
        self.set_evtproc(&proc, &proc.cur_s);

        match &proc.keycmd {
            KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        match &proc.keycmd {
            KeyCmd::DeleteNextChar => self.edit_proc(KeyCmd::DeleteNextChar),
            KeyCmd::DeletePrevChar => self.edit_proc(KeyCmd::DeletePrevChar),
            KeyCmd::Cut => self.edit_proc(KeyCmd::Cut),
            KeyCmd::InsertLine => self.edit_proc(KeyCmd::InsertLine),
            KeyCmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(KeyCmd::InsertStr(proc.str.clone()));
                } else {
                    self.edit_proc(KeyCmd::InsertBox(proc.box_sel_redo_vec.clone()));
                }
            }
            KeyCmd::ReplaceExec(is_regex, replace_str, search_map) => self.edit_proc(KeyCmd::ReplaceExec(*is_regex, replace_str.clone(), search_map.clone())),
            _ => {}
        }
    }
}
