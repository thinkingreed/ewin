use crate::ctx_menu::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, evt::*};
use ewin_view::view_traits::view_trait::*;

impl ViewEvtTrait for CtxMenu {
    fn resize(&mut self) -> ActType {
        Log::debug_key("Editor.resize");
        self.is_show = false;
        return ActType::Draw(DParts::All);
    }
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return true;
    }
}
