use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn search(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.search");
        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_search();
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(p_cmd) => match p_cmd {
                P_Cmd::InsertStr(_) | P_Cmd::Cut | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Undo | P_Cmd::Redo => {
                    let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    term.curt().editor.exec_search_incremental(search_str);
                    return ActType::Draw(DParts::All);
                }
                P_Cmd::FindNext | P_Cmd::FindBack => return EvtAct::exec_search_confirm(term),
                _ => return if EvtAct::is_draw_prompt_tgt_keycmd(&term.curt().prom.p_cmd) { ActType::Draw(DParts::Prompt) } else { ActType::Cancel },
            },
            _ => return ActType::Cancel,
        };
    }

    pub fn exec_search_confirm(term: &mut Terminal) -> ActType {
        Log::debug_s("exec_search_confirm");

        if let KeyCmd::Prom(p_cmd) = &term.keycmd {
            term.curt().editor.e_cmd = match p_cmd {
                P_Cmd::FindNext => E_Cmd::FindNext,
                P_Cmd::FindBack => E_Cmd::FindBack,
                _ => E_Cmd::Null,
            };
        }

        let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
        if let Some(err_str) = term.curt().editor.exec_search_confirm(search_str) {
            return ActType::Draw(DParts::MsgBar(err_str));
        } else {
            // Do not clear grep information in case of grep result
            // Because grep result cannot be judged
            term.clear_curt_tab(false);
            return ActType::Draw(DParts::All);
        }
    }
}
