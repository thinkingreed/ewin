use crate::ctx_menu::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for CtxMenu {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return true;
    }

    fn view(&self) -> View {
        // TODO
        // implement View
        return View::default();
    }
    fn exec_mouse_up_left(&mut self) {}
}
