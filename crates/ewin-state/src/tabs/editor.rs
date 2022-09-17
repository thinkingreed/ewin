use ewin_cfg::model::general::default::*;
use ewin_const::models::model::*;
use ewin_key::model::*;

impl EditorState {
    pub fn toggle_state(&mut self, state: TabsEditerStateType) {
        match state {
            TabsEditerStateType::Scale => self.scale.is_enable = !self.scale.is_enable,
            TabsEditerStateType::RowNo => self.row_no.is_enable = !self.row_no.is_enable,
            TabsEditerStateType::KeyMacroRecord => self.key_macro.is_record = !self.key_macro.is_record,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EditorState {
    pub is_read_only: bool,
    pub is_changed: bool,
    // pub is_changed_org: bool,
    pub is_dragging: bool,
    pub mouse: Mouse,
    pub scale: CfgEditorScale,
    pub row_no: CfgEditorRowNo,
    pub window_split_type: WindowSplitType,
    pub key_macro: KeyMacroState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabsEditerStateType {
    Scale,
    RowNo,
    KeyMacroRecord,
}
