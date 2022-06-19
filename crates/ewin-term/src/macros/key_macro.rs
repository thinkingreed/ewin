use crate::{ewin_com::model::*, model::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::_cfg::key::cmd::Cmd;
impl Tab {
    pub fn record_key_macro_start(&mut self) -> ActType {
        Log::debug_key("macro_record_start");
        if self.editor.state.key_macro.is_record {
            self.editor.state.key_macro.is_record = false;
            return ActType::Draw(DParts::Editor(E_DrawRange::All));
        } else {
            self.editor.state.key_macro.is_record = true;
            self.editor.key_vec = vec![];
            return ActType::Draw(DParts::StatusBar);
        }
    }

    pub fn exec_key_macro(term: &mut Terminal) -> ActType {
        if term.curt().editor.key_vec.is_empty() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_key_record_exec.to_string()));
        }
        Log::debug("key_record_vec", &term.curt().editor.key_vec);

        term.curt().editor.state.key_macro.is_exec = true;

        let macro_vec = term.curt().editor.key_vec.clone();
        for (i, mac) in macro_vec.iter().enumerate() {
            // term.keycmd = KeyCmd::Edit(mac.e_cmd.clone());
            term.curt().editor.cmd = Cmd::to_cmd(mac.cmd_type.clone());

            if i == macro_vec.len() - 1 {
                term.curt().editor.state.key_macro.is_exec_end = true;
            }
            EvtAct::exec_editor(term);
        }
        term.curt().editor.state.key_macro.is_exec = false;
        term.curt().editor.state.key_macro.is_exec_end = false;

        return ActType::Next;
    }
}
