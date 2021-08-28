use crate::{
    cont::promptcont::PromptCont,
    ewin_core::{_cfg::keys::*, log::Log, model::*},
};

impl PromptCont {
    pub fn edit_proc(&mut self, keycmd: KeyCmd) {
        Log::debug("PromptCont.keycmd", &keycmd);
        if keycmd == KeyCmd::DeleteNextChar {
            if !self.sel.is_selected_width() && self.cur.x == self.buf.len() {
                return;
            }
        } else if keycmd == KeyCmd::DeletePrevChar && !self.sel.is_selected_width() && self.cur.x == 0 {
            return;
        }

        let is_selected_org = self.sel.is_selected_width();
        let mut ep_del = Proc::default();
        let mut evt_proc = EvtProc::default();

        // selected range delete
        if self.sel.is_selected_width() {
            ep_del = Proc { keycmd: if keycmd == KeyCmd::DeleteNextChar { KeyCmd::DeleteNextChar } else { KeyCmd::DeletePrevChar }, ..Proc::default() };
            ep_del.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.s_disp_x };
            ep_del.cur_e = self.cur;
            let sel = self.sel.get_range();

            Log::debug("self.cur 111", &self.cur);

            ep_del.str = self.buf[sel.sx..sel.ex].iter().collect::<String>();
            ep_del.sel = self.sel;
            self.del_sel_range();
            Log::debug("self.cur 222", &self.cur);
            self.sel.clear();
            evt_proc.sel_proc = Some(ep_del.clone());
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if !(is_selected_org && (keycmd == KeyCmd::DeleteNextChar || keycmd == KeyCmd::DeletePrevChar)) {
            let mut ep = Proc { keycmd: keycmd.clone(), ..Proc::default() };

            ep.cur_s = self.cur;
            match &keycmd {
                KeyCmd::InsertStr(str) => ep.str = str.clone(),
                _ => {}
            }
            match &keycmd {
                KeyCmd::DeleteNextChar => self.delete(&mut ep),
                KeyCmd::DeletePrevChar => self.backspace(&mut ep),
                KeyCmd::Cut => self.cut(ep_del.str),
                KeyCmd::InsertStr(str) => {
                    if str.is_empty() {
                        self.paste(&mut ep);
                    } else {
                        self.insert_str(&mut ep);
                    }
                }
                _ => {}
            }
            ep.cur_e = self.cur;
            if keycmd != KeyCmd::Cut {
                evt_proc.evt_proc = Some(ep.clone());
            }
        }

        // Register edit history
        if self.keycmd != KeyCmd::Undo && self.keycmd != KeyCmd::Redo {
            self.history.undo_vec.push(evt_proc);
        }
    }
}
