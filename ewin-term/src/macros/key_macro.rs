use crate::{ewin_core::global::LANG, ewin_core::log::Log, ewin_core::model::DrawType, model::EvtAct, tab::Tab, terminal::Terminal};
use std::io::Write;

impl Tab {
    pub fn record_key_macro_start(&mut self) {
        Log::debug_key("macro_record_start");
        if self.state.key_macro_state.is_record {
            self.state.key_macro_state.is_record = false;
            self.mbar.clear_keyrecord();
            self.editor.draw_type = DrawType::All;
        } else {
            self.state.key_macro_state.is_record = true;
            self.mbar.set_keyrecord(&LANG.key_recording);
            self.editor.key_vec = vec![];
        }
    }

    pub fn exec_macro_key<T: Write>(out: &mut T, term: &mut Terminal) {
        Log::debug("key_record_vec", &term.curt().editor.key_vec);

        if term.curt().editor.key_vec.len() > 0 {
            term.curt().state.key_macro_state.is_exec = true;

            let macro_vec = term.curt().editor.key_vec.clone();
            for (i, mac) in macro_vec.iter().enumerate() {
                term.curt().editor.keys = mac.keys;
                if i == macro_vec.len() - 1 {
                    term.curt().state.key_macro_state.is_exec_end = true;
                }
                EvtAct::match_event(term.curt().editor.keys, out, term);
            }
            term.curt().state.key_macro_state.is_exec = false;
            term.curt().state.key_macro_state.is_exec_end = false;
        } else {
            term.curt().mbar.set_err(&LANG.no_key_record_exec.to_string());
        }
    }
}
