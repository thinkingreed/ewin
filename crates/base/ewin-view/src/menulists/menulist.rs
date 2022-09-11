use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{def::*, models::model::*};
use ewin_state::term::*;
use ewin_utils::str_edit::*;

use crate::scrollbar_v::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuList {
    pub config: MenuListConfig,
    pub cont: MenuListCont,
    pub parent_sel_y: usize,
    pub parent_sel_y_org: usize,
    pub child_sel_y: usize,
    pub child_sel_y_org: usize,
    // Highest position of Widget
    pub disp_sy: usize,
    // Lowest position of Widget
    pub disp_ey: usize,
    pub disp_sy_org: usize,
    pub disp_ey_org: usize,
    pub offset_y: usize,
    pub scrl_v: ScrollbarV,
}

impl MenuList {
    pub fn new(config: MenuListConfig) -> Self {
        return MenuList { config, ..MenuList::default() };
    }

    pub fn clear(&mut self) {
        self.cont = MenuListCont::default();
        self.parent_sel_y = USIZE_UNDEFINED;
        self.parent_sel_y_org = USIZE_UNDEFINED;
        self.child_sel_y = USIZE_UNDEFINED;
        self.child_sel_y_org = USIZE_UNDEFINED;
        self.disp_sy = USIZE_UNDEFINED;
        self.disp_ey = 0;
        self.offset_y = 0;
        self.scrl_v = ScrollbarV::default();
    }
}

impl Default for MenuList {
    fn default() -> Self {
        MenuList {
            config: MenuListConfig::default(),
            cont: MenuListCont::default(),
            parent_sel_y: USIZE_UNDEFINED,
            parent_sel_y_org: USIZE_UNDEFINED,
            child_sel_y: USIZE_UNDEFINED,
            child_sel_y_org: USIZE_UNDEFINED,
            disp_sy: USIZE_UNDEFINED,
            disp_ey: 0,
            disp_sy_org: USIZE_UNDEFINED,
            disp_ey_org: 0,
            offset_y: 0,
            scrl_v: ScrollbarV::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// Fixed display information
pub struct MenuListCont {
    pub height: usize,
    pub width: usize,
    pub y_area: (usize, usize),
    pub x_area: (usize, usize),
    pub cont_vec: Vec<(MenuListMenu, Option<MenuListCont>)>,
}

impl MenuListCont {
    pub fn clear(&mut self) {
        self.y_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.x_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
    }
}

impl Default for MenuListCont {
    fn default() -> Self {
        MenuListCont { height: USIZE_UNDEFINED, width: USIZE_UNDEFINED, y_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), x_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), cont_vec: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MenuListMenu {
    pub name: String,
    pub disp_name: String,
    pub is_select: bool,
    pub is_enable: bool,
}

impl Default for MenuListMenu {
    fn default() -> Self {
        MenuListMenu { name: String::new(), disp_name: String::new(), is_enable: true, is_select: false }
    }
}

impl MenuListMenu {
    pub fn new(menu_str: &str, menunm_max_len: usize) -> Self {
        Self { name: cut_str(menu_str, menunm_max_len, false, true), ..MenuListMenu::default() }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MenuListConfig {
    pub disp_type: MenuListDispType,
    pub menulist_type: MenuListType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuListDispType {
    Fixed,
    Dynamic,
}
impl Default for MenuListDispType {
    fn default() -> Self {
        MenuListDispType::Fixed
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuListType {
    Pulldown,
    MenuList,
}

impl Default for MenuListType {
    fn default() -> Self {
        MenuListType::MenuList
    }
}
