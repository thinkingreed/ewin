use crate::{model::*, tab::*, terminal::*};
use ewin_core::{_cfg::key::keycmd::*, def::*, model::*};

impl EvtAct {
    pub fn close(term: &mut Terminal) -> ActType {
        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                Tab::prom_close(term);
                return ActType::Draw(DParts::All);
            }
            _ => {}
        }
        match &term.curt().prom.p_cmd {
            P_Cmd::InsertStr(str) => {
                if str == &'y'.to_string() {
                    let act_type = Tab::save(term);
                    return if let ActType::Draw(_) = act_type { act_type } else { EvtAct::check_exit_close(term) };
                } else if str == &'n'.to_string() {
                    return EvtAct::check_exit_close(term);
                } else {
                    return ActType::Cancel;
                }
            }
            _ => return ActType::Cancel,
        }
    }
    pub fn check_exit_close(term: &mut Terminal) -> ActType {
        if term.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            term.del_tab(term.idx);
            if term.state.is_all_close_confirm || term.state.close_other_than_this_tab_idx != USIZE_UNDEFINED {
                let is_exit = if term.state.is_all_close_confirm {
                    term.close_tabs(USIZE_UNDEFINED)
                } else if term.tabs.len() == 1 {
                    false
                } else {
                    term.close_tabs(term.state.close_other_than_this_tab_idx)
                };
                return if is_exit { ActType::Exit } else { ActType::Draw(DParts::All) };
            } else {
                return ActType::Draw(DParts::All);
            }
        }
    }
}
