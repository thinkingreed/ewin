use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, evt::*};
use ewin_key::key::cmd::*;
use ewin_view::view_traits::view_trait::*;

impl ViewEvtTrait for Prom {
    fn resize(&mut self) -> ActType {
        Log::debug_key("Prom.resize");
        match self.cmd.cmd_type {
            CmdType::Resize(_, _) => self.set_size(),
            _ => return ActType::Next,
        }
        return ActType::Draw(DParts::All);
    }

    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        false
    }
}
