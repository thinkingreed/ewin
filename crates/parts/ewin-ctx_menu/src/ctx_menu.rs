use ewin_cfg::{colors::*, log::*};
use ewin_const::{def::USIZE_UNDEFINED, models::term::*};
use ewin_key::{key::keys::*, key_traits::key_trait::*};
use ewin_view::menulists::{core::*, menulist::*};
use std::collections::HashMap;
use tokio::sync::MutexGuard;

use crate::global::*;

impl CtxMenu {
    pub fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenu.draw");
        Log::debug("self.is_show", &self.is_show);

        if self.is_show {
            self.menulist.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
        }
    }

    pub fn is_check_clear(&mut self, keys: Keys) {
        if self.is_show && !self.is_allow_key(keys) {
            self.clear();
        }
    }

    #[track_caller]
    pub fn get() -> MutexGuard<'static, CtxMenu> {
        return CTX_MENU.get().unwrap().try_lock().unwrap();
    }
}

impl MenuListTrait for CtxMenu {
    fn clear(&mut self) {
        self.is_show = false;
        self.menulist.clear();
        for (_, parent_cont) in self.place_menulist_map.iter_mut() {
            parent_cont.clear();
            for (_, child_cont_option) in parent_cont.cont_vec.iter_mut() {
                if let Some(child_cont) = child_cont_option {
                    child_cont.clear();
                }
            }
        }
    }
    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenu.draw");
        Log::debug("CtxMenu", &self);

        self.menulist.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxMenu {
    pub is_show: bool,
    pub place_info: CtxMenuPlaceInfo,
    pub place: CtxMenuPlace,
    pub place_menulist_map: HashMap<CtxMenuPlace, MenuListCont>,
    pub menulist: MenuList,
}

impl Default for CtxMenu {
    fn default() -> Self {
        CtxMenu { is_show: false, place_info: CtxMenuPlaceInfo::FileBar(CtxMenuFileBar { tgt_idx: USIZE_UNDEFINED }), place: CtxMenuPlace::None, place_menulist_map: HashMap::new(), menulist: MenuList::new(MenuListConfig { menulist_type: MenuListType::MenuList, disp_type: MenuListDispType::Dynamic }) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtxMenuPlaceInfo {
    FileBar(CtxMenuFileBar),
    None,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CtxMenuFileBar {
    pub tgt_idx: usize,
}
