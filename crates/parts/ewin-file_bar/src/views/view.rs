use crate::filebar::*;
use ewin_state::term::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for FileBar {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return false;
    }

    fn view(&self) -> View {
        return self.view;
    }

    fn exec_mouse_up_left(&mut self) {
        State::get().filebar.is_dragging = false;
    }
}
