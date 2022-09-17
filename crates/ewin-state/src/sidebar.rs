#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SideBarState {
    pub is_show: bool,

    pub resize: SideBarResizeState,
}

impl Default for SideBarState {
    fn default() -> Self {
        return SideBarState { resize: SideBarResizeState::default(), is_show: false };
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum SideBarResizeState {
    #[default]
    None,
    Start,
    Resizing,
}
