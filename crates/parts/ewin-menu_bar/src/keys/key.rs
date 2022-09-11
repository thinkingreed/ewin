use ewin_cfg::log::*;
use ewin_key::{key::keys::*, key_traits::key_trait::*};

use crate::menubar::*;

impl KeyTrait for MenuBar {
    fn is_allow_key(&mut self, keys: Keys) -> bool {
        Log::debug_key("MenuBar.is_allow_key");

        Log::debug("self.menulist.is_show", &self.menulist.is_show);

        let rtn = match keys {
            Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) if self.menulist.is_show => true,
            Keys::MouseMove(y, _) if y == (self.view.y as u16) || self.menulist.is_show => true,
            Keys::MouseDownLeft(y, x) => {
                if self.is_menubar_displayed_area(y as usize, x as usize).0 {
                    true
                } else {
                    self.menulist.curt.is_mouse_within_area(y as usize, x as usize)
                }
            }
            _ => false,
        };
        return rtn;
    }
}
