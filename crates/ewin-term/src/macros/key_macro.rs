use ewin_com::{
    _cfg::key::keycmd::KeyCmd,
    model::{ActType, RParts},
};

use crate::{
    ewin_com::{_cfg::lang::lang_cfg::*, log::*},
    model::*,
    tab::*,
};

impl Tab {
    pub fn record_key_macro_start(&mut self) -> ActType {
        Log::debug_key("macro_record_start");
        if self.editor.state.key_macro.is_record {
            self.editor.state.key_macro.is_record = false;
            self.mbar.clear_key_record();
            return ActType::Render(RParts::Editor);
        } else {
            self.editor.state.key_macro.is_record = true;
            self.editor.key_vec = vec![];
            return ActType::Render(RParts::MsgBar(Lang::get().key_recording.to_string()));
        }
    }

    pub fn exec_key_macro(term: &mut Terminal) -> ActType {
        if term.curt().editor.key_vec.is_empty() {
            return ActType::Render(RParts::MsgBar(Lang::get().no_key_record_exec.to_string()));
        }
        Log::debug("key_record_vec", &term.curt().editor.key_vec);

        term.curt().editor.state.key_macro.is_exec = true;

        let macro_vec = term.curt().editor.key_vec.clone();
        for (i, mac) in macro_vec.iter().enumerate() {
            term.keycmd = KeyCmd::Edit(mac.e_cmd.clone());
            term.curt().editor.e_cmd = mac.e_cmd.clone();

            if i == macro_vec.len() - 1 {
                term.curt().editor.state.key_macro.is_exec_end = true;
            }
            EvtAct::ctrl_editor(term);
        }
        term.curt().editor.state.key_macro.is_exec = false;
        term.curt().editor.state.key_macro.is_exec_end = false;

        return ActType::Next;
    }
}
