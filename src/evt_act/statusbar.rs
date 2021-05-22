use crate::{log::*, model::*, prompt::prompt::Prompt, terminal::Terminal};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};

impl EvtAct {
    pub fn check_statusbar(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_statusbar");

        let evt = term.curt().editor.evt.clone();
        let tab = term.curt();

        match evt {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);
                if y != tab.sbar.disp_row_posi {
                    return EvtActType::Hold;
                }
                if tab.sbar.enc_nl_area.0 <= x && x <= tab.sbar.enc_nl_area.1 {
                    if term.curt().state.is_enc_nl {
                        term.clear_curt_tab_status();
                        return EvtActType::DrawOnly;
                    } else {
                        Prompt::enc_nl(term);
                    }
                }
                return EvtActType::Hold;
            }
            _ => return EvtActType::Hold,
        }
    }
}
