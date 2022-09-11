use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_key::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, model::*},
};
use ewin_job::job::*;
use ewin_key::{global::*, model::*};
use ewin_state::term::*;
use ewin_utils::files::file::*;

impl PromWatchFile {
    pub fn watch_file(&mut self) -> ActType {
        Log::debug_key("EvtAct::grep_result");

        match &self.base.cmd.cmd_type {
            CmdType::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_R => {
                    Job::send_cmd(CmdType::ReOpenFile(FileOpenType::Reopen));
                    return ActType::None;
                }
                CHAR_L => {
                    State::get().curt_mut_state().file.watch_mode = WatchMode::NotEditedWillReloadedAuto;

                    Job::send_cmd(CmdType::ReOpenFile(FileOpenType::Reopen));
                    return ActType::None;
                }
                CHAR_N => {
                    State::get().curt_mut_state().file.watch_mode = WatchMode::NotMonitor;
                    WATCH_INFO.get().unwrap().try_lock().map(|mut watch_info| watch_info.mode = WatchMode::NotMonitor).unwrap();

                    State::get().curt_mut_state().clear();
                    return ActType::Draw(DrawParts::TabsAll);
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }

    pub fn check_watch_file() -> bool {
        // Check if the file has been updated after opening
        let file = State::get().curt_state().file.clone();
        if State::get().curt_state().is_nomal() {
            if file.watch_mode == WatchMode::NotEditedWillReloadedAuto && !State::get().curt_mut_state().editor.is_changed {
                Job::send_cmd(CmdType::ReOpenFile(FileOpenType::Reopen));
                return true;
            } else if let Some(latest_mod_time) = File::get_modified_time(&file.fullpath) {
                if latest_mod_time > file.mod_time {
                    Log::debug("latest_modified_time > h_file.modified_time ", &(latest_mod_time > file.mod_time));
                    Job::send_cmd(CmdType::WatchFileResultProm);
                    return true;
                }
            }
        }
        return false;
    }

    pub fn new() -> Self {
        let mut prom = PromWatchFile { ..PromWatchFile::default() };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().file_has_been_modified_by_other_app.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let reopen = PromContKeyMenu { disp_str: format!("{}({})", Lang::get().reopen, Lang::get().edit_discard), key: PromContKeyMenuType::OneChar(CHAR_R.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        let load = PromContKeyMenu { disp_str: Lang::get().not_edited_will_reloaded_auto.to_string(), key: PromContKeyMenuType::OneChar(CHAR_L.to_string()) };
        let not = PromContKeyMenu { disp_str: Lang::get().no_further_monitoring.to_string(), key: PromContKeyMenuType::OneChar(CHAR_N.to_string()) };

        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![reopen]], ..PromContKeyDesc::default() }));
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![cancel]], ..PromContKeyDesc::default() }));
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![load]], ..PromContKeyDesc::default() }));
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![not]], ..PromContKeyDesc::default() }));

        return prom;
    }

    pub fn init() -> ActType {
        Log::debug_key("Tab::prom_watch_result");
        State::get().curt_mut_state().prom = PromState::WatchFile;
        Prom::get().init(Box::new(PromWatchFile::new()));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromWatchFile {
    pub base: PromBase,
}

impl PromTrait for PromWatchFile {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
