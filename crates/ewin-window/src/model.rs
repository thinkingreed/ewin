use crate::ctx_menu::org::TermPlace;
use ewin_com::{_cfg::key::keycmd::C_Cmd, def::*};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxMenu {
    pub c_cmd: C_Cmd,
    pub ctx_menu_place_map: HashMap<TermPlace, PopUpCont>,
    pub popup: PopUp,
}

impl Default for CtxMenu {
    fn default() -> Self {
        CtxMenu { c_cmd: C_Cmd::Null, ctx_menu_place_map: HashMap::new(), popup: PopUp::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopUp {
    pub curt_cont: PopUpCont,
    pub parent_sel_y: usize,
    pub parent_sel_y_cache: usize,
    pub child_sel_y: usize,
    pub child_sel_y_cache: usize,
    pub disp_sy: usize,
    pub disp_ey: usize,
}

impl Default for PopUp {
    fn default() -> Self {
        PopUp { curt_cont: PopUpCont::default(), parent_sel_y: USIZE_UNDEFINED, parent_sel_y_cache: USIZE_UNDEFINED, child_sel_y: USIZE_UNDEFINED, child_sel_y_cache: USIZE_UNDEFINED, disp_sy: USIZE_UNDEFINED, disp_ey: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// Fixed display information
pub struct PopUpCont {
    pub height: usize,
    pub width: usize,
    pub y_area: (usize, usize),
    pub x_area: (usize, usize),
    pub menu_vec: Vec<(PopUpMenu, Option<PopUpCont>)>,
}

impl PopUpCont {
    pub fn clear(&mut self) {
        self.y_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.x_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
    }
}

impl Default for PopUpCont {
    fn default() -> Self {
        PopUpCont { height: USIZE_UNDEFINED, width: USIZE_UNDEFINED, y_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), x_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), menu_vec: vec![] }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct PopUpMenu {
    pub name: String,
    pub name_disp: String,
}
