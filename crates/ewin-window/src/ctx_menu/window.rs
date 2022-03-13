use crate::{model::*, window::*};
use ewin_com::{colors::*, log::*};

impl WindowTrait for CtxMenu {
    fn clear(&mut self) {
        self.window.clear();
        for (_, parent_cont) in self.ctx_menu_place_map.iter_mut() {
            parent_cont.clear();
            for (_, child_cont_option) in parent_cont.menu_vec.iter_mut() {
                if let Some(child_cont) = child_cont_option {
                    child_cont.clear();
                }
            }
        }
    }
    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenu.draw");
        Log::debug("CtxMenu", &self);

        self.window.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}
