use crate::sidebar::*;
use ewin_state::{sidebar::*, term::*};
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for SideBar {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return true;
    }
    fn view(&self) -> View {
        return self.cont.as_base().view;
    }

    fn exec_mouse_up_left(&mut self) {
        self.scrl_v.is_enable = false;
        State::get().sidebar.resize = SideBarResizeState::None;
    }
}
