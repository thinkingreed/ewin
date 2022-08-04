use crate::tab::Tab;
use ewin_cfg::{lang::lang_cfg::*, log::*, model::default::*};
use ewin_const::model::*;
use ewin_key::{key::cmd::*, util::*};

impl Tab {
    pub fn replace(&mut self) -> ActType {
        Log::info_key("EvtAct.replace");

        match &self.prom.cmd.cmd_type {
            CmdType::Confirm => {
                let mut search_str = self.prom.curt.as_mut_base().get_tgt_input_area_str(0);
                let mut replace_str = self.prom.curt.as_mut_base().get_tgt_input_area_str(1);

                search_str = change_regex(search_str);
                replace_str = change_regex(replace_str);

                if search_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_set_search_str.to_string()));
                } else {
                    let cfg_search = CfgEdit::get_search();
                    let end_idx = if cfg_search.regex { self.editor.buf.len_bytes() } else { self.editor.buf.len_chars() };

                    let idx_set = self.editor.buf.search(&search_str, 0, end_idx, &cfg_search);
                    if idx_set.is_empty() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().cannot_find_search_char.to_string()));
                    }
                    self.editor.edit_proc_cmd_type(CmdType::ReplaceExec(search_str, replace_str, idx_set));

                    self.clear_curt_tab(true);
                    self.editor.state.is_changed = true;
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
