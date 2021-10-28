use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, global::*, log::*, model::*, util::*},
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn replace(term: &mut Terminal) -> ActType {
        Log::info_key("EvtAct.replace");
        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_replace();
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(P_Cmd::ConfirmPrompt) => {
                let mut search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let mut replace_str = term.curt().prom.cont_2.buf.iter().collect::<String>();

                search_str = change_regex(search_str);
                replace_str = change_regex(replace_str);

                if search_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_search_str.to_string()));
                } else {
                    let end_idx;
                    {
                        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
                        end_idx = if cfg_search.regex { term.curt().editor.buf.len_bytes() } else { term.curt().editor.buf.len_chars() };
                    }
                    let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

                    let search_map = term.curt().editor.buf.search(&search_str, 0, end_idx, cfg_search);
                    if search_map.is_empty() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().cannot_find_char_search_for.to_string()));
                    }
                    term.curt().editor.edit_proc(E_Cmd::ReplaceExec(cfg_search.regex, replace_str, search_map));

                    term.clear_curt_tab(true);
                    term.tabs[term.idx].editor.state.is_changed = true;
                }
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
