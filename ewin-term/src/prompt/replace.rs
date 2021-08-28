use crate::{
    ewin_core::{_cfg::keys::*, global::*, log::Log, model::*, util::*},
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn replace(term: &mut Terminal) -> EvtActType {
        Log::info_key("EvtAct.replace");
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prompt_replace();
                return EvtActType::Next;
            }
            KeyCmd::ConfirmPrompt => {
                let mut search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let mut replace_str = term.curt().prom.cont_2.buf.iter().collect::<String>();

                search_str = change_regex(search_str);
                replace_str = change_regex(replace_str);

                if search_str.is_empty() {
                    term.curt().mbar.set_err(&LANG.not_entered_search_str);
                } else {
                    let end_idx;
                    {
                        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
                        end_idx = if cfg_search.regex { term.curt().editor.buf.len_bytes() } else { term.curt().editor.buf.len_chars() };
                    }
                    let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

                    let search_map = term.curt().editor.buf.search(&search_str, 0, end_idx, cfg_search);
                    if search_map.len() == 0 {
                        term.curt().mbar.set_err(&LANG.cannot_find_char_search_for);
                        return EvtActType::DrawOnly;
                    }
                    term.curt().editor.edit_proc(KeyCmd::ReplaceExec(cfg_search.regex, replace_str, search_map));

                    term.clear_curt_tab();
                    term.tabs[term.idx].editor.is_changed = true;
                }
                term.curt().editor.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}
