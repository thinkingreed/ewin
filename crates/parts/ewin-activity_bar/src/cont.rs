use ewin_view::view::*;
use serde::Deserialize;

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ActivityCont {
    pub base: ActivityContBase,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Hash)]
pub struct ActivityContBase {
    #[serde(skip_deserializing)]
    pub view: View,
    pub icon: String,
    #[serde(skip_deserializing)]
    pub is_select: bool,
}
