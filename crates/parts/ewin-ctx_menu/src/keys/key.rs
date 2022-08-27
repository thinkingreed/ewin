use crate::ctx_menu::*;
use ewin_cfg::log::*;
use ewin_key::{key::keys::*, key_traits::key_trait::*};

impl KeyTrait for CtxMenu {
    fn is_allow_key(&mut self, keys: Keys) -> bool {
        Log::debug_key("CtxMenu.is_allow_key");
        return match keys {
            Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) => true,
            Keys::MouseMove(_, _) => true,
            Keys::MouseDownLeft(y, x) => self.menulist.is_mouse_within_area(y as usize, x as usize),
            // Ctx_menu is processed by each place
            Keys::MouseDownRight(_, _) | Keys::MouseDragRight(_, _) => false,
            _ => true,
        };
    }
}
