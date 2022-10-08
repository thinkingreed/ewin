#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivityBarState {
    pub is_show: bool,
}

impl Default for ActivityBarState {
    fn default() -> Self {
        return ActivityBarState { is_show: false };
    }
}
