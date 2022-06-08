use crate::widget::ctx_menu::*;
use ewin_com::{_cfg::key::keycmd::*, model::*};
use ewin_const::def::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Widget {
    pub config: WidgetConfig,
    pub cont: WidgetCont,
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

impl Widget {
    pub fn new(config: WidgetConfig) -> Self {
        return Widget { config, ..Widget::default() };
    }

    pub fn clear(&mut self) {
        self.cont = WidgetCont::default();
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

impl Default for Widget {
    fn default() -> Self {
        Widget {
            config: WidgetConfig::default(),
            cont: WidgetCont::default(),
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
pub struct WidgetCont {
    pub height: usize,
    pub width: usize,
    pub y_area: (usize, usize),
    pub x_area: (usize, usize),
    pub cont_vec: Vec<(WidgetMenu, Option<WidgetCont>)>,
}

impl WidgetCont {
    pub fn clear(&mut self) {
        self.y_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.x_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
    }
}

impl Default for WidgetCont {
    fn default() -> Self {
        WidgetCont { height: USIZE_UNDEFINED, width: USIZE_UNDEFINED, y_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), x_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), cont_vec: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WidgetMenu {
    pub name: String,
    pub disp_name: String,
    pub is_select: bool,
    pub is_enable: bool,
    pub is_get_settings: bool,
}

impl Default for WidgetMenu {
    fn default() -> Self {
        WidgetMenu { name: String::new(), disp_name: String::new(), is_enable: true, is_select: false, is_get_settings: false }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxWidget {
    pub c_cmd: C_Cmd,
    pub ctx_menu_place_map: HashMap<TermPlace, WidgetCont>,
    pub widget: Widget,
}

impl Default for CtxWidget {
    fn default() -> Self {
        CtxWidget { c_cmd: C_Cmd::Null, ctx_menu_place_map: HashMap::new(), widget: Widget::new(WidgetConfig { widget_type: WidgetType::Widget, disp_type: WidgetDispType::Dynamic }) }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct WidgetConfig {
    pub disp_type: WidgetDispType,
    pub widget_type: WidgetType,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WidgetDispType {
    Fixed,
    Dynamic,
}
impl Default for WidgetDispType {
    fn default() -> Self {
        WidgetDispType::Fixed
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WidgetType {
    Pulldown,
    Widget,
}

impl Default for WidgetType {
    fn default() -> Self {
        WidgetType::Widget
    }
}
