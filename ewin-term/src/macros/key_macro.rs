use ewin_core::model::{ActType, DParts};

use crate::{
    ewin_core::{global::*, log::*},
    model::*,
    tab::*,
    terminal::*,
};

impl Tab {
    pub fn record_key_macro_start(&mut self) -> ActType {
        Log::debug_key("macro_record_start");
        if self.editor.state.key_macro.is_record {
            self.editor.state.key_macro.is_record = false;
            self.mbar.clear_key_record();
            return ActType::Draw(DParts::Editor);
        } else {
            self.editor.state.key_macro.is_record = true;
            self.editor.key_vec = vec![];
            return ActType::Draw(DParts::MsgBar(LANG.key_recording.to_string()));
        }
    }

    pub fn exec_key_macro(term: &mut Terminal) {
        Log::debug("key_record_vec", &term.curt().editor.key_vec);

        term.curt().editor.state.key_macro.is_exec = true;

        let macro_vec = term.curt().editor.key_vec.clone();
        for (i, mac) in macro_vec.iter().enumerate() {
            term.keys = mac.keys;
            if i == macro_vec.len() - 1 {
                term.curt().editor.state.key_macro.is_exec_end = true;
            }
            EvtAct::ctrl_editor(term);
        }
        term.curt().editor.state.key_macro.is_exec = false;
        term.curt().editor.state.key_macro.is_exec_end = false;
    }
}
