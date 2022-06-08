use super::parts::input_area::*;
use crate::ewin_com::{_cfg::key::keycmd::*, model::*};
use ewin_cfg::log::*;

impl PromContInputArea {
    pub fn edit_proc(&mut self, p_cmd: P_Cmd) -> ActType {
        Log::debug("PromptCont.keycmd", &p_cmd);

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
            let act_type = match &p_cmd {
                P_Cmd::DelNextChar => self.delete(&mut ep),
                P_Cmd::DelPrevChar => self.backspace(&mut ep),
                P_Cmd::Cut => self.cut(ep_del.str),
                P_Cmd::InsertStr(str) if str.is_empty() => self.paste(&mut ep),
                P_Cmd::InsertStr(str) => {
                    ep.str = str.clone();
                    self.insert_str(&mut ep);
                    ActType::Next
                }
                _ => ActType::Next,
            };

            if act_type != ActType::Next {
                return act_type;
            }
            ep.cur_e = self.cur;
            if p_cmd != P_Cmd::Cut {
                evt_proc.proc = Some(ep.clone());
            }
        }

        // Register edit history
        if self.base.p_cmd != P_Cmd::Undo && self.base.p_cmd != P_Cmd::Redo {
            self.history.undo_vec.push(evt_proc);
        }
        return ActType::Next;
    }
}
