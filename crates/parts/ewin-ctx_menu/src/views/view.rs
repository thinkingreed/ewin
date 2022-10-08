use crate::ctx_menu::*;
use ewin_view::traits::view_evt::*;

impl ViewEvtTrait for CtxMenu {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return true;
    }

    fn exec_mouse_up_left(&mut self) {}
}
