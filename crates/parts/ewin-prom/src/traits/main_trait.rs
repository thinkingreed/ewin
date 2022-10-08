use crate::model::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::log::*;
use ewin_key::key::cmd::*;

pub trait PromTrait: Any + DynClone + Send + 'static + std::fmt::Debug {
    fn as_base(&self) -> &PromBase;
    fn as_mut_base(&mut self) -> &mut PromBase;

    fn clear_sels(&mut self) {
        Log::debug("self.cmd", &self.as_base().cmd);
        match self.as_base().cmd.cmd_type {
            CmdType::Copy | CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorUp | CmdType::CursorDown | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::NextContent | CmdType::BackContent | CmdType::MouseDownLeft(_, _) => {
                self.as_mut_base().clear_sels();
            }
            _ => {}
        };
    }
}

downcast!(dyn PromTrait);
dyn_clone::clone_trait_object!(PromTrait);
