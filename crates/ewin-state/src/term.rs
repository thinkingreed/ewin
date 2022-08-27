use crate::tabs::state::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct State {
    pub tabs: TabsStates,
    pub term: TermState,
}

impl Default for State {
    fn default() -> Self {
        return State { term: TermState::default(), tabs: TabsStates::default() };
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
