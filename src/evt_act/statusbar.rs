use crate::{_cfg::keys::KeyCmd, log::*, model::*, prompt::prompt::prompt::*, terminal::Terminal};

impl EvtAct {
    pub fn check_statusbar(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_statusbar");

        let tab = term.curt();

        match tab.prom.keycmd {
            KeyCmd::MouseDownLeft(y, x) => {
                let (x, y) = (x as usize, y as usize);
                if y != tab.sbar.disp_row_posi {
                    return EvtActType::Hold;
                }
                if tab.sbar.cur_area.0 <= x && x <= tab.sbar.cur_area.1 {
                    if term.curt().state.is_move_row {
                        term.clear_curt_tab();
                        return EvtActType::DrawOnly;
                    } else {
                        Prompt::move_row(term);
                        return EvtActType::DrawOnly;
                    }
                }
                if tab.sbar.enc_nl_area.0 <= x && x <= tab.sbar.enc_nl_area.1 {
                    if term.curt().state.is_enc_nl {
                        term.clear_curt_tab();
                        return EvtActType::DrawOnly;
                    } else {
                        Prompt::enc_nl(term);
                        return EvtActType::DrawOnly;
                    }
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
