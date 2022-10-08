use crate::sidebar::*;
use ewin_state::{sidebar::*, term::*};
use ewin_view::traits::view_evt::*;

impl ViewEvtTrait for SideBar {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        return true;
    }

    fn exec_mouse_up_left(&mut self) {
        self.cont.disable_scrl_v();
        // TODO disable scrl_h
        State::get().sidebar.resize = SideBarResizeState::None;
    }
}
