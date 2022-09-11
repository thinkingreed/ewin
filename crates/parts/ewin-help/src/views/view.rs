use crate::help::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for Help {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return false;
    }
    fn view(&self) -> View {
        return self.view;
    }
}
