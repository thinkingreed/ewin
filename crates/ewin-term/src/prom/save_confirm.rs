use crate::model::*;
use ewin_cfg::log::*;
use ewin_com::{_cfg::key::keycmd::*, model::*};
use ewin_const::def::*;
impl EvtAct {
    pub fn save_confirm(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::save_confirm");
        Log::debug("term.curt().prom_plugin.p_cmd", &term.curt().prom.p_cmd);
        match &term.curt().prom.p_cmd {
            P_Cmd::InsertStr(ref string) => match string.to_uppercase().as_str() {
                CHAR_Y => {
                    let act_type = Tab::save(term, SaveType::Normal);
                    return if let ActType::Draw(_) = act_type { act_type } else { EvtAct::check_exit_close(term) };
                }
                CHAR_N => return EvtAct::check_exit_close(term),
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }

    pub fn check_exit_close(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::check_exit_close");

        if term.tabs.len() == 1 {
            return ActType::Exit;
        } else {
            term.del_tab(term.tab_idx);
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
