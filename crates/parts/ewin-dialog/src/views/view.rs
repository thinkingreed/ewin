use crate::dialog::*;
use ewin_cfg::log::*;
use ewin_view::{view::*, view_traits::view_trait::*};

impl ViewEvtTrait for Dialog {
    fn is_tgt_mouse_move(&mut self, y: usize, x: usize) -> bool {
        if self.is_show {
            Log::debug_key("Dialog.is_tgt_mouse_move");

            for btn in self.btn_group.vec.iter_mut() {
                if btn.view.is_range(y, x) {
                    btn.view.is_on_mouse = true;
                    return true;
                }
                if btn.view.is_range_around(y, x) {
                    btn.view.is_on_mouse = false;
                    return true;
                }
                btn.view.is_on_mouse = false;
            }
            if self.close_btn.is_range(y, x) {
                self.close_btn.is_on_mouse = true;
                return true;
            } else if self.close_btn.is_range_around(y, x) {
                self.close_btn.is_on_mouse = false;
                return true;
            }
        }
        return false;
    }

    fn view(&self) -> View {
        return self.view;
    }
}
