use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*, model::general::default::*};
use ewin_const::models::{model::WindowSplitType, view::*};
use ewin_job::job::*;
use ewin_state::term::State;
use ewin_utils::str_edit::*;
use ewin_view::menulists::{core::*, menulist::*};
use indexmap::IndexMap;
use std::collections::HashMap;

impl MenubarMenuList {
    pub fn init(&mut self) {
        Log::debug_key("MenubarMenuList.init");
        let mut map: Vec<HashMap<String, Vec<HashMap<String, Vec<String>>>>> = serde_json::from_str(&Cfg::get().general.menubar.content.to_string()).unwrap();
        Log::debug("map", &map);
        self.set_internal_struct(&mut map);
        self.set_all_disp_name();
    }

    pub fn set_internal_struct(&mut self, vec: &mut Vec<HashMap<String, Vec<HashMap<String, Vec<String>>>>>) {
        let mut menubar_map = IndexMap::new();
        // Dividing by 2 is parent, child
        // -8 is extra
        let menunm_max_len = MenuList::get_menunm_max_len();
        for map in vec {
            for (parent_name, child_vec_map) in map {
                let parent_menu = MenuListMenu::new(parent_name, menunm_max_len);
                let mut child_cont = MenuListCont { ..MenuListCont::default() };

                for child_map in child_vec_map {
                    // let mut child_menu_vec: Vec<(WidgetMenu, Option<WidgetCont>)> = vec![];
                    let mut grandchild_cont = MenuListCont { ..MenuListCont::default() };

                    for (child_str, child_vec) in child_map {
                        let child_menu = MenuListMenu::new(child_str, menunm_max_len); // { name: , ..WidgetMenu::default() };

                        let mut grandchild_menu_vec: Vec<(MenuListMenu, Option<MenuListCont>)> = vec![];
                        for grandchild_str in child_vec {
                            let grandchild_menu = MenuListMenu { name: cut_str(grandchild_str, menunm_max_len, false, true), ..MenuListMenu::default() };
                            grandchild_menu_vec.push((grandchild_menu, None));
                        }
                        grandchild_cont.cont_vec = grandchild_menu_vec;

                        child_cont.cont_vec.push((child_menu, if grandchild_cont.cont_vec.is_empty() { None } else { Some(grandchild_cont.clone()) }));
                    }
                }
                menubar_map.insert(parent_menu.name, child_cont);
            }
        }
        self.menu_map = menubar_map;
    }

    pub fn set_all_disp_name(&mut self) {
        Log::debug_key("set_all_disp_name");

        for (parent_menu_str, child_cont) in self.menu_map.iter_mut() {
            MenubarMenuList::set_disp_name(parent_menu_str, child_cont);
        }
        Log::debug("self.menu_map", &self.menu_map);
    }

