use crate::tabs::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, evt::*};
use ewin_key::{key::cmd::*, model::*};
use ewin_state::term::*;

impl Tabs {
    pub fn ctrl_statusbar(tabs: &mut Tabs, cmd_type: &CmdType) -> ActType {
        Log::debug_key("ctrl_statusbar");

        match cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                Log::debug("xxx", &x);
                Log::debug("tabs.curt().sbar.cur_area", &tabs.curt().sbar.cur_area);
                Log::debug("tabs.curt().sbar.enc_nl_area", &tabs.curt().sbar.enc_nl_area);

                let (x, _) = (*x as usize, *y as usize);
                if tabs.curt().sbar.cur_area.0 <= x && x <= tabs.curt().sbar.cur_area.1 {
                    // if tabs.curt().state.is_move_row {
                    if State::get().curt_state().prom == PromState::MoveRow {
                        tabs.curt().clear_curt_tab(true);
                    } else {
                        tabs.curt().prom_show_com(&CmdType::MoveRowProm);
                    }
                    return ActType::Draw(DParts::All);
                }
                Log::debug("tabs.curt().sbar.enc_nl_area", &tabs.curt().sbar.enc_nl_area);

                if tabs.curt().sbar.enc_nl_area.0 <= x && x <= tabs.curt().sbar.enc_nl_area.1 {
                    if State::get().curt_state().prom == PromState::EncNl {
                        tabs.curt().clear_curt_tab(true);
                    } else {
                        return tabs.curt().prom_show_com(&CmdType::EncodingProm);
                    }
                    return ActType::Draw(DParts::All);
                }
                return ActType::Cancel;
            }

            _ => return ActType::Next,
        }
    }
}
