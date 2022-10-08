use crate::tooltip::*;
use ewin_const::models::event::*;
use ewin_key::key::cmd::*;

impl ToolTip {
    pub fn ctrl_tooltip(_: &CmdType) -> ActType {
        return ActType::None;
    }
}
