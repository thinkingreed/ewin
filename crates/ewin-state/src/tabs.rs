use crate::{editor_state::*, global::*, header_file::*};
use ewin_cfg::model::default::*;
use ewin_key::model::*;
use tokio::sync::MutexGuard;

impl Tabs {
    /*
     * editor_cfg
     */
    #[track_caller]
    pub fn curt_mut_state(&mut self) -> &mut TabsState {
        return self.state_vec.get_mut(self.idx).unwrap();
    }
    #[track_caller]
    pub fn curt_state(&mut self) -> &TabsState {
        return self.state_vec.get_mut(self.idx).unwrap();
    }

    /*
     * h_file
     */
    #[track_caller]
    pub fn curt_h_file(&mut self) -> &HeaderFile {
        return self.h_file_vec.get(self.idx).unwrap();
    }
    #[track_caller]
    pub fn curt_mut_h_file(&mut self) -> &mut HeaderFile {
        return self.h_file_vec.get_mut(self.idx).unwrap();
    }

    /*
     * com
     */
    #[track_caller]
    pub fn get() -> MutexGuard<'static, Tabs> {
        return TABS.get().unwrap().try_lock().unwrap();
    }
    #[track_caller]
    pub fn set_idx(idx: usize) {
        TABS.get().unwrap().try_lock().unwrap().idx = idx;
    }

    #[track_caller]
    pub fn del_file(&mut self, del_idx: usize, curt_idx: usize) {
        self.idx = curt_idx;
        self.h_file_vec.remove(del_idx);
        self.state_vec.remove(del_idx);
    }

    #[track_caller]
    pub fn get_init_file_info() -> Tabs {
        return Tabs { idx: 0, state_vec: vec![], h_file_vec: vec![] };
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tabs {
    pub idx: usize,
    pub state_vec: Vec<TabsState>,
    pub h_file_vec: Vec<HeaderFile>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TabsState {
    pub editor: TabsEditorState,
}

impl Default for TabsState {
    fn default() -> Self {
        TabsState { editor: TabsEditorState { scale: Cfg::get().general.editor.scale, row_no: Cfg::get().general.editor.row_no, window_split_type: WindowSplitType::None } }
    }
}
