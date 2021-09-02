use crate::{ctx_menu::init::*, ewin_core::_cfg::keys::*, ewin_core::colors::*, ewin_core::def::*, ewin_core::global::*, ewin_core::log::Log, ewin_core::model::*, ewin_core::util::*, model::*, terminal::*};
use crossterm::{cursor::MoveTo, terminal::size};
use directories::BaseDirs;
use std::{
    cmp::{max, min},
    io::stdout,
};
use {ConvType, CurDirection, DrawType, FmtType};

impl CtxMenuGroup {
    // How far the display is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn select_ctx_menu(term: &mut Terminal) {
        if let Some(ctx_menu) = term.ctx_menu_group.curt_child_menu() {
            CtxMenuGroup::check_func(term, &term.ctx_menu_group.curt_parent_menu().unwrap().0.name, &ctx_menu.name);
        } else if !term.ctx_menu_group.is_exist_child_curt_parent() {
            if let Some((parent_ctx_menu, _)) = term.ctx_menu_group.curt_parent_menu() {
                CtxMenuGroup::check_func(term, &parent_ctx_menu.name, "");
            }
        }
    }
    pub fn check_func(term: &mut Terminal, parent_name: &str, child_name: &str) {
        Log::debug_key("exec_func");

        if LANG_MAP.get(parent_name).is_some() {
            if &LANG.macros == LANG_MAP.get(parent_name).unwrap() {
                if let Some(base_dirs) = BaseDirs::new() {
                    let full_path_str = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR).join(child_name);
                    if full_path_str.exists() {
                        Macros::exec_js_macro(term, &full_path_str.to_string_lossy().to_string());
                    } else {
                        term.curt().mbar.set_err(&LANG.file_not_found);
                    }
                }
            } else if LANG_MAP.get(child_name).is_some() {
                CtxMenuGroup::exec_func(term, child_name);
            } else {
                CtxMenuGroup::exec_func(term, parent_name);
            }
        } else if LANG_MAP.get(child_name).is_some() {
            CtxMenuGroup::exec_func(term, child_name);
        }
        term.clear_ctx_menu();
        term.curt().editor.draw_type = DrawType::All;
    }

    pub fn exec_func(term: &mut Terminal, name: &str) {
        match &LANG_MAP[name] {
            //// editor
            // convert
            s if s == &LANG.to_uppercase || s == &LANG.to_lowercase || s == &LANG.to_full_width || s == &LANG.to_half_width || s == &LANG.to_space || s == &LANG.to_tab => {
                term.curt().editor.convert(ConvType::from_str(&LANG_MAP[name]));
                term.curt().editor.sel.clear();
            }
            // format
            s if s == &LANG.json || s == &LANG.xml || s == &LANG.html => {
                if let Some(err_str) = term.curt().editor.format(FmtType::from_str(s)) {
                    term.curt().mbar.set_err(&err_str);
                } else {
                    // highlight data reset
                    term.editor_draw_vec[term.idx].clear();
                }
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.cut => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Cut), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.copy => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Copy), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.paste => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::InsertStr("".to_string())), &mut stdout(), term);
            }
            s if s == &LANG.all_select => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::AllSelect), &mut stdout(), term);
            }

            //// headerbar
            // close
            s if s == &LANG.close => {

                /* TODO workspcae
                if Prompt___::close(term) {
                    Terminal::exit();
                }
                */
            }
            // close other than this tab
            s if s == &LANG.close_other_than_this_tab => {
                let _ = term.close_tabs(term.idx);
            }
            _ => {}
        };
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenuGroup.draw");

        for (parent_idx, (parent_menu, child_cont_option)) in self.curt_cont.menu_vec.iter_mut().enumerate() {
            let color = if parent_idx == self.parent_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
            let name = format!("{}{}{}", color, parent_menu.name_disp, Colors::get_default_fg_bg());

            str_vec.push(format!("{}{}", MoveTo((self.curt_cont.x_area.0) as u16, (self.curt_cont.y_area.0 + parent_idx) as u16), name));
            if parent_idx == self.parent_sel_y {
                if let Some(child_cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in child_cont.menu_vec.iter().enumerate() {
                        let c_name = cut_str(child_menu.name_disp.clone(), child_cont.x_area.1 + 1 - child_cont.x_area.0, false, false);

                        let color = if child_idx == self.child_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
                        let name = format!("{}{}{}", color, c_name, Colors::get_default_fg_bg());
                        str_vec.push(format!("{}{}", MoveTo(child_cont.x_area.0 as u16, (child_cont.y_area.0 + child_idx) as u16), name));
                    }
                }
            }
        }
    }

    pub fn set_curt_term_place(term: &mut Terminal, y: usize) {
        if term.hbar.disp_row_posi == y {
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&TermPlace::HeaderBar].clone();
        } else if term.curt().editor.disp_row_posi <= y && y <= term.curt().editor.disp_row_posi + term.curt().editor.disp_row_num {
            let place_cond = if term.curt().editor.sel.is_selected() { TermPlace::Editor(TermPlaceCond::EditorRangeSelected) } else { TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected) };
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&place_cond].clone();
        }
    }
    pub fn is_ctx_menu_displayed_area(term: &mut Terminal, y: usize, x: usize) -> bool {
        if y == term.hbar.disp_row_posi {
            for h_file in term.hbar.file_vec.iter() {
                if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 || h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                    return true;
                }
            }
        } else {
            return true;
        }
        return false;
    }
    pub fn show_init(term: &mut Terminal, y: usize, x: usize) {
        let (y, x) = if y == USIZE_UNDEFINED {
            (term.curt().editor.cur.y - term.curt().editor.offset_y + term.hbar.disp_row_num, if term.curt().editor.mouse_mode == MouseMode::Normal { term.curt().editor.cur.disp_x + term.curt().editor.get_rnw_and_margin() } else { term.curt().editor.cur.disp_x })
        } else {
            (y, x)
        };

        term.state.is_ctx_menu = true;
        CtxMenuGroup::set_curt_term_place(term, y);
        term.ctx_menu_group.set_parent_disp_area(y, x);
    }

    pub fn set_parent_disp_area(&mut self, y: usize, x: usize) {
        Log::debug_key("set_parent_disp_area");

        let (cols, rows) = size().unwrap();
        let (cols, rows) = (cols as usize, rows as usize);
        // rows
        let (sy, ey) = if y + CtxMenuGroup::EXTRA_FROM_CUR_Y + self.curt_cont.height > rows {
            let base_y = y - CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_y - self.curt_cont.height + 1, base_y)
        } else {
            let base_y = y + CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.curt_cont.height - 1)
        };
        // cols
        let (sx, ex) = if x + CtxMenuGroup::EXTRA_FROM_CUR_X + self.curt_cont.width > cols {
            let base_x = x + CtxMenuGroup::EXTRA_FROM_CUR_Y;
            (base_x - self.curt_cont.width + 1, base_x)
        } else {
            let base_x = x + CtxMenuGroup::EXTRA_FROM_CUR_X;
            (base_x, base_x + self.curt_cont.width)
        };
        self.curt_cont.y_area = (sy, ey);
        self.curt_cont.x_area = (sx, ex);

        self.disp_sy = min(self.disp_sy, sy);
        self.disp_ey = max(self.disp_ey, ey);
    }

    pub fn clear(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
        self.parent_sel_y_cache = USIZE_UNDEFINED;
        self.child_sel_y = USIZE_UNDEFINED;
        self.disp_sy = USIZE_UNDEFINED;
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
        Log::debug_key("CtxMenuGroup.is_mouse_within_range");

        if self.curt_cont.y_area.0 <= y && y <= self.curt_cont.y_area.1 && self.curt_cont.x_area.0 <= x && x <= self.curt_cont.x_area.1 {
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y].1 {
                if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
                    return true;
                };
            }
        }
        return false;
    }

    pub fn set_child_disp_area(&mut self) {
        if let Some((_, Some(child_cont))) = self.curt_cont.menu_vec.get_mut(self.parent_sel_y) {
            let (cols, rows) = size().unwrap();
            let (cols, rows) = (cols as usize, rows as usize);
            // rows
            let (sy, ey) = if self.curt_cont.y_area.0 + self.parent_sel_y + child_cont.height > rows {
                (rows - child_cont.height, rows)
            } else {
                let child_base_y = self.curt_cont.y_area.0 + self.parent_sel_y;
                (child_base_y, child_base_y + child_cont.height - 1)
            };
            // cols
            let (sx, ex) = if self.curt_cont.x_area.1 + 1 + child_cont.width + 1 > cols {
                if child_cont.width > self.curt_cont.x_area.0 {
                    // Adjust ex to fit in range
                    (0, self.curt_cont.x_area.0 - 1)
                } else {
                    (self.curt_cont.x_area.0 - child_cont.width, self.curt_cont.x_area.0 - 1)
                }
            } else {
                (self.curt_cont.x_area.1 + 1, self.curt_cont.x_area.1 + child_cont.width)
            };

            child_cont.y_area = (min(sy, ey), max(sy, ey));
            child_cont.x_area = (min(sx, ex), max(sx, ex));

            self.disp_sy = min(self.disp_sy, sy);
            self.disp_ey = max(self.disp_ey, ey);
        }
    }
    pub fn ctrl_mouse_move(&mut self, y: usize, x: usize) {
        if self.curt_cont.y_area.0 <= y && y <= self.curt_cont.y_area.1 && self.curt_cont.x_area.0 <= x && x <= self.curt_cont.x_area.1 {
            self.parent_sel_y_cache = self.parent_sel_y;
            self.parent_sel_y = y - self.curt_cont.y_area.0;
        }
        self.set_child_disp_area();

        if let Some((_, Some(child_cont))) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
                self.child_sel_y_cache = self.child_sel_y;
                self.child_sel_y = y - child_cont.y_area.0;
            } else {
                self.child_sel_y = USIZE_UNDEFINED;
            }
        }
    }

    pub fn get_draw_range(&mut self, offset_y: usize, hbar_disp_row_num: usize, editor_disp_row_num: usize) -> Option<(usize, usize)> {
        Log::debug_key("CtxMenuGroup.get_draw_range");
        let sy = offset_y + self.disp_sy - hbar_disp_row_num;
        let ey = offset_y + self.disp_ey;

        if !self.is_menu_change() {
            return None;
        }
        return Some((min(sy, ey), min(max(sy, ey), editor_disp_row_num)));
    }
    pub fn is_menu_change(&mut self) -> bool {
        if self.parent_sel_y != USIZE_UNDEFINED && self.parent_sel_y == self.parent_sel_y_cache && self.child_sel_y == self.child_sel_y_cache {
            return false;
        }
        return true;
    }

    pub fn curt_parent_menu(&self) -> Option<(CtxMenu, Option<CtxMenuCont>)> {
        if let Some((ctx_menu, child_cont_option)) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            return Some((ctx_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }

    pub fn curt_child_menu(&mut self) -> Option<CtxMenu> {
        if self.child_sel_y != USIZE_UNDEFINED {
            // let child_sel_y = self.child_sel_y;

            if let Some((_, Some(child_cont))) = self.curt_parent_menu() {
                return Some(child_cont.menu_vec[self.child_sel_y].0.clone());
            }
        }
        return None;
    }

    pub fn is_exist_child_curt_parent(&self) -> bool {
        if let Some((_, Some(_))) = self.curt_parent_menu() {
            return true;
        }
        return false;
    }

    pub fn cur_move(&mut self, cur_dir: CurDirection) {
        Log::debug_key("CtxMenuGroup.cur_move");
        if self.child_sel_y != USIZE_UNDEFINED {
            match cur_dir {
                CurDirection::Down => {
                    if let Some((_, Some(mut child_cont))) = self.curt_parent_menu() {
                        self.child_sel_y = if child_cont.menu_vec.get_mut(self.child_sel_y + 1).is_none() { 0 } else { self.child_sel_y + 1 }
                    }
                }
                CurDirection::Up => {
                    if let Some((_, Some(child_cont))) = self.curt_parent_menu() {
                        self.child_sel_y = if self.child_sel_y == 0 { child_cont.menu_vec.len() - 1 } else { self.child_sel_y - 1 };
                    }
                }
                CurDirection::Left => self.child_sel_y = USIZE_UNDEFINED,
                CurDirection::Right => {}
            }
        } else {
            self.parent_sel_y_cache = self.parent_sel_y;
            match cur_dir {
                CurDirection::Down => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.curt_cont.menu_vec.get_mut(self.parent_sel_y + 1).is_none() { 0 } else { self.parent_sel_y + 1 },
                CurDirection::Up => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y == 0 { self.curt_cont.menu_vec.len() - 1 } else { self.parent_sel_y - 1 },
                CurDirection::Right => {
                    if self.is_exist_child_curt_parent() {
                        self.child_sel_y = 0;
                    }
                }
                CurDirection::Left => {}
            }
            self.set_child_disp_area();
        }
    }
}
