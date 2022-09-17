use crate::model::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, event::*, term::*};
use ewin_job::job::*;
use ewin_state::{tabs::editor::*, term::*};

impl Editor {
    pub fn record_key_macro_start(&mut self) -> ActType {
        Log::debug_key("macro_record_start");

        if State::get().curt_state().editor.key_macro.is_record {
            State::get().curt_mut_state().editor.toggle_state(TabsEditerStateType::KeyMacroRecord);
            return ActType::Draw(DrawParts::Editor(E_DrawRange::All));
        } else {
            self.key_vec = vec![];
            State::get().curt_mut_state().editor.toggle_state(TabsEditerStateType::KeyMacroRecord);
            return ActType::Draw(DrawParts::StatusBar);
        }
    }

    pub fn exec_key_macro(&mut self) -> ActType {
        if self.key_vec.is_empty() {
            return ActType::Draw(DrawParts::MsgBar(Lang::get().no_key_record_exec.to_string()));
        }
        for (i, mac) in self.key_vec.iter().enumerate() {
            let act_type = if i == self.key_vec.len() - 1 { ActType::Draw(DrawParts::TabsAll) } else { ActType::None };
            Job::send_cmd_act_type(mac.cmd_type.clone(), Place::Editor, Some(act_type));
        }

        return ActType::None;
    }
}
