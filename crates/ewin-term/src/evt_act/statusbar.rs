use crate::{model::*, terms::term::*};
use ewin_cfg::log::*;
use ewin_const::model::*;
use ewin_key::{key::cmd::*, model::*};

impl EvtAct {
    pub fn ctrl_statusbar(term: &mut Term) -> ActType {
        Log::debug_key("ctrl_statusbar");

        match &term.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                Log::debug("xxx", &x);
                Log::debug("term.curt().sbar.cur_area", &term.clone().curt().sbar.cur_area);
                Log::debug("term.curt().sbar.enc_nl_area", &term.clone().curt().sbar.enc_nl_area);

                let (x, _) = (*x as usize, *y as usize);
                if term.curt().sbar.cur_area.0 <= x && x <= term.curt().sbar.cur_area.1 {
                    // if term.curt().state.is_move_row {
                    if term.curt().state.prom == PromState::MoveRow {
                        term.curt().clear_curt_tab(true);
                    } else {
                        term.curt().prom_show_com(&CmdType::MoveRowProm);
                    }
                    return ActType::Draw(DParts::All);
                }
                Log::debug("term.curt().sbar.enc_nl_area", &term.curt().sbar.enc_nl_area);

                if term.curt().sbar.enc_nl_area.0 <= x && x <= term.curt().sbar.enc_nl_area.1 {
                    if term.curt().state.prom == PromState::EncNl {
                        term.curt().clear_curt_tab(true);
                    } else {
                        return term.curt().prom_show_com(&CmdType::EncodingProm);
                    }
                    return ActType::Draw(DParts::All);
                }
                return ActType::Cancel;
            }

            _ => return ActType::Next,
        }
    }
}
