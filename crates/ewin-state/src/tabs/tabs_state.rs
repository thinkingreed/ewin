use crate::editor::*;
use ewin_cfg::log::Log;
use ewin_key::model::*;
use ewin_utils::files::file::*;

impl TabsState {
    pub fn clear(&mut self) {
        Log::debug_key("TabState.clear");

        self.prom = PromState::default();
    }

    pub fn is_nomal(&self) -> bool {
        self.prom == PromState::None
    }

    pub fn is_nomal_and_not_grep_result(&self) -> bool {
        if !self.is_nomal() || self.prom == PromState::GrepResult {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabsState {
    pub editor: TabsEditorState,
    pub file: File,
    pub prom: PromState,
    pub grep: GrepInfo,
}
