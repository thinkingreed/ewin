use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
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
        match &proc.e_cmd {
            E_Cmd::InsertStr(_) | E_Cmd::InsertLine | E_Cmd::Cut | E_Cmd::ReplaceExec(_, _, _) => self.set_evtproc(proc, &proc.cur_s),
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_e } else { &proc.cur_s });
                } else if proc.e_cmd == E_Cmd::DelNextChar {
                    self.set_evtproc(proc, &proc.cur_s);
                } else {
                    self.set_evtproc(proc, &proc.cur_e);
                }
            }
            _ => {}
        }
    }
    pub fn undo_exec(&mut self, proc: &Proc) {
        match &proc.e_cmd {
            E_Cmd::InsertLine => self.edit_proc(E_Cmd::DelNextChar),
            E_Cmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    // Set paste target with sel
                    self.sel = proc.sel;
                    self.edit_proc(E_Cmd::DelNextChar);
                } else {
                    self.edit_proc(E_Cmd::DelBox(proc.box_sel_vec.clone()));
                }
            }
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar => {
                self.edit_proc(if proc.box_sel_vec.is_empty() { E_Cmd::InsertStr(proc.str.clone()) } else { E_Cmd::InsertBox(proc.box_sel_vec.clone()) });
            }
            E_Cmd::ReplaceExec(is_regex, replace_str, search_map) => {
                let replace_map = self.get_replace_map(*is_regex, replace_str, search_map);

                if *is_regex {
                    for ((s, e), org_str) in replace_map {
                        let mut map = BTreeMap::new();
                        map.insert((s, e), "".to_string());
                        self.edit_proc(E_Cmd::ReplaceExec(*is_regex, org_str.clone(), map));
                    }
                } else {
                    let search_str = search_map.iter().min().unwrap().1;
                    self.edit_proc(E_Cmd::ReplaceExec(*is_regex, search_str.clone(), replace_map));
                }
            }
            _ => {}
        }
    }
    // last cursor posi set
    pub fn undo_finalize(&mut self, proc: &Proc) {
        match &proc.e_cmd {
            E_Cmd::DelNextChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(proc, if proc.cur_s.x > proc.cur_e.x { &proc.cur_s } else { &proc.cur_e });
                } else {
                    self.set_evtproc(proc, &proc.cur_s);
                }
            }
            E_Cmd::DelPrevChar => {
                if proc.sel.is_selected() {
                    self.set_evtproc(proc, &proc.cur_e);
                } else if !proc.box_sel_vec.is_empty() {
                    self.set_evtproc(proc, &proc.cur_s);
                }
            }
            E_Cmd::ReplaceExec(_, _, _) => {
                // Return cursor position
                self.set_evtproc(proc, &proc.cur_s);
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

        match &proc.e_cmd {
            E_Cmd::DelNextChar | E_Cmd::DelPrevChar | E_Cmd::Cut => self.sel = proc.sel,
            _ => {}
        }
        match &proc.e_cmd {
            E_Cmd::DelNextChar => self.edit_proc(E_Cmd::DelNextChar),
            E_Cmd::DelPrevChar => self.edit_proc(E_Cmd::DelPrevChar),
            E_Cmd::Cut => self.edit_proc(E_Cmd::Cut),
            E_Cmd::InsertLine => self.edit_proc(E_Cmd::InsertLine),
            E_Cmd::InsertStr(_) => {
                if proc.box_sel_vec.is_empty() {
                    self.edit_proc(E_Cmd::InsertStr(proc.str.clone()));
                } else {
                    self.edit_proc(E_Cmd::InsertBox(proc.box_sel_redo_vec.clone()));
                }
            }
            E_Cmd::ReplaceExec(is_regex, replace_str, search_map) => self.edit_proc(E_Cmd::ReplaceExec(*is_regex, replace_str.clone(), search_map.clone())),
            _ => {}
        }
    }
}
