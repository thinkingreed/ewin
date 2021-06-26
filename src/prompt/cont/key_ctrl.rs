use crate::{
    _cfg::keys::KeyCmd,
    clipboard::{get_clipboard, set_clipboard},
    log::Log,
    model::{Cur, EvtProc, EvtType},
    prompt::cont::promptcont::*,
};

impl PromptCont {
    pub fn paste(&mut self, ep: &mut EvtProc) {
        // for Not Undo
        if self.keycmd == KeyCmd::Paste {
            ep.str = get_clipboard().unwrap_or("".to_string());
        }
        let chars: Vec<char> = ep.str.chars().collect();
        ep.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
        for c in chars {
            self.buf.insert(self.cur.x, c);
            self.cur_right();
        }
        ep.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
    }
    pub fn cut(&mut self, cut_str: String) {
        Log::debug_key("cut");
        // self.sel = ep.sel.clone();
        set_clipboard(cut_str.clone());
    }

    pub fn undo(&mut self) {
        Log::debug_s("PromptCont.undo");

        if let Some(ep) = self.history.get_undo_last() {
            Log::debug("EvtProc", &ep);
            // initial cursor posi set
            match ep.evt_type {
                EvtType::Cut | EvtType::InsertChar | EvtType::Paste => self.set_evtproc(&ep.cur_s),
                EvtType::Del | EvtType::BS => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(if ep.cur_s.x > ep.cur_e.x { &ep.cur_e } else { &ep.cur_s });
                    } else {
                        if ep.evt_type == EvtType::Del {
                            self.set_evtproc(&ep.cur_s);
                        } else {
                            self.set_evtproc(&ep.cur_e);
                        }
                    }
                }
                _ => {}
            }
            // exec
            match ep.evt_type {
                EvtType::InsertChar => self.exec_edit_proc(EvtType::Del, ""),
                EvtType::Paste => {
                    // Set paste target with sel
                    self.sel = ep.sel;
                    self.exec_edit_proc(EvtType::Del, "");
                }
                EvtType::Del | EvtType::BS | EvtType::Cut => self.exec_edit_proc(EvtType::Paste, &ep.str),
                _ => {}
            }

            match ep.evt_type {
                EvtType::Del => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(if ep.cur_s.x > ep.cur_e.x { &ep.cur_s } else { &ep.cur_e });
                    } else {
                        self.set_evtproc(&ep.cur_s);
                    }
                }
                EvtType::BS => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(&ep.cur_e);
                    }
                }
                _ => {}
            }
        }
    }
    pub fn redo(&mut self) {
        Log::debug_s("PromptCont.redo");

        if let Some(ep) = self.history.get_redo_last() {
            Log::debug("EvtProc", &ep);
            self.set_evtproc(&ep.cur_s);
            self.sel = ep.sel;
            match ep.evt_type {
                EvtType::Del => self.exec_edit_proc(EvtType::Del, ""),
                EvtType::BS => self.exec_edit_proc(EvtType::BS, ""),
                EvtType::Cut => self.exec_edit_proc(EvtType::Cut, ""),
                EvtType::InsertChar => self.exec_edit_proc(EvtType::InsertChar, &ep.str),
                EvtType::Paste => {
                    self.sel.clear();
                    self.exec_edit_proc(EvtType::Paste, &ep.str);
                }
                _ => {}
            }
        }
    }

    pub fn set_evtproc(&mut self, cur: &Cur) {
        self.cur.x = cur.x;
        self.cur.disp_x = cur.disp_x;
    }
}
