use crate::model::*;
use ewin_com::{_cfg::key::keycmd::*, util::*};
use std::{collections::HashMap, hash::Hash, slice::Iter};

impl CtxMenu {
    pub fn set_ctx_menu_cmd(&mut self, keycmd: KeyCmd) {
        //  let keycmd = Keybind::keys_to_keycmd(&keys, None, KeyWhen::CtxMenuFocus);
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
                let mut parent_cont = PopUpCont { ..PopUpCont::default() };
                let mut parent_menu_vec: Vec<(PopUpMenu, Option<PopUpCont>)> = vec![];
                for parent_map in condition_vec {
                    for (parent_name, c_vec) in parent_map.iter_mut() {
                        let parent_ctx_menu = PopUpMenu { name: cut_str(parent_name.clone(), menunm_max_len, false, true), ..PopUpMenu::default() };
                        let mut child_cont = PopUpCont { ..PopUpCont::default() };
                        let mut child_menu_vec: Vec<(PopUpMenu, Option<PopUpCont>)> = vec![];

                        for child_name in c_vec {
                            let child_ctx_menu = PopUpMenu { name: cut_str(child_name.clone(), menunm_max_len, false, true), ..PopUpMenu::default() };
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
