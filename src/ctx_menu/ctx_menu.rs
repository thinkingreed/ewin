use crate::{
    colors::*,
    def::*,
    global::{LANG, LANG_MAP},
    log::Log,
    model::ConvType,
    terminal::Terminal,
    util::*,
};
use crossterm::{cursor::MoveTo, terminal::size};
use std::{
    cmp::{max, min},
    collections::HashMap,
    hash::Hash,
    slice::Iter,
    str::FromStr,
};

impl CtxMenuGroup {
    // How far the display is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn click_ctx_menu(term: &mut Terminal) {
        if let Some(ctx_menu) = term.ctx_menu_group.curt_child_menu() {
            CtxMenuGroup::exec_func(term, &ctx_menu.name);
        } else {
            if let Some((parent_ctx_menu, _)) = term.ctx_menu_group.curt_parent_menu() {
                CtxMenuGroup::exec_func(term, &parent_ctx_menu.name);
            }
        }
    }
    pub fn exec_func(term: &mut Terminal, name: &str) {
        Log::debug("exec_func", &name);
        Log::debug("LANG.to_uppercase", &LANG.to_uppercase);
        Log::debug("LANG_MAP", &LANG_MAP[name]);

        match &LANG_MAP[name] {
            s if s == &LANG.to_uppercase || s == &LANG.to_lowercase || s == &LANG.to_full_width || s == &LANG.to_half_width || s == &LANG.to_space || s == &LANG.to_tab => term.curt().editor.convert(ConvType::from_str(&LANG_MAP[name])),
            _ => {}
        };

        term.clear_ctx_menu();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenuGroup.draw");

        for (idx, (parent_menu, child_cont_option)) in self.curt_cont.menu_vec.iter().enumerate() {
            let name = if idx == self.parent_sel_y {
                format!("{}{}{}", Colors::get_ctx_menu_fg_bg_sel(), parent_menu.name_disp, Colors::get_default_fg_bg())
            } else {
                format!("{}{}{}", Colors::get_ctx_menu_fg_bg_non_sel(), parent_menu.name_disp, Colors::get_default_fg_bg())
            };

            str_vec.push(format!("{}{}", MoveTo((self.curt_cont.x_area.0) as u16, (self.curt_cont.y_area.0 + idx) as u16), name));
            if idx == self.parent_sel_y {
                if let Some(child_cont) = child_cont_option {
                    for (idx, (child_menu, _)) in child_cont.menu_vec.iter().enumerate() {
                        let name = if idx == self.child_sel_y { format!("{}{}{}", Colors::get_ctx_menu_fg_bg_sel(), child_menu.name_disp, Colors::get_default_fg_bg()) } else { format!("{}{}{}", Colors::get_ctx_menu_fg_bg_non_sel(), child_menu.name_disp, Colors::get_default_fg_bg()) };
                        str_vec.push(format!("{}{}", MoveTo((child_cont.x_area.0) as u16, (child_cont.y_area.0 + idx) as u16), name));
                    }
                }
            }
        }
    }

    pub fn set_curt_term_place(term: &mut Terminal, y: usize) {
        if term.hbar.disp_row_posi == y {
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&TermPlace::HeaderBar].clone();
        } else if term.curt().editor.disp_row_posi <= y && y <= term.curt().editor.disp_row_posi + term.curt().editor.disp_row_num {
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&TermPlace::Editor].clone();
        }
    }

    pub fn set_parent_disp_area(&mut self, y: usize, x: usize) {
        Log::debug_key("set_parent_disp_area");

        Log::debug("self.curt_cont.height", &self.curt_cont.height);

        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);
        // rows
        let (rows_sy, rows_ey) = if y + CtxMenuGroup::EXTRA_FROM_CUR_Y + self.curt_cont.height > rows {
            let base_y = y - CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_y - self.curt_cont.height + 1, base_y)
        } else {
            let base_y = y + CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.curt_cont.height - 1)
        };
        // cols
        let (cols_sx, cols_ex) = if x + CtxMenuGroup::EXTRA_FROM_CUR_X + self.curt_cont.width > cols {
            let base_x = x + CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_x - self.curt_cont.width + 1, base_x)
        } else {
            let base_x = x + CtxMenuGroup::EXTRA_FROM_CUR_X;
            (base_x, base_x + self.curt_cont.width - 1)
        };
        self.curt_cont.y_area = (rows_sy, rows_ey);

        Log::debug("self.curt_cont.y_area", &self.curt_cont.y_area);
        self.curt_cont.x_area = (cols_sx, cols_ex);
        self.disp_sy = min(self.disp_sy, rows_sy);
        self.disp_ey = max(self.disp_ey, rows_ey);
    }

    pub fn init(&mut self) {
        let ctx_menu_str = r#"
         {
            "editor": [
                {"convert": ["to_uppercase", "to_lowercase", "to_full_width", "to_half_width", "to_space", "to_tab"]},
                {"format": ["json", "xml"]}
            ],
            "headerBar": [
                {"format": ["json", "xml"]}
            ]
        }
          "#;

        let mut map: HashMap<String, Vec<HashMap<String, Vec<String>>>> = serde_json::from_str(ctx_menu_str).unwrap();

        self.set_internal_struct(&mut map);
        self.set_disp_name();
    }

    pub fn set_internal_struct(&mut self, map: &mut HashMap<String, Vec<HashMap<String, Vec<String>>>>) {
        let mut ctx_menu_map = HashMap::new();
        for (place_str, vec) in map {
            let mut parent_cont = CtxMenuCont { ..CtxMenuCont::default() };
            let mut parent_menu_vec: Vec<(CtxMenu, Option<CtxMenuCont>)> = vec![];
            for map in vec {
                for (parent_name, c_vec) in map.iter_mut() {
                    let parent_ctx_menu = CtxMenu { name: parent_name.clone(), ..CtxMenu::default() };

                    let mut child_cont = CtxMenuCont { ..CtxMenuCont::default() };
                    let mut child_menu_vec: Vec<(CtxMenu, Option<CtxMenuCont>)> = vec![];
                    for child_name in c_vec {
                        let child_ctx_menu = CtxMenu { name: child_name.clone(), ..CtxMenu::default() };
                        child_menu_vec.push((child_ctx_menu, None));
                    }
                    child_cont.menu_vec = child_menu_vec;
                    parent_menu_vec.push((parent_ctx_menu, if child_cont.menu_vec.is_empty() { None } else { Some(child_cont) }));
                }
                parent_cont.menu_vec = parent_menu_vec.clone();
            }
            Log::debug("place_str", &place_str);
            ctx_menu_map.insert(TermPlace::from_str(place_str).unwrap(), parent_cont);
        }
        self.ctx_menu_place_map = ctx_menu_map;
    }

    pub fn set_disp_name(&mut self) {
        let mut parent_max_len = 0;
        let mut child_max_len_vec: Vec<usize> = vec![];
        let exist_child_mark = " >";

        for term_place in TermPlace::iter() {
            // max_len name max length
            for (idx, (parent_menu, child_cont_option)) in self.ctx_menu_place_map[&term_place].menu_vec.iter().enumerate() {
                let parent_name_len = get_str_width(&LANG_MAP[&parent_menu.name]);
                parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };
                if let Some(child_cont) = child_cont_option {
                    parent_max_len += exist_child_mark.len();
                    let mut child_max_len = 0;
                    for (child_menu, _) in child_cont.menu_vec.iter() {
                        let child_name_len = get_str_width(&LANG_MAP[&child_menu.name]);
                        child_max_len = if child_name_len > child_max_len { child_name_len } else { child_max_len };
                    }
                    child_max_len_vec.insert(idx, child_max_len);
                }
            }
            // set name_disp
            for (idx, (parent_menu, child_menu_cont_option)) in self.ctx_menu_place_map.get_mut(&term_place).unwrap().menu_vec.iter_mut().enumerate() {
                let space;
                let perent_str = &LANG_MAP[&parent_menu.name];
                if let Some(child_cont) = child_menu_cont_option {
                    space = parent_max_len - (get_str_width(perent_str) + exist_child_mark.len());
                    parent_menu.name_disp = format!(" {}{}{} ", perent_str, " ".repeat(space), exist_child_mark);
                    for (child_menu, _) in child_cont.menu_vec.iter_mut() {
                        let child_str = &LANG_MAP[&child_menu.name];
                        let diff = child_max_len_vec[idx] - get_str_width(&child_str);
                        child_menu.name_disp = format!(" {}{} ", child_str, " ".repeat(diff));
                    }
                    child_cont.height = child_cont.menu_vec.len();
                    child_cont.width = child_max_len_vec[idx];
                } else {
                    space = parent_max_len - get_str_width(perent_str);
                    parent_menu.name_disp = format!(" {}{} ", perent_str, " ".repeat(space));
                }
            }

            self.ctx_menu_place_map.get_mut(&term_place).unwrap().height = self.ctx_menu_place_map[&term_place].menu_vec.len();
            // +2 is Extra space
            self.ctx_menu_place_map.get_mut(&term_place).unwrap().width = parent_max_len + 2;
        }
    }
    pub fn clear(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
        self.parent_sel_y_cache = USIZE_UNDEFINED;
        self.child_sel_y = USIZE_UNDEFINED;
        self.selected_cont = CtxMenuContType::None;
        self.disp_sy = 0;
        self.disp_ey = 0;
        for (_, parent_cont) in self.ctx_menu_place_map.iter_mut() {
            parent_cont.clear();
            for (_, child_cont_option) in parent_cont.menu_vec.iter_mut() {
                if let Some(child_cont) = child_cont_option {
                    child_cont.clear();
                }
            }
        }
    }
    pub fn is_mouse_within_range(&mut self, y: usize, x: usize) -> bool {
        Log::debug_key("is_mouse_within_range");
        if self.curt_cont.y_area.0 <= y && y <= self.curt_cont.y_area.1 && self.curt_cont.x_area.0 <= x && x <= self.curt_cont.x_area.1 {
            self.selected_cont = CtxMenuContType::Parent;
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y].1 {
                if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
                    self.selected_cont = CtxMenuContType::Child;
                    return true;
                };
            }
        }
        self.selected_cont = CtxMenuContType::None;
        return false;
    }
    pub fn ctrl_mouse_move(&mut self, y: usize, x: usize) {
        if self.curt_cont.y_area.0 <= y && y <= self.curt_cont.y_area.1 && self.curt_cont.x_area.0 <= x && x <= self.curt_cont.x_area.1 {
            self.parent_sel_y_cache = self.parent_sel_y;
            self.parent_sel_y = y - self.curt_cont.y_area.0;
        }
        if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y].1 {
            let child_base_y = self.curt_cont.y_area.0 + self.parent_sel_y;

            let (cols, rows) = size().unwrap();
            let (cols, rows) = (cols as usize, rows as usize);
            // rows
            let (rows_sy, rows_ey) = if y + child_cont.height > rows { (y - child_cont.height + 1, y) } else { (child_base_y, child_base_y + child_cont.height - 1) };

            child_cont.y_area = (rows_sy, rows_ey);

            Log::debug("child_cont.y_area", &child_cont.y_area);

            self.disp_sy = min(self.disp_sy, rows_sy);
            self.disp_ey = max(self.disp_ey, rows_ey);
            child_cont.x_area = (self.curt_cont.x_area.1 + 1, self.curt_cont.x_area.1 + 1 + child_cont.width);

            if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
                self.child_sel_y_cache = self.child_sel_y;
                self.child_sel_y = y - child_cont.y_area.0;
            } else {
                self.child_sel_y = USIZE_UNDEFINED;
            }
        }
    }

    pub fn get_draw_range(&mut self, disp_row_num: usize) -> Option<(usize, usize)> {
        let sy = self.disp_sy;
        let ey = self.disp_ey;

        if self.parent_sel_y != USIZE_UNDEFINED {
            if (self.selected_cont == CtxMenuContType::Parent && self.parent_sel_y != USIZE_UNDEFINED && self.parent_sel_y == self.parent_sel_y_cache) || (self.selected_cont == CtxMenuContType::Child && self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y == self.child_sel_y_cache) {
                return None;
            }
        }
        /*
        let sy = self.curt_cont.y_area.0 - disp_row_num;
        let mut ey = self.curt_cont.y_area.1;
        if self.parent_sel_y != USIZE_UNDEFINED {
            if (self.selected_cont == CtxMenuContType::Parent && self.parent_sel_y != USIZE_UNDEFINED && self.parent_sel_y == self.parent_sel_y_cache) || (self.selected_cont == CtxMenuContType::Child && self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y == self.child_sel_y_cache) {
                return None;
            }
            if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y].1 {
                ey = max(ey, child_cont.y_area.1);
                if self.parent_sel_y_cache != USIZE_UNDEFINED {
                    if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y_cache].1 {
                        ey = max(ey, child_cont.y_area.1);
                    }
                }
            }
        }
         */
        return Some((min(sy, ey), max(sy, ey)));
    }

    pub fn curt_parent_menu(&self) -> Option<(CtxMenu, Option<CtxMenuCont>)> {
        if let Some((ctx_menu, child_cont_option)) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            return Some((ctx_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }

    pub fn curt_child_menu(&mut self) -> Option<CtxMenu> {
        let child_sel_y = self.child_sel_y;
        if let Some((_, child_cont_option)) = self.curt_parent_menu() {
            if let Some(child_cont) = child_cont_option {
                return Some(child_cont.menu_vec[child_sel_y].0.clone());
            }
        }
        return None;
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CtxMenuGroup {
    //   pub curt_term_place: TermPlace,
    pub curt_cont: CtxMenuCont,

    pub selected_cont: CtxMenuContType,
    pub parent_sel_y: usize,
    pub parent_sel_y_cache: usize,
    pub child_sel_y: usize,
    pub child_sel_y_cache: usize,
    pub ctx_menu_place_map: HashMap<TermPlace, CtxMenuCont>,
    pub disp_sy: usize,
    pub disp_ey: usize,
}

impl Default for CtxMenuGroup {
    fn default() -> Self {
        CtxMenuGroup { parent_sel_y: USIZE_UNDEFINED, parent_sel_y_cache: USIZE_UNDEFINED, child_sel_y: USIZE_UNDEFINED, child_sel_y_cache: USIZE_UNDEFINED, ctx_menu_place_map: HashMap::new(), curt_cont: CtxMenuCont::default(), selected_cont: CtxMenuContType::None, disp_sy: 0, disp_ey: 0 }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
// Fixed display information
pub struct CtxMenuCont {
    pub height: usize,
    pub width: usize,
    pub y_area: (usize, usize),
    pub x_area: (usize, usize),
    pub menu_vec: Vec<(CtxMenu, Option<CtxMenuCont>)>,
}

impl CtxMenuCont {
    fn clear(&mut self) {
        self.y_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        self.x_area = (USIZE_UNDEFINED, USIZE_UNDEFINED);
    }
}

impl Default for CtxMenuCont {
    fn default() -> Self {
        CtxMenuCont { height: USIZE_UNDEFINED, width: USIZE_UNDEFINED, y_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), x_area: (USIZE_UNDEFINED, USIZE_UNDEFINED), menu_vec: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CtxMenu {
    pub name: String,
    pub name_disp: String,
}

impl Default for CtxMenu {
    fn default() -> Self {
        CtxMenu { name: String::new(), name_disp: String::new() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermPlace {
    Editor,
    HeaderBar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CtxMenuContType {
    None,
    Parent,
    Child,
}
impl TermPlace {
    pub fn iter() -> Iter<'static, TermPlace> {
        static TERM_PLACE: [TermPlace; 2] = [TermPlace::Editor, TermPlace::HeaderBar];
        TERM_PLACE.iter()
    }
}

impl FromStr for TermPlace {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "editor" => Ok(TermPlace::Editor),
            "headerBar" => Ok(TermPlace::HeaderBar),
            _ => Err(()),
        }
    }
}
