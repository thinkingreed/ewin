use crate::tabs::*;
use ewin_cfg::log::*;

use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*},
};
use ewin_key::key::cmd::*;

impl Tabs {
    pub fn save_forced(&mut self) -> ActType {
        Log::debug_key("EvtAct.save_forced");

        match &self.curt().prom.cmd.cmd_type {
            CmdType::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_Y => {
                    let act_type = self.curt().save(&SaveFileType::Forced);
                    self.curt().clear_curt_tab(true);
                    return if let ActType::Draw(_) = act_type {
                        act_type
                    } else if let ActType::Next = act_type {
                        ActType::Draw(DParts::All)
                    } else {
                        ActType::Cancel
                    };
                }
                CHAR_R => {
                    self.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }
}
