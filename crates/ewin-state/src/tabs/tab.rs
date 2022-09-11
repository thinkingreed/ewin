use crate::{editor::*, term::*};
use ewin_cfg::log::Log;
use ewin_key::model::*;
use ewin_utils::files::file::*;

impl State {
    #[track_caller]
    pub fn curt_mut_state(&mut self) -> &mut TabState {
        return self.tabs.vec.get_mut(self.tabs.idx).unwrap();
    }
    #[track_caller]
    pub fn curt_state(&mut self) -> &TabState {
        return self.tabs.vec.get_mut(self.tabs.idx).unwrap();
    }
    #[track_caller]
    pub fn tgt_state(&mut self, idx: usize) -> &TabState {
        return self.tabs.vec.get_mut(idx).unwrap();
    }
}

impl TabState {
    pub fn clear(&mut self) {
        Log::debug_key("TabState.clear");
        self.prom = PromState::default();
    }

    pub fn is_nomal(&self) -> bool {
        self.prom == PromState::None
    }

    pub fn is_nomal_or_grep_result(&self) -> bool {
        if self.is_nomal() || self.prom == PromState::GrepResult {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabState {
    pub editor: TabsEditorState,
    pub file: File,
    pub prom: PromState,
    pub grep: GrepInfo,
}
