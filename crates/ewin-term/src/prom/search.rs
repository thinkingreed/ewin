use crate::tab::*;
use ewin_cfg::log::*;
use ewin_const::model::*;
use ewin_key::key::cmd::*;

impl Tab {
    pub fn search(&mut self) -> ActType {
        Log::debug_key("EvtAct.search");
        let search_str = self.prom.curt.as_mut_base().get_curt_input_area_str();

        match self.prom.cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::Cut | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo => {
                self.editor.exec_search_incremental(search_str);
                return ActType::Draw(DParts::All);
            }
            CmdType::FindNext | CmdType::FindBack => return self.exec_search_confirm(search_str),
            _ => return ActType::Cancel,
        }
    }

    pub fn exec_search_confirm(&mut self, search_str: String) -> ActType {
        Log::debug_s("Tab.exec_search_confirm");

        self.editor.cmd = self.prom.cmd.clone();

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
