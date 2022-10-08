use crate::model::*;
use ewin_cfg::log::*;
use ewin_const::models::term::*;
use ewin_ctx_menu::{ctx_menu::*, traits::traits::*};

impl ViewCtxMenuTrait for Editor {
    fn get_term_place(&mut self) -> CtxMenuPlace {
        return if self.win_mgr.curt_mut().sel.is_selected() { CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeSelected) } else { CtxMenuPlace::Editor(CtxMenuPlaceEditorCond::EditorRangeNonSelected) };
    }

    fn is_tgt_ctx_menu(&mut self, y: usize, x: usize) -> bool {
        Log::debug_key("Editor.is_tgt_ctx_menu");
        Log::debug("yyy", &y);
        Log::debug("xxx", &x);
        if self.view.is_range(y, x) && self.get_rnw_and_margin() < x {
            return true;
        }
        return false;
    }
    fn get_place_info(&mut self, _: usize, _: usize) -> CtxMenuPlaceInfo {
        return CtxMenuPlaceInfo::None;
    }
}
