use crate::tabs::tabs::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub tabs: TabsState,
    pub term: TermState,
}

impl Default for State {
    fn default() -> Self {
        return State { term: TermState::default(), tabs: TabsState::default() };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermState {
    pub is_displayable: bool,
    pub is_sidebar_show: bool,
}

impl Default for TermState {
    fn default() -> Self {
        return TermState { is_displayable: true, is_sidebar_show: false };
    }
}
