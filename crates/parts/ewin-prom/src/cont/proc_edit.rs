use super::parts::input_area::*;
use crate::ewin_key::model::*;
use ewin_cfg::log::*;
use ewin_const::models::event::*;
use ewin_key::cur::*;
use ewin_key::key::cmd::*;

impl PromContInputArea {
    pub fn edit_proc(&mut self, cmd: Cmd) -> ActType {
        Log::debug("PromptCont.Cmd", &cmd);

        let is_selected_org = self.sel.is_selected_width();
        let mut ep_del = Proc::default();
        let mut evt_proc = EvtProc::default();

        // selected range delete
        if self.sel.is_selected_width() {
            ep_del = Proc { cmd: if cmd.cmd_type == CmdType::DelNextChar { Cmd::to_cmd(CmdType::DelNextChar) } else { Cmd::to_cmd(CmdType::DelPrevChar) }, ..Proc::default() };
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
        if !(is_selected_org && (cmd.cmd_type == CmdType::DelNextChar || cmd.cmd_type == CmdType::DelPrevChar)) {
            let mut ep = Proc { cmd: cmd.clone(), ..Proc::default() };

            ep.cur_s = self.cur;
            let act_type = match &cmd.cmd_type {
                CmdType::DelNextChar => self.delete(&mut ep),
                CmdType::DelPrevChar => self.backspace(&mut ep),
                CmdType::Cut => self.cut(ep_del.str),
                CmdType::InsertStr(str) if str.is_empty() => self.paste(&mut ep),
                CmdType::InsertStr(str) => {
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
            if cmd.cmd_type != CmdType::Cut {
                evt_proc.proc = Some(ep.clone());
            }
        }

        // Register edit history
        if self.base.cmd.cmd_type != CmdType::Undo && self.base.cmd.cmd_type != CmdType::Redo {
            self.history.undo_vec.push(evt_proc);
        }
        return ActType::Next;
    }
}
