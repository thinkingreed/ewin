use ewin_cfg::log::Log;

use crate::{
    key::{cmd::*, keys::*},
    model::*,
};

impl TabState {
    pub fn clear(&mut self) {
        Log::debug_key("TabState.clear");
        self.prom = PromState::None;
    }

    pub fn is_nomal(&self) -> bool {
        match self.prom {
            PromState::Search | PromState::SaveConfirm | PromState::Replase | PromState::Grep | PromState::Greping | PromState::MoveRow | PromState::EncNl | PromState::SaveNewFile | PromState::SaveForced | PromState::WatchFile | PromState::OpenFile => return false,
            _ => {}
        };
        return true;
    }

    pub fn is_nomal_and_not_grep_result(&self) -> bool {
        if !self.is_nomal() || self.prom == PromState::GrepResult {
            return false;
        }
        true
    }

    pub fn judge_when_prompt(&self, keys: &Keys) -> bool {
        Log::debug_key("judge_when_prompt");
        Log::debug("self", &self);

        if !self.is_nomal() || (self.prom == PromState::GrepResult && keys == &Cmd::cmd_to_keys(CmdType::Confirm)) {
            return true;
        }
        return false;
    }
}
