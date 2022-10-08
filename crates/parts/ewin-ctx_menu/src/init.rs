use std::collections::HashMap;

use crate::ctx_menu::*;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::models::{term::*, view::*};
use ewin_utils::{str_edit::*, util::*};
use ewin_view::menulists::menulist::*;

impl CtxMenu {
    pub fn init(&mut self) {
        let mut map: HashMap<String, HashMap<String, Vec<HashMap<String, Vec<String>>>>> = serde_json::from_str(&Cfg::get().general.context_menu.content.to_string()).unwrap();
        self.set_internal_struct(&mut map);
        self.set_disp_name();
    }

    pub fn set_internal_struct(&mut self, map: &mut HashMap<String, HashMap<String, Vec<HashMap<String, Vec<String>>>>>) {
        let mut ctx_menu_map = HashMap::new();
        let menunm_max_len = MenuList::get_menunm_max_len();

        for (place_str, condition_map) in map {
            for (cond_str, condition_vec) in condition_map {
                let mut parent_cont = MenuListCont { ..MenuListCont::default() };
                let mut parent_menu_vec: Vec<(MenuListMenu, Option<MenuListCont>)> = vec![];
                for parent_map in condition_vec {
                    for (parent_name, c_vec) in parent_map.iter_mut() {
                        let parent_ctx_menu = MenuListMenu { name: cut_str(parent_name, menunm_max_len, false, true), ..MenuListMenu::default() };
                        let mut child_cont = MenuListCont { ..MenuListCont::default() };
                        let mut child_menu_vec: Vec<(MenuListMenu, Option<MenuListCont>)> = vec![];

                        for child_name in c_vec {
                            let child_ctx_menu = MenuListMenu { name: cut_str(child_name, menunm_max_len, false, true), ..MenuListMenu::default() };
                            child_menu_vec.push((child_ctx_menu, None));
                        }
                        child_cont.cont_vec = child_menu_vec;
                        parent_menu_vec.push((parent_ctx_menu, if child_cont.cont_vec.is_empty() { None } else { Some(child_cont) }));
                    }
                    parent_cont.cont_vec = parent_menu_vec.clone();
                }
                ctx_menu_map.insert(CtxMenuPlace::from_str(place_str, cond_str), parent_cont);
            }
        }
        self.place_menulist_map = ctx_menu_map;
    }

    fn set_disp_name(&mut self) {
        Log::debug_key("set_disp_name");

        let exist_child_mark = " > ";
        let mut parent_max_len_map: HashMap<CtxMenuPlace, usize> = HashMap::new();
        let mut child_max_len_map: HashMap<CtxMenuPlace, Vec<usize>> = HashMap::new();
        for term_place in CtxMenuPlace::iter() {
            // max_len name max length
            let mut is_exist_child_mark_flg = false;
            let mut child_max_len_vec: Vec<usize> = vec![];
            let mut parent_max_len = 0;

            for (idx, (parent_menu, child_cont_option)) in self.place_menulist_map[term_place].cont_vec.iter().enumerate() {
                let parent_name_len = get_str_width(get_cfg_lang_name(&parent_menu.name));

                parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };

                let mut child_max_len = 0;
                if let Some(child_cont) = child_cont_option {
                    // +1 is extra
                    if !is_exist_child_mark_flg {
                        is_exist_child_mark_flg = true;
                    }
                    for (child_menu, _) in child_cont.cont_vec.iter() {
                        let child_name_len = get_str_width(get_cfg_lang_name(&child_menu.name));
                        child_max_len = if child_name_len > child_max_len { child_name_len } else { child_max_len };
                    }
                }

                child_max_len_vec.insert(idx, child_max_len);
            }
            if is_exist_child_mark_flg {
                parent_max_len += exist_child_mark.len();
            }

            parent_max_len_map.insert(*term_place, parent_max_len);
            child_max_len_map.insert(*term_place, child_max_len_vec);

            // set name_disp
            for (idx, (parent_menu, child_menu_cont_option)) in self.place_menulist_map.get_mut(term_place).unwrap().cont_vec.iter_mut().enumerate() {
                let parent_max_len = parent_max_len_map[term_place];
                let child_max_len_vec = &child_max_len_map[term_place];
                let perent_str = get_cfg_lang_name(&parent_menu.name);
                let space = parent_max_len - get_str_width(perent_str);
                if let Some(child_cont) = child_menu_cont_option {
                    parent_menu.disp_name = format!("  {}{}{}", perent_str, get_space(space - exist_child_mark.len()), exist_child_mark);
                    for (child_menu, _) in child_cont.cont_vec.iter_mut() {
                        let child_str = get_cfg_lang_name(&child_menu.name);
                        let diff = child_max_len_vec[idx] - get_str_width(child_str);
                        child_menu.disp_name = format!("  {}{}  ", child_str, get_space(diff));
                    }
                    // child_cont.height = child_cont.menu_vec.len();
                    // +4 is extra
                    child_cont.view.width = child_max_len_vec[idx] + 4;
                } else {
                    parent_menu.disp_name = format!("  {}{}", perent_str, get_space(space));
                }
            }
            //  self.ctx_menu_place_map.get_mut(term_place).unwrap().height = self.ctx_menu_place_map[term_place].menu_vec.len();
            // +1 is Extra
            self.place_menulist_map.get_mut(term_place).unwrap().view.width = parent_max_len_map[term_place] + 2;
        }
    }
}
