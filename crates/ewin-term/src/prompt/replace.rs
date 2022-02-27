use ewin_com::_cfg::model::default::Cfg;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, log::*, model::*, util::*},
    model::*,
};

impl EvtAct {
    pub fn replace(term: &mut Terminal) -> ActType {
        Log::info_key("EvtAct.replace");
        match &term.curt().prom.keycmd {
            KeyCmd::Prom(P_Cmd::Resize(_, _)) => {
                term.curt().prom_replace();
                return ActType::Render(RParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                let mut search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let mut replace_str = term.curt().prom.cont_2.buf.iter().collect::<String>();

                search_str = change_regex(search_str);
                replace_str = change_regex(replace_str);

                if search_str.is_empty() {
                    return ActType::Render(RParts::MsgBar(Lang::get().not_entered_search_str.to_string()));
                } else {
                    let cfg_search = Cfg::get_edit_search();
                    let end_idx = if cfg_search.regex { term.curt().editor.buf.len_bytes() } else { term.curt().editor.buf.len_chars() };

                    let idx_set = term.curt().editor.buf.search(&search_str, 0, end_idx, &cfg_search);
                    if idx_set.is_empty() {
                        return ActType::Render(RParts::MsgBar(Lang::get().cannot_find_char_search_for.to_string()));
                    }
                    term.curt().editor.edit_proc(E_Cmd::ReplaceExec(search_str, replace_str, idx_set));

                    term.clear_curt_tab(true);
                    term.tabs[term.tab_idx].editor.state.is_changed = true;
                }
                return ActType::Render(RParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
