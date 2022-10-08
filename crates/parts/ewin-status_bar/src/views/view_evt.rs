use crate::statusbar::*;
use ewin_view::traits::view_evt::*;

impl ViewEvtTrait for StatusBar {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return false;
    }

    fn exec_mouse_up_left(&mut self) {}
}
