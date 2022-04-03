use crate::{core::WindowTrait, model::*};
use ewin_com::{
    _cfg::{key::keycmd::*, model::default::Cfg},
    log::Log,
    util::*,
};
use std::{collections::HashMap, hash::Hash, slice::Iter};

impl CtxMenu {
    pub fn init(&mut self) {
        let mut map: HashMap<String, HashMap<String, Vec<HashMap<String, Vec<String>>>>> = serde_json::from_str(&Cfg::get().general.context_menu.content.to_string()).unwrap();
        self.set_internal_struct(&mut map);
        self.set_disp_name();
    }

    fn set_disp_name(&mut self) {
        Log::debug_key("set_disp_name");

        let exist_child_mark = " > ";
        let mut parent_max_len_map: HashMap<TermPlace, usize> = HashMap::new();
        let mut child_max_len_map: HashMap<TermPlace, Vec<usize>> = HashMap::new();
        for term_place in TermPlace::iter() {
            // max_len name max length
            let mut is_exist_child_mark_flg = false;
            let mut child_max_len_vec: Vec<usize> = vec![];
            let mut parent_max_len = 0;

            for (idx, (parent_menu, child_cont_option)) in self.ctx_menu_place_map[term_place].menu_vec.iter().enumerate() {
                let parent_name_len = get_str_width(get_cfg_lang_name(&parent_menu.name));

                parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };

                let mut child_max_len = 0;
                if let Some(child_cont) = child_cont_option {
                    // +1 is extra
                    if !is_exist_child_mark_flg {
                        is_exist_child_mark_flg = true;
                    }
                    for (child_menu, _) in child_cont.menu_vec.iter() {
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
            for (idx, (parent_menu, child_menu_cont_option)) in self.ctx_menu_place_map.get_mut(term_place).unwrap().menu_vec.iter_mut().enumerate() {
                let parent_max_len = parent_max_len_map[term_place];
                let child_max_len_vec = &child_max_len_map[term_place];
                let perent_str = get_cfg_lang_name(&parent_menu.name);
                let space = parent_max_len - get_str_width(perent_str);
                if let Some(child_cont) = child_menu_cont_option {
                    parent_menu.name_disp = format!("  {}{}{}", perent_str, " ".repeat(space - exist_child_mark.len()), exist_child_mark);
                    for (child_menu, _) in child_cont.menu_vec.iter_mut() {
                        let child_str = get_cfg_lang_name(&child_menu.name);
                        let diff = child_max_len_vec[idx] - get_str_width(child_str);
                        child_menu.name_disp = format!("  {}{}  ", child_str, " ".repeat(diff));
                    }
                    // child_cont.height = child_cont.menu_vec.len();
                    // +4 is extra
                    child_cont.width = child_max_len_vec[idx] + 4;
                } else {
                    let exist_child_mark_space = if is_exist_child_mark_flg { "" } else { "  " };
                    parent_menu.name_disp = format!("  {}{}{}", perent_str, " ".repeat(space), exist_child_mark_space);
                }
            }
            //  self.ctx_menu_place_map.get_mut(term_place).unwrap().height = self.ctx_menu_place_map[term_place].menu_vec.len();
            // +1 is Extra
            self.ctx_menu_place_map.get_mut(term_place).unwrap().width = parent_max_len_map[term_place] + 1;
        }
    }

    pub fn set_ctx_menu_cmd(&mut self, keycmd: KeyCmd) {
        self.c_cmd = match keycmd {
            KeyCmd::CtxMenu(c_cmd) => c_cmd,
            _ => C_Cmd::Null,
        };
    }

    pub fn set_internal_struct(&mut self, map: &mut HashMap<String, HashMap<String, Vec<HashMap<String, Vec<String>>>>>) {
        let mut ctx_menu_map = HashMap::new();
        let (cols, _) = get_term_size();
        // Dividing by 2 is parent, child
        // -8 is extra
        let menunm_max_len = cols as usize / 2 - 8;

        for (place_str, condition_map) in map {
            for (cond_str, condition_vec) in condition_map {
                let mut parent_cont = WindowCont { ..WindowCont::default() };
                let mut parent_menu_vec: Vec<(WindowMenu, Option<WindowCont>)> = vec![];
                for parent_map in condition_vec {
                    for (parent_name, c_vec) in parent_map.iter_mut() {
                        let parent_ctx_menu = WindowMenu { name: cut_str(parent_name, menunm_max_len, false, true), ..WindowMenu::default() };
                        let mut child_cont = WindowCont { ..WindowCont::default() };
                        let mut child_menu_vec: Vec<(WindowMenu, Option<WindowCont>)> = vec![];

                        for child_name in c_vec {
                            let child_ctx_menu = WindowMenu { name: cut_str(child_name, menunm_max_len, false, true), ..WindowMenu::default() };
                            child_menu_vec.push((child_ctx_menu, None));
                        }
                        child_cont.menu_vec = child_menu_vec;
                        parent_menu_vec.push((parent_ctx_menu, if child_cont.menu_vec.is_empty() { None } else { Some(child_cont) }));
                    }
                    parent_cont.menu_vec = parent_menu_vec.clone();
                }
                ctx_menu_map.insert(TermPlace::from_str(place_str, cond_str), parent_cont);
            }
        }
        self.ctx_menu_place_map = ctx_menu_map;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TermPlace {
    Editor(TermPlaceCond),
    HeaderBar,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// TermPlaceCondition
pub enum TermPlaceCond {
    EditorRangeSelected,
    EditorRangeNonSelected,
    None,
}

impl TermPlace {
    pub fn iter() -> Iter<'static, TermPlace> {
        static TERM_PLACE: [TermPlace; 3] = [TermPlace::Editor(TermPlaceCond::EditorRangeSelected), TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected), TermPlace::HeaderBar];
        TERM_PLACE.iter()
    }
}

impl TermPlace {
    fn from_str(place_str: &str, cond_str: &str) -> TermPlace {
        match place_str {
            "editor" => match cond_str {
                "range_selected" => TermPlace::Editor(TermPlaceCond::EditorRangeSelected),
                "range_non_selected" => TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected),
                _ => TermPlace::None,
            },
            "header_bar" => TermPlace::HeaderBar,
            _ => TermPlace::None,
        }
    }
}

use ewin_com::colors::*;

impl WindowTrait for CtxMenu {
    fn clear(&mut self) {
        self.window.clear();
        for (_, parent_cont) in self.ctx_menu_place_map.iter_mut() {
            parent_cont.clear();
            for (_, child_cont_option) in parent_cont.menu_vec.iter_mut() {
                if let Some(child_cont) = child_cont_option {
                    child_cont.clear();
                }
            }
        }
    }
    fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenu.draw");
        Log::debug("CtxMenu", &self);

        self.window.draw(str_vec, &Colors::get_ctx_menu_fg_bg_sel(), &Colors::get_ctx_menu_fg_bg_non_sel());
    }
}
