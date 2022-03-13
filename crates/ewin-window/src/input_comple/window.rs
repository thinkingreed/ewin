use crate::{model::*, window::*};
use ewin_com::{colors::Colors, log::*};

impl WindowTrait for InputComple {
    fn clear(&mut self) {
        self.window.clear();
    }

    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("InputComple.draw");
        Log::debug("InputComple", &self);
        self.window.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}
