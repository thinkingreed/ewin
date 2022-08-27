use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use ewin_view::menulists::{core::*, menulist::*};
use indexmap::*;
use std::ops::*;

impl Pulldown {
    pub const MARGIN: usize = 2;
    pub const MAX_HEIGHT: usize = 10;
    pub const ARROW_STR: &'static str = "▽";

    pub fn set_disp_name(&mut self, menu_set: IndexSet<String>) {
        Log::debug_key("set_disp_name");

        //
        self.menulist.set_disp_name_single_menulist(menu_set.into_iter(), Some(2));
        self.set_sel_name();
    }

    pub fn set_sel_name(&mut self) {
        let disp_str = &self.menulist.cont.cont_vec[self.sel_idx].0.disp_name;
        // -3 is Extra
        self.sel_str = format!(" {} {} ", disp_str.trim(), Pulldown::ARROW_STR);
    }

    pub fn get_sel_name(&mut self) -> String {
        let s = self.sel_str.replace(Pulldown::ARROW_STR, "").trim().to_string();
        return if s == Lang::get().none { "".to_string() } else { s };
    }
}

impl MenuListTrait for Pulldown {
    fn clear(&mut self) {
        self.menulist.clear();
    }

    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("InputComple.draw");
        // calc offset
        self.menulist.calc_scrlbar_v();
        self.menulist.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pulldown {
    pub is_show: bool,
    pub sel_idx: usize,
    pub sel_str: String,
    pub x_range: Range<usize>,
    pub menulist: MenuList,
}

impl Pulldown {
    pub fn new() -> Self {
        Pulldown { sel_str: String::new(), x_range: Range::default(), menulist: MenuList::new(MenuListConfig { menulist_type: MenuListType::Pulldown, disp_type: MenuListDispType::Fixed }), ..Pulldown::default() }
    }
}
impl Default for Pulldown {
    fn default() -> Self {
        Pulldown { is_show: false, sel_idx: 0, sel_str: String::new(), x_range: Range::default(), menulist: MenuList::new(MenuListConfig { ..MenuListConfig::default() }) }
    }
}
