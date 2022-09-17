use crate::{filebar::*, sidebar::*, tabs::tabs::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub tabs: TabsState,
    pub term: TermState,
    pub sidebar: SideBarState,
    pub filebar: FileBarState,
}

impl Default for State {
    fn default() -> Self {
        return State { term: TermState::default(), tabs: TabsState::default(), sidebar: SideBarState::default(), filebar: FileBarState::default() };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TermState {
    pub is_displayable: bool,
}

impl Default for TermState {
    fn default() -> Self {
        return TermState { is_displayable: true };
    }
}
