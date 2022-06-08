use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*},
    model::*,
};
use ewin_cfg::log::*;

impl EvtAct {
    pub fn ctrl_statusbar(term: &mut Terminal) -> ActType {
        Log::debug_key("check_statusbar");

        match &term.keycmd {
            KeyCmd::StatusBar(s_cmd) => match &s_cmd {
                S_Cmd::MouseDownLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    if term.curt().sbar.cur_area.0 <= x && x <= term.curt().sbar.cur_area.1 {
                        // if term.curt().state.is_move_row {
                        if term.curt().state.prom == PromState::MoveRow {
                            term.curt().clear_curt_tab(true);
                        } else {
                            term.curt().prom_move_row();
                        }
                        return ActType::Draw(DParts::All);
                    }
                    if term.curt().sbar.enc_nl_area.0 <= x && x <= term.curt().sbar.enc_nl_area.1 {
                        if term.curt().state.prom == PromState::EncNl {
                            term.curt().clear_curt_tab(true);
                        } else {
                            let h_file = &term.curt_h_file().clone();
                            return term.curt().prom_enc_nl(h_file);
                        }
                        return ActType::Draw(DParts::All);
                    }
                    return ActType::Cancel;
                }
            },
            _ => return ActType::Next,
        }
    }
}
