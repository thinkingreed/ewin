use crate::window::ctx_menu::*;
use ewin_com::{_cfg::key::keycmd::*, def::*, model::*};
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxMenu {
    pub c_cmd: C_Cmd,
    pub ctx_menu_place_map: HashMap<TermPlace, WindowCont>,
    pub window: Window,
}

impl Default for CtxMenu {
    fn default() -> Self {
        CtxMenu { c_cmd: C_Cmd::Null, ctx_menu_place_map: HashMap::new(), window: Window::new(WindowType::CtxMenu) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    pub window_type: WindowType,
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

impl Window {
    pub fn new(window_type: WindowType) -> Self {
        let mut window = Window { ..Window::default() };
        window.window_type = window_type;
        window
    }

    pub fn clear(&mut self) {
        self.curt_cont = WindowCont::default();
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

impl Default for Window {
    fn default() -> Self {
        Window { window_type: WindowType::InputComple, curt_cont: WindowCont::default(), parent_sel_y: USIZE_UNDEFINED, parent_sel_y_org: USIZE_UNDEFINED, child_sel_y: USIZE_UNDEFINED, child_sel_y_org: USIZE_UNDEFINED, disp_sy: USIZE_UNDEFINED, disp_ey: 0, offset_y: 0, scrl_v: ScrollbarV::default() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WindowType {
    CtxMenu,
    InputComple,
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
    pub search_set: BTreeSet<String>,
}

impl Default for InputComple {
    fn default() -> Self {
        // row_words_vec: vec![RowWords::default()] is Correspondence of initial state
        InputComple { window: Window::new(WindowType::InputComple), all_words_map: BTreeMap::default(), row_words_vec: vec![RowWords::default()], search_set: BTreeSet::default() }
    }
}

impl InputComple {
    pub fn clear(&mut self) {
        self.window.clear();
        self.search_set = BTreeSet::default();
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RowWords {
    pub words: BTreeSet<String>,
}
