use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
    model::*,
};

impl EvtAct {
    pub fn ctrl_statusbar(term: &mut Terminal) -> ActType {
        Log::debug_key("check_statusbar");

        match &term.keycmd {
            KeyCmd::StatusBar(s_cmd) => match &s_cmd {
                S_Cmd::MouseDownLeft(y, x) => {
                    let (x, _) = (*x as usize, *y as usize);
                    if term.curt().sbar.cur_area.0 <= x && x <= term.curt().sbar.cur_area.1 {
                        if term.curt().state.is_move_row {
                            term.clear_curt_tab(true, true);
                        } else {
                            term.curt().prom_move_row();
                        }
                        return ActType::Render(RParts::All);
                    }
                    if term.curt().sbar.enc_nl_area.0 <= x && x <= term.curt().sbar.enc_nl_area.1 {
                        if term.curt().state.is_enc_nl {
                            term.clear_curt_tab(true, true);
                        } else {
                            term.curt().prom_enc_nl();
                        }
                        return ActType::Render(RParts::All);
                    }
                    return ActType::Cancel;
                }
            },
            _ => return ActType::Next,
        }
    }
}
