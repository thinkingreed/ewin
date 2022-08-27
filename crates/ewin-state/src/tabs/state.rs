use crate::{editor::*, global::*, term::*};
use ewin_cfg::{log::*, model::default::*};
use ewin_const::models::model::*;
use ewin_key::model::*;
use ewin_utils::files::file::*;
use tokio::sync::{MutexGuard, TryLockError};

use super::{all::*, tabs_state::*};

impl State {
    #[track_caller]
    pub fn curt_mut_state(&mut self) -> &mut TabsState {
        return self.tabs.vec.get_mut(self.tabs.idx).unwrap();
    }
    #[track_caller]
    pub fn curt_state(&mut self) -> &TabsState {
        return self.tabs.vec.get_mut(self.tabs.idx).unwrap();
    }
    #[track_caller]
    pub fn tgt_state(&mut self, idx: usize) -> &TabsState {
        return self.tabs.vec.get_mut(idx).unwrap();
    }

    #[track_caller]
    pub fn get() -> MutexGuard<'static, State> {
        return TABS.get().unwrap().try_lock().unwrap();
    }

    #[track_caller]
    pub fn get_result() -> Result<MutexGuard<'static, State>, TryLockError> {
        return TABS.get().unwrap().try_lock();
    }

    #[track_caller]
    pub fn del_file(&mut self, del_idx: usize, curt_idx: usize) {
        self.tabs.idx = curt_idx;
        // self.h_file_vec.remove(del_idx);
        self.tabs.vec.remove(del_idx);
        self.tabs.all.set_idx_close_other_than_this_tab(del_idx);
    }

    #[track_caller]
    pub fn get_init_file_info() -> State {
        return State { term: TermState::default(), tabs: TabsStates::default() };
    }

    pub fn add_tab(&mut self, file: File) {
        self.tabs.idx = self.tabs.vec.len();
        self.tabs.vec.insert(self.tabs.idx, TabsState { file, ..TabsState::default() });
    }
}

impl Default for TabsState {
    fn default() -> Self {
        TabsState {
            editor: TabsEditorState {
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TabsStates {
    pub idx: usize,
    pub all: TabsAllState,
    pub vec: Vec<TabsState>,
}
