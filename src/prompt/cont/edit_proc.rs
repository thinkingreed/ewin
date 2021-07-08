use crate::{_cfg::keys::KeyCmd, model::*, prompt::cont::promptcont::PromptCont};

impl PromptCont {
    pub fn edit_proc(&mut self, keycmd: KeyCmd) {
        if keycmd == KeyCmd::DeleteNextChar {
            if !self.sel.is_selected() && self.cur.x == self.buf.len() {
                return;
            }
        } else if keycmd == KeyCmd::DeletePrevChar {
            if !self.sel.is_selected() && self.cur.x == 0 {
                return;
            }
        }
        let is_selected_org = self.sel.is_selected();
        let mut ep_org = EvtProc::default();
        // selected range delete
        if self.sel.is_selected() {
            ep_org = EvtProc { keycmd: if keycmd == KeyCmd::DeleteNextChar { KeyCmd::DeleteNextChar } else { KeyCmd::DeletePrevChar }, ..EvtProc::default() };
            ep_org.cur_s = Cur { y: self.sel.sy, x: self.sel.sx, disp_x: self.sel.s_disp_x };
            ep_org.cur_e = self.cur;
            let sel = self.sel.get_range();
            ep_org.str = self.buf[sel.sx..sel.ex].iter().collect::<String>();
            ep_org.sel = self.sel;
            self.del_sel_range();
            self.sel.clear();
            self.history.regist_edit(self.keycmd.clone(), &ep_org);
        }

        // not selected Del, BS, Cut or InsertChar, Paste, Enter
        if !(is_selected_org && (keycmd == KeyCmd::DeleteNextChar || keycmd == KeyCmd::DeletePrevChar)) {
            let mut ep = EvtProc { keycmd: keycmd.clone(), ..EvtProc::default() };

            ep.cur_s = self.cur;
            match &keycmd {
                // KeyCmd::InsertChar(c) => ep.str = c.to_string(),
                KeyCmd::InsertStr(str) => ep.str = str.clone(),
                _ => {}
            }
            match &keycmd {
                KeyCmd::DeleteNextChar => self.delete(&mut ep),
                KeyCmd::DeletePrevChar => self.backspace(&mut ep),
                KeyCmd::CutSelect => self.cut(ep_org.str),
                //  KeyCmd::InsertChar(c) => self.insert_char(*c),
                KeyCmd::InsertStr(str) => {
                    if str.is_empty() {
                        self.paste(&mut ep);
                    } else {
                        self.insert_str(&mut ep);
                    }
                }
                _ => {}
            }
            if keycmd != KeyCmd::CutSelect {
                ep.cur_e = self.cur;
                self.history.regist_edit(self.keycmd.clone(), &ep);
            }
        }
    }
}
