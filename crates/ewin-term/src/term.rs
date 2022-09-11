use ewin_const::models::term::*;
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::term::*;
use ewin_tabs::tabs::*;

impl Term {
    pub fn new() -> Self {
        Term { ..Term::default() }
    }
}

impl Default for Term {
    fn default() -> Self {
        Term { cmd: Cmd::default(), keys: Keys::Null, keys_org: Keys::Null, place: Place::Tabs, place_org: Place::Tabs, tabs: Tabs::default(), state: State::default() }
    }
}

#[derive(Debug, Clone)]
// Terminal
pub struct Term {
    pub cmd: Cmd,
    pub keys: Keys,
    pub keys_org: Keys,
    pub place: Place,
    pub place_org: Place,
    pub tabs: Tabs,
    pub state: State,
}
