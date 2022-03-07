use crate::{model::*, window::*};
use crossterm::cursor::MoveTo;
use ewin_com::{_cfg::model::default::Cfg, colors::Colors, def::*, log::*, util::*};
use std::{
    cmp::{max, min},
    collections::HashMap,
};

use super::org::TermPlace;

impl Window for CtxMenu {
    fn init(&mut self) {
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
                    child_cont.height = child_cont.menu_vec.len();
                    // +4 is extra
                    child_cont.width = child_max_len_vec[idx] + 4;
                } else {
                    let exist_child_mark_space = if is_exist_child_mark_flg { "" } else { "  " };
                    parent_menu.name_disp = format!("  {}{}{}", perent_str, " ".repeat(space), exist_child_mark_space);
                }
            }
            self.ctx_menu_place_map.get_mut(term_place).unwrap().height = self.ctx_menu_place_map[term_place].menu_vec.len();
            // +1 is Extra
            self.ctx_menu_place_map.get_mut(term_place).unwrap().width = parent_max_len_map[term_place] + 1;
        }
    }

    fn get_draw_range_y(&mut self, offset_y: usize, hbar_disp_row_num: usize, editor_row_len: usize) -> Option<(usize, usize)> {
        Log::debug_key("CtxMenuGroup.get_draw_range");
        if !self.popup.is_menu_change() {
            return None;
        };
        let mut sy = self.popup.disp_sy - hbar_disp_row_num;
        let ey = self.popup.disp_ey - hbar_disp_row_num;

        if self.popup.parent_sel_y_cache != USIZE_UNDEFINED {
            sy = min(sy, self.popup.curt_cont.y_area.0 + self.popup.parent_sel_y_cache);
        }
        if let Some((_, Some(child_cont))) = self.popup.get_curt_parent() {
            // -1 is the correspondence when the previous child menu exists.
            sy = min(sy, child_cont.y_area.0 - 1);
        }
        return Some((offset_y + min(sy, ey), min(offset_y + max(sy, ey), offset_y + editor_row_len)));
    }

    fn clear(&mut self) {
        self.popup.clear();
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
        Log::debug_key("CtxMenuGroup.draw");
        Log::debug("CtxMenuGroup", &self);

        for (parent_idx, (parent_menu, child_cont_option)) in self.popup.curt_cont.menu_vec.iter_mut().enumerate() {
            let color = if parent_idx == self.popup.parent_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
            let name = format!("{}{}", color, parent_menu.name_disp,);

            str_vec.push(format!("{}{}", MoveTo((self.popup.curt_cont.x_area.0) as u16, (self.popup.curt_cont.y_area.0 + parent_idx) as u16), name));
            if parent_idx == self.popup.parent_sel_y {
                if let Some(child_cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in child_cont.menu_vec.iter().enumerate() {
                        let c_name = cut_str(child_menu.name_disp.clone(), child_cont.x_area.1 + 1 - child_cont.x_area.0, false, false);

                        let color = if child_idx == self.popup.child_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
                        let name = format!("{}{}", color, c_name,);
                        str_vec.push(format!("{}{}", MoveTo(child_cont.x_area.0 as u16, (child_cont.y_area.0 + child_idx) as u16), name));
                    }
                }
            }
        }
        str_vec.push(Colors::get_default_fg_bg());
    }
}
