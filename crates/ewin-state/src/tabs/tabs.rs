use crate::term::*;
use ewin_cfg::model::general::default::*;
use ewin_const::models::model::*;
use ewin_key::model::*;
use ewin_utils::files::file::*;

use super::{all::*, editor::*, tab::*};

impl State {
    pub fn add_tab(&mut self, file: File) {
        self.tabs.idx = self.tabs.vec.len();
        self.tabs.vec.insert(self.tabs.idx, TabState { file, ..TabState::default() });
    }

    pub fn del_file(&mut self, del_idx: usize, curt_idx: usize) {
        self.tabs.idx = curt_idx;
        // self.h_file_vec.remove(del_idx);
        self.tabs.vec.remove(del_idx);
        self.tabs.all.set_idx_close_other_than_this_tab(del_idx);
    }

    pub fn is_opened_file_idx(&self, fullpath: &str) -> Option<usize> {
        self.tabs.vec.iter().position(|tab| tab.file.fullpath == fullpath)
    }
}

impl TabsState {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TabsState {
    pub idx: usize,
    pub all: TabsAllState,
    pub vec: Vec<TabState>,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            editor: EditorState {
                is_read_only: false,
                is_changed: false,
                // is_changed_org: false,
                mouse: Mouse::Enable,
                is_dragging: false,
                scale: Cfg::get().general.editor.scale,
                row_no: Cfg::get().general.editor.row_no,
                window_split_type: WindowSplitType::None,
                key_macro: KeyMacroState::default(),
            },
            file: File::default(),
            prom: PromState::default(),
            grep: GrepInfo::default(),
        }
    }
}
