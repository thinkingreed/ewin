use ewin_cfg::model::default::{CfgEditorRowNo, CfgEditorScale};
use ewin_key::model::*;

impl TabsEditorState {
    pub fn toggle_state(&mut self, state: TabsEditerStateType) {
        match state {
            TabsEditerStateType::Scale => self.scale.is_enable = !self.scale.is_enable,
            TabsEditerStateType::RowNo => self.row_no.is_enable = !self.row_no.is_enable,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TabsEditorState {
    pub scale: CfgEditorScale,
    pub row_no: CfgEditorRowNo,
    pub window_split_type: WindowSplitType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsEditerStateType {
    Scale,
    RowNo,
}
