use crate::{model::*, terms::term::*};
use ewin_cfg::log::*;
use ewin_const::{def::*, model::*};
use ewin_key::key::cmd::*;

impl EvtAct {
    pub fn save_forced(term: &mut Term) -> ActType {
        Log::debug_key("EvtAct.save_forced");

        match &term.curt().prom.cmd.cmd_type {
            CmdType::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_Y => {
                    let act_type = term.curt().save(SaveType::Forced);
                    term.curt().clear_curt_tab(true);
                    return if let ActType::Draw(_) = act_type {
                        act_type
                    } else if let ActType::Next = act_type {
                        ActType::Draw(DParts::All)
                    } else {
                        ActType::Cancel
                    };
                }
                CHAR_R => {
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }
}