    pub fn set_disp_name(parent_menu_str: &String, child_cont: &mut MenuListCont) {
        Log::debug_key("set_disp_name");

        let exist_child_mark = " > ";
        let mut child_max_len_map: HashMap<String, usize> = HashMap::new();
        let mut grandchild_max_len_map: HashMap<(String, String), usize> = HashMap::new();

        // Get the maximum length of the name
        let mut child_max_len = 0;
        let mut is_exist_grandchild = false;
        for (idx, (child_menu, grandchild_cont_opt)) in child_cont.cont_vec.iter().enumerate() {
            if let Some(child_str) = get_edit_func_str(&child_menu.name) {
                Log::debug("child_str", &child_str);
                let child_name_len = get_str_width(&format!("{}{}", MenubarMenuList::get_add_setting_str(&child_str), child_str));
                Log::debug("child_menu", &child_menu);
                Log::debug("child_max_len 111", &child_max_len);
                child_max_len = if child_name_len > child_max_len { child_name_len } else { child_max_len };
                Log::debug("child_max_len 222", &child_max_len);
            }

            if let Some(grandchild_cont) = grandchild_cont_opt {
                let mut grandchild_max_len = 0;
                if !is_exist_grandchild {
                    is_exist_grandchild = true;
                }
                for (grandchild_menu, _) in grandchild_cont.cont_vec.iter() {
                    Log::debug("grandchild_menu.name", &grandchild_menu.name);
                    if let Some(grandchild_str) = get_edit_func_str(&grandchild_menu.name) {
                        // let grandchild_name_len = get_str_width(&grandchild_str);
                        Log::debug("grandchild_str", &grandchild_str);
                        let grandchild_name_len = get_str_width(&format!("{}{}", MenubarMenuList::get_add_setting_str(&grandchild_str), grandchild_str));
                        grandchild_max_len = if grandchild_name_len > grandchild_max_len { grandchild_name_len } else { grandchild_max_len };
                    }
                }
                grandchild_max_len_map.insert((parent_menu_str.clone(), child_menu.name.clone()), grandchild_max_len);
            }

            if child_cont.cont_vec.len() - 1 == idx {
                if is_exist_grandchild {
                    child_max_len += exist_child_mark.len();
                }
                child_max_len_map.insert(parent_menu_str.clone(), child_max_len);
            }
        }
        Log::debug("grandchild_max_len_map", &grandchild_max_len_map);

        // set name_disp
        let mut is_exist_grandchild = false;
        for (child_menu, grandchild_cont_opt) in child_cont.cont_vec.iter_mut() {
            if let Some(child_str) = get_edit_func_str(&child_menu.name) {
                Log::debug("child_str", &child_str);

                let child_max_len = child_max_len_map[parent_menu_str];
                let mut grandchild_max_len = 0;

                let child_str_edit = format!("{}{}", MenubarMenuList::get_add_setting_str(&child_str), child_str);
                Log::debug("child_str_edit", &child_str_edit);

                let space = child_max_len - get_str_width(&child_str_edit);
                if let Some(grandchild_cont) = grandchild_cont_opt {
                    if !is_exist_grandchild {
                        is_exist_grandchild = true;
                    }
                    for (grandchild_menu, _) in grandchild_cont.cont_vec.iter_mut() {
                        if let Some(grandchild_str) = get_edit_func_str(&grandchild_menu.name) {
                            // let grandchild_str = get_cfg_lang_name(&grandchild_menu.name);
                            Log::debug("grandchild_str", &grandchild_str);
                            grandchild_max_len = grandchild_max_len_map[&(parent_menu_str.clone(), child_menu.name.clone())];
                            let diff = grandchild_max_len - get_str_width(&grandchild_str);

                            grandchild_menu.disp_name = format!("{}{}{}", MenubarMenuList::get_add_setting_str(&grandchild_str), grandchild_str, get_space(diff));
                            Log::debug("grandchild_menu.disp_name", &grandchild_menu.disp_name);
                        }
                    }
                    // +3 is extra
                    grandchild_cont.width = grandchild_max_len + 3;
                    Log::debug("grandchild_cont.width", &grandchild_cont.width);
                    child_menu.disp_name = format!("{}{}{}", child_str_edit, get_space(space - exist_child_mark.len()), exist_child_mark);
                } else {
                    child_menu.disp_name = format!("{}{}", child_str_edit, get_space(space),);
                }
            }
        }
        child_cont.width = child_max_len_map[parent_menu_str];

        Log::debug("child_cont", &child_cont);
    }

    pub fn get_add_setting_str(menu_str: &str) -> String {
        Log::debug_key("get_add_setting_str");

        let mut add_str = String::new();

        let state_editor = State::get().curt_state().editor;
        let state_sidebar = State::get().sidebar;
        if Lang::get().scale == menu_str {
            add_str.push_str(if state_editor.scale.is_enable { "*" } else { " " });
        } else if Lang::get().row_no == menu_str {
            add_str.push_str(if state_editor.row_no.is_enable { "*" } else { " " });
        } else if Lang::get().left_and_right_split == menu_str {
            add_str.push_str(if state_editor.window_split_type == WindowSplitType::Vertical { "*" } else { " " });
        } else if Lang::get().top_and_bottom_split == menu_str {
            add_str.push_str(if state_editor.window_split_type == WindowSplitType::Horizontal { "*" } else { " " });
        } else if Lang::get().sidebar == menu_str {
            add_str.push_str(if state_sidebar.is_show { "*" } else { " " });
        } else {
            add_str.push(' ');
        }
        add_str.push(' ');

        Log::debug("add_str", &add_str);
        return add_str;
    }
}

impl MenuListTrait for MenubarMenuList {
    fn clear(&mut self) {
        self.is_show = false;
        self.curt.clear();
        for (_, parent_cont) in self.menu_map.iter_mut() {
            parent_cont.clear();
            for (_, child_cont_option) in parent_cont.cont_vec.iter_mut() {
                if let Some(child_cont) = child_cont_option {
                    child_cont.clear();
                }
            }
        }
    }
    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("MenubarMenuList.draw");
        Log::debug("MenubarMenuList", &self);
        self.curt.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}

#[derive(Debug, Clone)]
pub struct MenubarMenuList {
    pub is_show: bool,
    pub menu_map: IndexMap<String, MenuListCont>,
    pub curt: MenuList,
}

impl Default for MenubarMenuList {
    fn default() -> Self {
        MenubarMenuList { is_show: false, menu_map: IndexMap::new(), curt: MenuList::new(MenuListConfig { menulist_type: MenuListType::MenuList, disp_type: MenuListDispType::Fixed }) }
    }
}
