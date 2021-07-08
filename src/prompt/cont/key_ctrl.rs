use crate::{
    _cfg::keys::KeyCmd,
    clipboard::{get_clipboard, set_clipboard},
    log::Log,
    model::{Cur, EvtProc},
    prompt::cont::promptcont::*,
};

impl PromptCont {
    pub fn paste(&mut self, ep: &mut EvtProc) {
        // for Not Undo
        ep.str = get_clipboard().unwrap_or("".to_string());

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
        set_clipboard(&cut_str);
    }

    pub fn undo(&mut self) {
        Log::debug_s("PromptCont.undo");

        if let Some(ep) = self.history.get_undo_last() {
            Log::debug("EvtProc", &ep);
            // initial cursor posi set
            match ep.keycmd {
                KeyCmd::InsertStr(_) | KeyCmd::CutSelect => self.set_evtproc(&ep.cur_s),
                KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(if ep.cur_s.x > ep.cur_e.x { &ep.cur_e } else { &ep.cur_s });
                    } else {
                        if ep.keycmd == KeyCmd::DeleteNextChar {
                            self.set_evtproc(&ep.cur_s);
                        } else {
                            self.set_evtproc(&ep.cur_e);
                        }
                    }
                }
                _ => {}
            }
            // exec
            match ep.keycmd {
                // KeyCmd::InsertChar(_) => self.edit_proc(KeyCmd::DeleteNextChar),
                KeyCmd::InsertStr(_) => {
                    // Set paste target with sel
                    self.sel = ep.sel;
                    self.edit_proc(KeyCmd::DeleteNextChar);
                }
                KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::CutSelect => self.edit_proc(KeyCmd::InsertStr(ep.str)),
                _ => {}
            }
            // last cursor posi set
            match ep.keycmd {
                KeyCmd::DeleteNextChar => {
                    if ep.sel.is_selected() {
                        self.set_evtproc(if ep.cur_s.x > ep.cur_e.x { &ep.cur_s } else { &ep.cur_e });
                    } else {
                        self.set_evtproc(&ep.cur_s);
                    }
                }
                KeyCmd::DeletePrevChar => {
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
            match ep.keycmd {
                KeyCmd::DeleteNextChar => self.edit_proc(KeyCmd::DeleteNextChar),
                KeyCmd::DeletePrevChar => self.edit_proc(KeyCmd::DeletePrevChar),
                KeyCmd::CutSelect => self.edit_proc(KeyCmd::CutSelect),
                //  KeyCmd::InsertChar(c) => self.edit_proc(KeyComd::InsertChar(c)),
                KeyCmd::InsertStr(str) => {
                    self.sel.clear();
                    self.edit_proc(KeyCmd::InsertStr(str));
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
