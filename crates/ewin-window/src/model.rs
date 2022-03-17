use crate::ctx_menu::org::*;
use ewin_com::{_cfg::key::keycmd::*, def::*, model::ScrollbarV};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxMenu {
    pub c_cmd: C_Cmd,
    pub ctx_menu_place_map: HashMap<TermPlace, WindowCont>,
    pub window: Window,
}

impl Default for CtxMenu {
    fn default() -> Self {
        CtxMenu { c_cmd: C_Cmd::Null, ctx_menu_place_map: HashMap::new(), window: Window::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub e_cmd: E_Cmd,
    pub curt_cont: WindowCont,
    pub parent_sel_y: usize,
    pub parent_sel_y_org: usize,
    pub child_sel_y: usize,
    pub child_sel_y_org: usize,
    // Top of PopUp
    pub disp_sy: usize,
    // Bottom of PopUp
    pub disp_ey: usize,
    pub offset_y: usize,
    pub scrl_v: ScrollbarV,
}

impl Default for Window {
    fn default() -> Self {
        Window { e_cmd: E_Cmd::Null, curt_cont: WindowCont::default(), parent_sel_y: USIZE_UNDEFINED, parent_sel_y_org: USIZE_UNDEFINED, child_sel_y: USIZE_UNDEFINED, child_sel_y_org: USIZE_UNDEFINED, disp_sy: USIZE_UNDEFINED, disp_ey: 0, offset_y: 0, scrl_v: ScrollbarV::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// Fixed display information
pub struct WindowCont {
    pub height: usize,
    pub width: usize,
    pub y_area: (usize, usize),
    pub x_area: (usize, usize),
    pub menu_vec: Vec<(WindowMenu, Option<WindowCont>)>,
}

impl WindowCont {
    pub fn clear(&mut self) {
        self.y_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.x_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
    }
}

impl Default for WindowCont {
    fn default() -> Self {
        WindowCont { height: USIZE_UNDEFINED, width: USIZE_UNDEFINED, y_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), x_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), menu_vec: vec![] }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct WindowMenu {
    pub name: String,
    pub name_disp: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputComple {
    pub window: Window,
    pub all_words_map: BTreeMap<String, BTreeSet<usize>>,
    pub row_words_vec: Vec<RowWords>,
}

impl Default for InputComple {
    fn default() -> Self {
        // row_words_vec: vec![RowWords::default()] is Correspondence of initial state
        InputComple { window: Window::default(), all_words_map: BTreeMap::default(), row_words_vec: vec![RowWords::default()] }
    }
}

impl InputComple {
    pub fn clear(&mut self) {
        self.window = Window::default();
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RowWords {
    pub words: BTreeSet<String>,
}
