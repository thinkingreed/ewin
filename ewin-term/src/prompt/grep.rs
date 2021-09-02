use crate::{
    ewin_core::{_cfg::keys::*, global::*, log::*, model::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use std::{env, path::*};

impl EvtAct {
    pub fn grep(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.grep");
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_grep();
                return EvtActType::Next;
            }
            KeyCmd::ConfirmPrompt => {
                let search_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let search_filenm = term.curt().prom.cont_2.buf.iter().collect::<String>();
                let mut search_folder = term.curt().prom.cont_3.buf.iter().collect::<String>();

                Log::debug("search_str", &search_str);
                Log::debug("search_filenm", &search_filenm);
                Log::debug("search_folder", &search_folder);

                if search_str.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_str);
                } else if search_filenm.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_file);
                } else if search_folder.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_search_folder);
                } else {
                    term.clear_curt_tab();
                    term.curt().state.clear_grep_info();

                    if search_folder.chars().nth(0).unwrap() != '/' && search_folder.chars().nth(0).unwrap() != 'C' {
                        let current_dir = env::current_dir().unwrap().display().to_string();
                        search_folder = format!("{}/{}", current_dir, search_folder);
                    }
                    Log::debug_s(&search_folder);
                    let path = Path::new(&search_folder).join(&search_filenm);

                    term.curt().prom.prom_grep.cache_search_filenm = search_filenm.clone();
                    term.curt().prom.prom_grep.cache_search_folder = search_folder.clone();

                    let mut grep_tab = Tab::new();
                    grep_tab.editor.search.str = search_str.clone();
                    grep_tab.editor.search.filenm = path.to_string_lossy().to_string();
                    grep_tab.editor.search.folder = search_folder.clone();
                    grep_tab.editor.mouse_mode = term.mouse_mode;

                    grep_tab.mbar.set_info(&LANG.searching);

                    grep_tab.state.grep_state.is_result = true;
                    grep_tab.state.grep_state.is_stdout_end = false;
                    grep_tab.state.grep_state.is_stderr_end = false;
                    grep_tab.state.grep_state.search_str = search_str.clone();
                    grep_tab.state.grep_state.search_filenm = search_filenm.clone();
                    grep_tab.state.grep_state.search_folder = search_folder.clone();
                    term.idx = term.tabs.len();
                    {
                        GREP_INFO_VEC.get().unwrap().try_lock().unwrap().push(grep_tab.state.grep_state.clone());
                    }
                    GREP_CANCEL_VEC.get().unwrap().try_lock().unwrap().resize_with(GREP_INFO_VEC.get().unwrap().try_lock().unwrap().len(), || false);

                    term.add_tab(grep_tab, HeaderFile::new(&format!(r#"{} "{}""#, &LANG.grep, &search_str)));
                    term.curt().prom.set_grep_working();
                    term.curt().editor.draw_type = DrawType::All;

                    return EvtActType::DrawOnly;
                }
                term.curt().editor.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}
