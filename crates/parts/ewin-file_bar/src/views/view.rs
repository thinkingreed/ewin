use crate::filebar::*;
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, evt::*};
use ewin_view::view_traits::view_trait::*;

impl ViewEvtTrait for FileBar {
    fn resize(&mut self) -> ActType {
        Log::debug_key("FileBar.resize");
        //  self.clear();
        return ActType::Draw(DParts::All);
    }
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return false;
    }
}
