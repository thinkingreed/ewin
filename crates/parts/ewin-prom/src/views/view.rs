use crate::model::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for Prom {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        false
    }
    fn view(&self) -> View {
        return self.view;
    }
}
