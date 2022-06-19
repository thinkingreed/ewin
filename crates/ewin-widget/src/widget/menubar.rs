use crate::{core::*, model::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::default::*};
use ewin_com::{_cfg::key::cmd::*, util::*};
use indexmap::IndexMap;
use std::collections::HashMap;

impl MenubarWidget {
    pub fn init(&mut self) {
        Log::debug_key("MenuWidget.init");
        let mut map: Vec<HashMap<String, Vec<HashMap<String, Vec<String>>>>> = serde_json::from_str(&Cfg::get().general.menubar.content.to_string()).unwrap();
        Log::debug("self.menu_map", &self.menu_map.clone());
        self.set_internal_struct(&mut map);
        self.set_all_disp_name();
    }

    pub fn set_internal_struct(&mut self, vec: &mut Vec<HashMap<String, Vec<HashMap<String, Vec<String>>>>>) {
        let mut menubar_map = IndexMap::new();
        // Dividing by 2 is parent, child
        // -8 is extra
        let menunm_max_len = Widget::get_menunm_max_len();
        for map in vec {
            for (parent_name, child_vec_map) in map {
                let parent_menu = WidgetMenu::new(parent_name, menunm_max_len);
                let mut child_cont = WidgetCont { ..WidgetCont::default() };

                for child_map in child_vec_map {
                    // let mut child_menu_vec: Vec<(WidgetMenu, Option<WidgetCont>)> = vec![];
                    let mut grandchild_cont = WidgetCont { ..WidgetCont::default() };

                    for (child_str, child_vec) in child_map {
                        let child_menu = WidgetMenu::new(child_str, menunm_max_len); // { name: , ..WidgetMenu::default() };

                        let mut grandchild_menu_vec: Vec<(WidgetMenu, Option<WidgetCont>)> = vec![];
                        for grandchild_str in child_vec {
                            let grandchild_menu = WidgetMenu { name: cut_str(grandchild_str, menunm_max_len, false, true), ..WidgetMenu::default() };
                            grandchild_menu_vec.push((grandchild_menu, None));
                        }
                        grandchild_cont.cont_vec = grandchild_menu_vec;

                        //  child_menu_vec.push(grandchild_menu_vec);
                        child_cont.cont_vec.push((child_menu, if grandchild_cont.cont_vec.is_empty() { None } else { Some(grandchild_cont.clone()) }));
                        //  parent_menu_vec.push((parent_ctx_menu, if child_cont.cont_vec.is_empty() { None } else { Some(child_cont) }));
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
            MenubarWidget::set_disp_name(parent_menu_str, child_cont);
        }
        Log::debug("self.menu_map", &self.menu_map);
    }

    pub fn set_disp_name(parent_menu_str: &String, child_cont: &mut WidgetCont) {
        Log::debug_key("set_disp_name");

        let exist_child_mark = " > ";
        let mut child_max_len_map: HashMap<String, (usize, bool)> = HashMap::new();
        let mut grandchild_max_len_map: HashMap<(String, String), usize> = HashMap::new();

        // Get the maximum length of the name
        let mut child_max_len = 0;
        let mut is_child_setting_exist = false;
        let mut is_exist_grandchild = false;
        for (idx, (child_menu, grandchild_cont_opt)) in child_cont.cont_vec.iter().enumerate() {
            if let Some(child_str) = get_edit_func_str(&child_menu.name) {
                if !is_child_setting_exist {
                    is_child_setting_exist = WidgetMenu::is_setting_menu(&child_str);
                }
                let child_name_len = get_str_width(&format!("{}{}", WidgetMenu::get_add_setting_str(&child_str), child_str));
                Log::debug("child_menu", &child_menu);
                Log::debug("child_max_len 111", &child_max_len);
                child_max_len = if child_name_len > child_max_len { child_name_len } else { child_max_len };
                Log::debug("child_max_len 222", &child_max_len);

                if let Some(grandchild_cont) = grandchild_cont_opt {
                    let mut grandchild_max_len = 0;
                    if !is_exist_grandchild {
                        is_exist_grandchild = true;
                    }
                    for (grandchild_menu, _) in grandchild_cont.cont_vec.iter() {
                        Log::debug("grandchild_menu.name", &grandchild_menu.name);
                        if let Some(grandchild_str) = get_edit_func_str(&grandchild_menu.name) {
                            let grandchild_name_len = get_str_width(&grandchild_str);
                            grandchild_max_len = if grandchild_name_len > grandchild_max_len { grandchild_name_len } else { grandchild_max_len };
                        }
                    }
                    grandchild_max_len_map.insert((parent_menu_str.clone(), child_menu.name.clone()), grandchild_max_len);
                }
            }

            if child_cont.cont_vec.len() - 1 == idx {
                if is_exist_grandchild {
                    child_max_len += exist_child_mark.len();
                }
                child_max_len_map.insert(parent_menu_str.clone(), (child_max_len, is_child_setting_exist));
            }
        }
        Log::debug("child_max_len_map", &child_max_len_map);
        Log::debug("grandchild_max_len_map", &grandchild_max_len_map);

        // set name_disp
        let mut is_exist_grandchild = false;
        for (child_menu, grandchild_cont_opt) in child_cont.cont_vec.iter_mut() {
            if let Some(child_str) = get_edit_func_str(&child_menu.name) {
                Log::debug("child_str", &child_str);

                let (child_max_len, is_child_setting_exist) = child_max_len_map[parent_menu_str];
                let mut grandchild_max_len = 0;
                let child_str_edit = if is_child_setting_exist {
                    if WidgetMenu::is_setting_menu(&child_str) {
                        format!("{}{}", WidgetMenu::get_add_setting_str(&child_str), child_str)
                    } else {
                        format!("  {}", child_str)
                    }
                } else {
                    child_str
                };
                Log::debug("child_str_edit", &child_str_edit);
                let space = child_max_len - get_str_width(&child_str_edit);
                if let Some(grandchild_cont) = grandchild_cont_opt {
                    if !is_exist_grandchild {
                        is_exist_grandchild = true;
                    }
                    for (grandchild_menu, _) in grandchild_cont.cont_vec.iter_mut() {
                        if let Some(grandchild_str) = get_edit_func_str(&grandchild_menu.name) {
                            // let grandchild_str = get_cfg_lang_name(&grandchild_menu.name);
                            grandchild_max_len = grandchild_max_len_map[&(parent_menu_str.clone(), child_menu.name.clone())];
                            let diff = grandchild_max_len - get_str_width(&grandchild_str);
                            grandchild_menu.disp_name = format!(" {}{} ", grandchild_str, " ".repeat(diff));
                        }
                    }
                    // child_cont.height = child_cont.menu_vec.len();
                    // +4 is extra
                    grandchild_cont.width = grandchild_max_len + 4;
                    child_menu.disp_name = format!(" {}{}{}", child_str_edit, " ".repeat(space - exist_child_mark.len()), exist_child_mark);
                } else {
                    child_menu.disp_name = format!(" {}{} ", child_str_edit, " ".repeat(space),);
                }
            }
        }
        // +1 is Extra
        child_cont.width = child_max_len_map[parent_menu_str].0;

        Log::debug("child_cont", &child_cont);
    }

    pub fn set_menubar_cmd(&mut self, cmd: Cmd) {
        self.cmd = cmd;
    }
}

impl WidgetMenu {
    pub fn new(menu_str: &str, menunm_max_len: usize) -> Self {
        Self { name: cut_str(menu_str, menunm_max_len, false, true), is_get_settings: WidgetMenu::is_setting_menu(menu_str), ..WidgetMenu::default() }
    }

    pub fn get_add_setting_str(menu_str: &str) -> String {
        Log::debug_key("get_add_setting_str");

        let mut add_str = String::new();
        if menu_str == Lang::get().scale {
            add_str.push_str(&format!("{} ", if CfgEdit::get().general.editor.scale.is_enable { "*" } else { " " }));
        } else if menu_str == Lang::get().row_no {
            add_str.push_str(&format!("{} ", if CfgEdit::get().general.editor.row_no.is_enable { "*" } else { " " }));
        }
        Log::debug("add_str", &add_str);
        return add_str;
    }

    pub fn is_setting_menu(menu_str: &str) -> bool {
        return menu_str == Lang::get().scale || menu_str == Lang::get().row_no;
    }
}

impl WidgetTrait for MenubarWidget {
    fn clear(&mut self) {
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
        Log::debug_key("MenuWidget.draw");
        Log::debug("MenuWidget", &self);
        self.curt.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}

#[derive(Debug, Clone)]
pub struct MenubarWidget {
    pub cmd: Cmd,
    pub menu_map: IndexMap<String, WidgetCont>,
    pub curt: Widget,
}

impl Default for MenubarWidget {
    fn default() -> Self {
        MenubarWidget { cmd: Cmd::to_cmd(CmdType::Null), menu_map: IndexMap::new(), curt: Widget::new(WidgetConfig { widget_type: WidgetType::Widget, disp_type: WidgetDispType::Fixed }) }
    }
}
