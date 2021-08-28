use crate::{
    ewin_core::{_cfg::keys::*, log::*, model::*},
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn search(term: &mut Terminal) -> EvtActType {
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prompt_search();
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::Undo | KeyCmd::Redo => {
                let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                term.curt().editor.exec_search_incremental(search_str);
                return EvtActType::DrawOnly;
            }
            KeyCmd::FindNext | KeyCmd::FindBack => return EvtAct::exec_search_confirm(term),
            _ => return EvtActType::Hold,
        };
    }

    pub fn exec_search_confirm(term: &mut Terminal) -> EvtActType {
        Log::debug_s("exec_search_confirm");
        let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
        if let Some(err_str) = term.curt().editor.exec_search_confirm(search_str) {
            term.curt().mbar.set_err(&err_str);
            return EvtActType::DrawOnly;
        } else {
            term.clear_curt_tab();
            return EvtActType::Next;
        }
    }
}
