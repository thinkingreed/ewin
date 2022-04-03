use crate::{model::*, tab::*};
use ewin_com::{_cfg::key::keycmd::*, model::*};

impl EvtAct {
    pub fn save_forced(term: &mut Terminal) -> ActType {
        match &term.curt().prom.p_cmd {
            P_Cmd::Resize(_, _) => {
                let h_file = term.curt_h_file().clone();
                term.curt().prom_save_forced(&h_file.modified_time, &h_file.fullpath);
                return ActType::Render(RParts::All);
            }
            P_Cmd::InsertStr(str) => {
                if str == &'y'.to_string() {
                    let act_type = Tab::save(term, SaveType::Forced);
                    term.clear_curt_tab(true, true);
                    return if let ActType::Render(_) = act_type {
                        act_type
                    } else if let ActType::Next = act_type {
                        ActType::Render(RParts::All)
                    } else {
                        ActType::Cancel
                    };
                } else if str == &'r'.to_string() {
                    term.reopen_curt_file();
                    return ActType::Render(RParts::All);
                } else {
                    return ActType::Cancel;
                }
            }
            _ => return ActType::Cancel,
        }
    }
}
