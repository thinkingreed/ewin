use crate::model::*;
use ewin_state::term::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for Editor {
    fn is_tgt_mouse_move(&mut self, _: usize, _: usize) -> bool {
        false
    }
    fn view(&self) -> View {
        return self.view;
    }

    fn exec_mouse_up_left(&mut self) {
        State::get().curt_mut_state().editor.is_dragging = false;

        for vec in self.win_mgr.win_list.iter_mut() {
            for win in vec {
                win.scrl_v.is_enable = false;
                win.scrl_h.is_enable = false;
            }
        }
    }
}
