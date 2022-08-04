use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::model::*;
use ewin_key::key::cmd::*;
use ewin_view::view_trait::view_evt_trait::*;

impl ViewEvtTrait for Prom {
    fn resize(&mut self) -> ActType {
        Log::debug_key("Prom.resize");
        match self.cmd.cmd_type {
            CmdType::Resize(_, _) => self.set_size(),
            _ => return ActType::Next,
        }
        return ActType::Draw(DParts::All);
    }
}
