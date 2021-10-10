use crate::{
    ewin_core::{_cfg::key::keycmd::*, log::Log, model::*},
    model::PromptCont,
};

impl PromptCont {
    pub fn edit_proc(&mut self, p_cmd: P_Cmd) {
        Log::debug("PromptCont.keycmd", &p_cmd);
        if p_cmd == P_Cmd::DelNextChar {
            if !self.sel.is_selected_width() && self.cur.x == self.buf.len() {
                return;
            }
        } else if p_cmd == P_Cmd::DelPrevChar && !self.sel.is_selected_width() && self.cur.x == 0 {
            return;
        }

        let is_selected_org = self.sel.is_selected_width();
        let mut ep_del = Proc::default();
        let mut evt_proc = EvtProc::default();

        // selected range delete
        if self.sel.is_selected_width() {
            ep_del = Proc { p_cmd: if p_cmd == P_Cmd::DelNextChar { P_Cmd::DelNextChar } else { P_Cmd::DelPrevChar }, ..Proc::default() };
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
        if !(is_selected_org && (p_cmd == P_Cmd::DelNextChar || p_cmd == P_Cmd::DelPrevChar)) {
            let mut ep = Proc { p_cmd: p_cmd.clone(), ..Proc::default() };

            ep.cur_s = self.cur;
            match &p_cmd {
                P_Cmd::InsertStr(str) => ep.str = str.clone(),
                _ => {}
            }
            match &p_cmd {
                P_Cmd::DelNextChar => self.delete(&mut ep),
                P_Cmd::DelPrevChar => self.backspace(&mut ep),
                P_Cmd::Cut => self.cut(ep_del.str),
                P_Cmd::InsertStr(str) if str.is_empty() => self.paste(&mut ep),
                P_Cmd::InsertStr(_) => self.insert_str(&mut ep),
                _ => {}
            }
            ep.cur_e = self.cur;
            if p_cmd != P_Cmd::Cut {
                evt_proc.evt_proc = Some(ep.clone());
            }
        }

        // Register edit history
        if self.p_cmd != P_Cmd::Undo && self.p_cmd != P_Cmd::Redo {
            self.history.undo_vec.push(evt_proc);
        }
    }
}
