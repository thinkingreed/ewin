use crate::{model::*, tab::*};
use ewin_com::{_cfg::key::keycmd::*, model::*};

impl EvtAct {
    pub fn save_forced(term: &mut Terminal) -> ActType {
        if term.curt().prom.keycmd == KeyCmd::Resize {
            let h_file = term.curt_h_file().clone();
            term.curt().prom_save_forced(h_file);
            return ActType::Draw(DParts::All);
        }
        match &term.curt().prom.p_cmd {
            P_Cmd::InsertStr(str) => {
                if str == &'y'.to_string() {
                    let act_type = Tab::save(term, true);
                    term.clear_curt_tab(true);
                    return if let ActType::Draw(_) = act_type {
                        act_type
                    } else if let ActType::Next = act_type {
                        ActType::Draw(DParts::All)
                    } else {
                        ActType::Cancel
                    };
                } else if str == &'r'.to_string() {
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                } else {
                    return ActType::Cancel;
                }
            }
            _ => return ActType::Cancel,
        }
    }
}
