use crate::model::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::log::*;
use ewin_com::_cfg::key::keycmd::*;

pub trait PromPluginTrait: Any + DynClone + Send + 'static + std::fmt::Debug {
    fn as_base(&self) -> &PromPluginBase;
    fn as_mut_base(&mut self) -> &mut PromPluginBase;
    fn get_disp_all_row_num(&mut self) -> usize {
        return self.as_mut_base().get_disp_all_row_num();
    }
    fn clear_sels(&mut self) {
        Log::debug("self.p_cmd", &self.as_base().p_cmd);
        match self.as_base().p_cmd {
            P_Cmd::Copy | P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorUp | P_Cmd::CursorDown | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::NextContent | P_Cmd::BackContent | P_Cmd::MouseDownLeft(_, _) => {
                self.as_mut_base().clear_sels();
            }
            _ => {}
        };
    }
}

downcast!(dyn PromPluginTrait);
dyn_clone::clone_trait_object!(PromPluginTrait);
