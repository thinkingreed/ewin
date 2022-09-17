use crate::statusbar::*;
use ewin_view::{view::View, view_traits::view_trait::*};

impl ViewEvtTrait for StatusBar {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return false;
    }

    fn view(&self) -> View {
        return self.view;
    }

    fn exec_mouse_up_left(&mut self) {}
}
