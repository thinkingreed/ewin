use crate::{
    ewin_com::{_cfg::key::keycmd::*, files::file::*, global::*, model::*},
    model::*,
};
use ewin_cfg::log::*;
use ewin_const::def::*;
use std::io::Write;

impl EvtAct {
    pub fn draw_watch_file<T: Write>(out: &mut T, term: &mut Terminal) -> bool {
        // Check if the file has been updated after opening
        let h_file = term.curt_h_file().clone();

        if term.curt().state.is_nomal() {
            if h_file.watch_mode == WatchMode::NotEditedWillReloadedAuto && !term.curt().editor.state.is_changed {
                term.reopen_curt_file();
                term.draw_all(out, DParts::All);
                return true;
            } else if let Some(latest_mod_time) = File::get_modified_time(&h_file.fullpath) {
                if latest_mod_time > h_file.mod_time {
                    Log::debug("latest_modified_time > h_file.modified_time ", &(latest_mod_time > h_file.mod_time));
                    term.curt().prom_watch_result();
                    term.draw(out, &DParts::All);
                    return true;
                }
            }
        }
        return false;
    }
    pub fn watch_file(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match &term.curt().prom.p_cmd {
            P_Cmd::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_R => {
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                CHAR_L => {
                    term.curt_h_file().watch_mode = WatchMode::NotEditedWillReloadedAuto;
                    Log::debug("WATCH_INFO", &WATCH_INFO);
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                CHAR_N => {
                    term.curt_h_file().watch_mode = WatchMode::NotMonitor;
                    WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.mode = WatchMode::NotMonitor).unwrap();
                    term.curt().clear_curt_tab(true);
                    return ActType::Draw(DParts::All);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }
}
