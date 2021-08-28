use crate::{ewin_core::_cfg::keys::*, model::*, tab::*, terminal::*};
use ewin_core::{
    def::USIZE_UNDEFINED,
    model::{DrawType, EvtActType},
};

impl EvtAct {
    pub fn close(term: &mut Terminal) -> EvtActType {
        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                Tab::prompt_close(term);
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(str) => {
                if str == &'y'.to_string() {
                    if Tab::save(term) {
                        return EvtAct::check_exit_close(term);
                    } else {
                        term.curt().editor.draw_type = DrawType::All;
                        return EvtActType::DrawOnly;
                    }
                } else if str == &'n'.to_string() {
                    return EvtAct::check_exit_close(term);
                } else {
                    return EvtActType::Hold;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
    pub fn check_exit_close(term: &mut Terminal) -> EvtActType {
        if term.tabs.len() == 1 {
            return EvtActType::Exit;
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
                if is_exit {
                    return EvtActType::Exit;
                } else {
                    return EvtActType::DrawOnly;
                }
            } else {
                return EvtActType::DrawOnly;
            }
        }
    }
}
