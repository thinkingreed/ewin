use ewin_com::global::WATCH_INFO;

use crate::{
    ewin_com::{_cfg::key::keycmd::*, file::*, log::*, model::*},
    model::*,
};
use std::{io::Write, path::Path};

impl EvtAct {
    pub fn draw_watch_result<T: Write>(out: &mut T, term: &mut Terminal) -> bool {
        Log::debug_key("draw_watch_result");

        // Check if the file has been updated after opening
        let h_file = term.curt_h_file().clone();

        if term.curt().state.is_nomal() {
            if h_file.watch_mode == WatchMode::NotEditedWillReloadedAuto && !term.curt().editor.state.is_changed {
                term.reopen_curt_file();
                term.render_all(out, DParts::All);
                return true;
            } else if Path::new(&h_file.fullpath).exists() && Path::new(&h_file.fullpath).is_file() {
                Log::debug("h_file.fullpath", &h_file.fullpath);
                let latest_modified_time = File::get_modified_time(&h_file.fullpath);
                if latest_modified_time > h_file.modified_time {
                    Log::debug("latest_modified_time > h_file.modified_time ", &(latest_modified_time > h_file.modified_time));
                    term.curt().prom_watch_result();
                    term.render(out, &DParts::All);
                    return true;
                }
            }
        }
        return false;
    }
    pub fn watch_result(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.set_disp_size();
                term.curt().prom.watch_result();
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(P_Cmd::InsertStr(c)) => {
                if c == &'r'.to_string() {
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                } else if c == &'l'.to_string() {
                    term.curt_h_file().watch_mode = WatchMode::NotEditedWillReloadedAuto;
                    // WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.mode = WatchMode::NotEditedWillReloadedAuto).unwrap();

                    Log::debug("WATCH_INFO", &WATCH_INFO);
                    term.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                } else if c == &'n'.to_string() {
                    term.curt_h_file().watch_mode = WatchMode::NotMonitor;
                    WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.mode = WatchMode::NotMonitor).unwrap();
                    term.clear_curt_tab(true);
                    return ActType::Draw(DParts::All);
                } else {
                    return ActType::Cancel;
                }
            }
            _ => return ActType::Cancel,
        }
    }
}
