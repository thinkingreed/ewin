use crate::{core::*, model::*};
use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use indexmap::*;
use std::ops::*;

impl Pulldown {
    pub const MARGIN: usize = 2;
    pub const MAX_HEIGHT: usize = 10;
    pub const ARROW_STR: &'static str = "▽";

    pub fn set_disp_name(&mut self, menu_set: IndexSet<String>) {
        Log::debug_key("set_disp_name");

        //
        self.widget.set_disp_name_single_widget(menu_set.into_iter(), Some(2));
        self.set_sel_name();
    }

    pub fn set_sel_name(&mut self) {
        let disp_str = &self.widget.cont.cont_vec[self.sel_idx].0.disp_name;
        // -3 is Extra
        self.sel_str = format!(" {} {} ", disp_str.trim(), Pulldown::ARROW_STR);
    }

    pub fn get_sel_name(&mut self) -> String {
        let s = self.sel_str.replace(Pulldown::ARROW_STR, "").trim().to_string();
        return if s == Lang::get().none { "".to_string() } else { s };
    }
}

impl WidgetTrait for Pulldown {
    fn clear(&mut self) {
        self.widget.clear();
    }

    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("InputComple.draw");
        // calc offset
        self.widget.calc_scrlbar_v();
        self.widget.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pulldown {
    pub is_disp: bool,
    pub sel_idx: usize,
    pub sel_str: String,
    pub x_range: Range<usize>,
    pub widget: Widget,
}

impl Pulldown {
    pub fn new() -> Self {
        Pulldown { sel_str: String::new(), x_range: Range::default(), widget: Widget::new(WidgetConfig { widget_type: WidgetType::Pulldown, disp_type: WidgetDispType::Fixed }), ..Pulldown::default() }
    }
}
impl Default for Pulldown {
    fn default() -> Self {
        Pulldown { is_disp: false, sel_idx: 0, sel_str: String::new(), x_range: Range::default(), widget: Widget::new(WidgetConfig { ..WidgetConfig::default() }) }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Area {
    y: usize,
    x_range: Range<usize>,
}
