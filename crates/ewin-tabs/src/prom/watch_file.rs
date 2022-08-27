use crate::tabs::*;
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, evt::*, model::*},
};
use ewin_key::{global::*, key::cmd::*};
use ewin_state::term::*;
use ewin_utils::files::file::*;
use std::io::Write;

impl Tabs {
    pub fn draw_watch_file<T: Write>(&mut self, out: &mut T) -> bool {
        // Check if the file has been updated after opening

        let file = State::get().curt_state().file.clone();
        if State::get().curt_state().is_nomal() {
            if file.watch_mode == WatchMode::NotEditedWillReloadedAuto && !State::get().curt_mut_state().editor.is_changed {
                self.reopen_curt_file();
                self.draw_all(out, DParts::All);
                return true;
            } else if let Some(latest_mod_time) = File::get_modified_time(&file.fullpath) {
                if latest_mod_time > file.mod_time {
                    Log::debug("latest_modified_time > h_file.modified_time ", &(latest_mod_time > file.mod_time));
                    self.curt().prom_show_com(&CmdType::WatchFileResult);
                    self.draw(out, &DParts::All);
                    return true;
                }
            }
        }
        return false;
    }
    pub fn watch_file(&mut self) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match &self.curt().prom.cmd.cmd_type {
            CmdType::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_R => {
                    self.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                CHAR_L => {
                    State::get().curt_mut_state().file.watch_mode = WatchMode::NotEditedWillReloadedAuto;
                    Log::debug("WATCH_INFO", &WATCH_INFO);
                    self.reopen_curt_file();
                    return ActType::Draw(DParts::All);
                }
                CHAR_N => {
                    State::get().curt_mut_state().file.watch_mode = WatchMode::NotMonitor;
                    WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.mode = WatchMode::NotMonitor).unwrap();
                    self.curt().clear_curt_tab(true);
                    return ActType::Draw(DParts::All);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }
}
