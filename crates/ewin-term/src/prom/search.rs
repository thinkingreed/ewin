use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*},
    model::Tab,
};
use ewin_cfg::log::*;

impl Tab {
    pub fn search(&mut self) -> ActType {
        Log::debug_key("EvtAct.search");

        let search_str = self.prom.curt.as_mut_base().get_curt_input_area_str();

        match &self.prom.p_cmd {
            P_Cmd::InsertStr(_) | P_Cmd::Cut | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Undo | P_Cmd::Redo => {
                self.editor.exec_search_incremental(search_str);
                return ActType::Draw(DParts::All);
            }
            P_Cmd::FindNext | P_Cmd::FindBack => return self.exec_search_confirm(search_str),
            _ => return ActType::Cancel,
        };
    }

    pub fn exec_search_confirm(&mut self, search_str: String) -> ActType {
        Log::debug_s("Tab.exec_search_confirm");

        self.editor.e_cmd = match self.prom.p_cmd {
            P_Cmd::FindNext => E_Cmd::FindNext,
            P_Cmd::FindBack => E_Cmd::FindBack,
            _ => E_Cmd::Null,
        };

        let act_type = self.editor.exec_search_confirm(search_str);
        if act_type != ActType::Next {
            return act_type;
        }
        // Do not clear grep information in case of grep result
        // Because grep result cannot be judged
        self.clear_curt_tab(false);
        return ActType::Draw(DParts::All);
    }
}
