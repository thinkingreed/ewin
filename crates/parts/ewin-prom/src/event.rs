use crate::cont::parts::pulldown::*;
use crate::each::{enc_nl::*, grep::*, move_row::*, open_file::*, replace::*, save_confirm::*, save_forced::*, save_new_file::*, search::*, watch_file::*};
use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::{global::*, key::cmd::*, model::*};
use ewin_state::term::*;

impl Prom {
    pub fn ctrl_prom(&mut self, cmd: &Cmd) -> ActType {
        Log::debug_key("ctrl_prom");

        if !State::get().curt_ref_state().is_nomal() {
            self.set_cmd(cmd);

            // Cancel
            let evt_act = self.cancel_prom();
            if evt_act != ActType::Next {
                return evt_act;
            }

            let evt_act = self.curt.as_mut_base().ctrl_cont();

            Log::debug("evt_act", &evt_act);
            if evt_act != ActType::Next {
                return evt_act;
            }

            // let prom = self.curt().prom.curt;
            let prom_state = State::get().curt_ref_state().prom;
            return match prom_state {
                PromState::Search => self.curt.downcast_mut::<PromSearch>().unwrap().search(),
                PromState::SaveConfirm => self.curt.downcast_mut::<PromSaveConfirm>().unwrap().save_confirm(),
                PromState::Replase => self.curt.downcast_mut::<PromReplace>().unwrap().replace(),
                PromState::Grep => self.curt.downcast_mut::<PromGrep>().unwrap().grep(),
                PromState::EncNl => self.curt.downcast_mut::<PromEncNl>().unwrap().enc_nl(),
                PromState::MoveRow => self.curt.downcast_mut::<PromMoveRow>().unwrap().move_row(),
                PromState::SaveNewFile => self.curt.downcast_mut::<PromSaveNewFile>().unwrap().save_new_filenm(),
                PromState::SaveForced => self.curt.downcast_mut::<PromSaveForced>().unwrap().save_forced(),
                PromState::WatchFile => self.curt.downcast_mut::<PromWatchFile>().unwrap().watch_file(),
                PromState::OpenFile => self.curt.downcast_mut::<PromOpenFile>().unwrap().open_file_prom(),
                _ => ActType::Draw(DrawParts::TabsAll),
            };
        } else {
            return ActType::Next;
        }
    }

    pub fn cancel_prom(&mut self) -> ActType {
        match self.cmd.cmd_type {
            CmdType::CancelProm => {
                if State::get().tabs_all().is_all_close_confirm {
                    // self.cancel_close_all_tab();

                    State::get().tabs_mut_all().is_all_close_confirm = false;
                    // Job::send_cmd(CmdType::ClearTabState(true) );
                    State::get().curt_mut_state().clear();
                    return ActType::None;
                } else if State::get().tabs_all().is_all_save {
                    State::get().tabs_mut_all().is_all_save = false;
                    State::get().curt_mut_state().clear();
                    return Job::send_cmd(CmdType::ClearTabState(true));
                } else if State::get().curt_ref_state().prom == PromState::Greping {
                    GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceling).unwrap();
                } else if State::get().curt_ref_state().prom == PromState::GrepResult {
                } else if State::get().curt_ref_state().prom == PromState::Search {
                    State::get().curt_mut_state().clear();
                    return Job::send_cmd(CmdType::ClearTabState(true));
                } else {
                    return Job::send_cmd(CmdType::ClearTabState(true));
                }
                return ActType::Draw(DrawParts::TabsAll);
            }
            _ => return ActType::Next,
        }
    }

    pub fn is_prom_pulldown() -> bool {
        if !State::get().curt_ref_state().is_nomal() {
            for cont in Prom::get().curt.as_base().cont_vec.iter() {
                if let Ok(pulldown_cont) = cont.downcast_ref::<PromContPulldown>() {
                    if pulldown_cont.pulldown.is_show {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn judge_when_prompt() -> bool {
        let is_nomal_or_grep_result = State::get().curt_ref_state().is_nomal_or_grep_result();
        // if !is_nomal_or_grep_result || (State::get().curt_state().prom == PromState::GrepResult && keys == Cmd::cmd_to_keys(CmdType::Confirm)) {
        if !is_nomal_or_grep_result {
            return true;
        }
        return false;
    }
}
