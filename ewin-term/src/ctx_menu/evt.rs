use crate::{
    ctx_menu::init::*,
    ewin_core::{
        _cfg::key::{keycmd::*, keys::*, keywhen::*},
        colors::*,
        def::*,
        global::*,
        log::*,
        model::*,
        util::*,
    },
    model::*,
    tab::Tab,
    terminal::*,
};
use crossterm::cursor::MoveTo;
use directories::BaseDirs;
use std::{
    cmp::{max, min},
    io::{stdout, Write},
};

impl CtxMenuGroup {
    // How far the CtxMenu is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn set_keys(&mut self, keys: Keys) {
        let keycmd = Keybind::keys_to_keycmd(&keys, KeyWhen::CtxMenuFocus);
        let c_cmd = match &keycmd {
            KeyCmd::CtxMenu(c_keycmd) => c_keycmd.clone(),
            _ => C_Cmd::Null,
        };
        self.c_cmd = c_cmd;
    }

    pub fn select_ctx_menu(term: &mut Terminal) -> ActType {
        if let Some((ctx_menu, _)) = term.ctx_menu_group.get_curt_child() {
            return CtxMenuGroup::check_func(term, &term.ctx_menu_group.get_curt_parent().unwrap().0.name, &ctx_menu.name);
        } else if !term.ctx_menu_group.is_exist_child_curt_parent() {
            if let Some((parent_ctx_menu, _)) = term.ctx_menu_group.get_curt_parent() {
                return CtxMenuGroup::check_func(term, &parent_ctx_menu.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_func(term: &mut Terminal, parent_name: &str, child_name: &str) -> ActType {
        Log::debug_key("check_func");

        let mut evt_act_type = ActType::Draw(DParts::Editor);
        if LANG_MAP.get(parent_name).is_some() {
            if &LANG.macros == LANG_MAP.get(parent_name).unwrap() {
                if let Some(base_dirs) = BaseDirs::new() {
                    let full_path_str = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR).join(child_name);
                    if full_path_str.exists() {
                        evt_act_type = Macros::exec_js_macro(term, &full_path_str.to_string_lossy().to_string());
                    } else {
                        evt_act_type = ActType::Draw(DParts::MsgBar(LANG.file_not_found.clone()));
                    }
                }
            } else if LANG_MAP.get(child_name).is_some() {
                evt_act_type = CtxMenuGroup::exec_func(term, child_name);
            } else {
                evt_act_type = CtxMenuGroup::exec_func(term, parent_name);
            }
        } else if LANG_MAP.get(child_name).is_some() {
            evt_act_type = CtxMenuGroup::exec_func(term, child_name);
        }
        term.clear_ctx_menu();
        return evt_act_type;
    }

    pub fn exec_func(term: &mut Terminal, name: &str) -> ActType {
        Log::debug_key("exec_func");

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
                    return ActType::Draw(DParts::AllMsgBar(err_str));
                } else {
                    // highlight data reset
                    term.editor_draw_vec[term.idx].clear();
                }
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.cut => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Cut)), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.copy => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Copy)), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &LANG.paste => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::InsertStr("".to_string()))), &mut stdout(), term);
            }
            s if s == &LANG.all_select => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::AllSelect)), &mut stdout(), term);
            }

            //// headerbar
            // close
            s if s == &LANG.close => {
                if Tab::prom_close(term) {
                    Terminal::exit();
                }
            }
            // close other than this tab
            s if s == &LANG.close_other_than_this_tab => {
                let _ = term.close_tabs(term.idx);
            }
            _ => {}
        };
        term.curt().editor.draw_range = EditorDrawRange::All;
        return ActType::Draw(DParts::Editor);
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T) {
        Log::debug_key("CtxMenuGroup.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        self.draw_cur();
        let _ = out.write(&v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("CtxMenuGroup.draw");
        Log::debug("CtxMenuGroup", &self);

        for (parent_idx, (parent_menu, child_cont_option)) in self.curt_cont.menu_vec.iter_mut().enumerate() {
            let color = if parent_idx == self.parent_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
            let name = format!("{}{}", color, parent_menu.name_disp,);

            str_vec.push(format!("{}{}", MoveTo((self.curt_cont.x_area.0) as u16, (self.curt_cont.y_area.0 + parent_idx) as u16), name));
            if parent_idx == self.parent_sel_y {
                if let Some(child_cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in child_cont.menu_vec.iter().enumerate() {
                        let c_name = cut_str(child_menu.name_disp.clone(), child_cont.x_area.1 + 1 - child_cont.x_area.0, false, false);

                        let color = if child_idx == self.child_sel_y { Colors::get_ctx_menu_fg_bg_sel() } else { Colors::get_ctx_menu_fg_bg_non_sel() };
                        let name = format!("{}{}", color, c_name,);
                        str_vec.push(format!("{}{}", MoveTo(child_cont.x_area.0 as u16, (child_cont.y_area.0 + child_idx) as u16), name));
                    }
                }
            }
        }
        str_vec.push(Colors::get_default_fg_bg());
    }

    pub fn draw_cur(&mut self) {
        Terminal::hide_cur();
    }

    pub fn set_curt_term_place(term: &mut Terminal, y: usize) {
        if term.hbar.row_posi == y {
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&TermPlace::HeaderBar].clone();
        } else if term.curt().editor.row_posi <= y && y <= term.curt().editor.row_posi + term.curt().editor.row_num {
            let place_cond = if term.curt().editor.sel.is_selected() { TermPlace::Editor(TermPlaceCond::EditorRangeSelected) } else { TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected) };
            term.ctx_menu_group.curt_cont = term.ctx_menu_group.ctx_menu_place_map[&place_cond].clone();
        }
    }
    pub fn is_ctx_menu_displayed_area(term: &mut Terminal, y: usize, x: usize) -> bool {
        if y == term.hbar.row_posi {
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
    pub fn show_init(term: &mut Terminal) {
        term.ctx_menu_group.clear();
        let (y, x) = match term.curt().editor.e_cmd {
            E_Cmd::MouseDownRight(y, x) => (y, x),
            E_Cmd::MouseDragRight(y, x) => (y, x),
            E_Cmd::CtxtMenu => (USIZE_UNDEFINED, USIZE_UNDEFINED),
            _ => return,
        };
        if CtxMenuGroup::is_ctx_menu_displayed_area(term, y, x) {
            let (y, x) = if y == USIZE_UNDEFINED {
                (term.curt().editor.cur.y - term.curt().editor.offset_y + term.hbar.row_num, if term.curt().editor.state.mouse_mode == MouseMode::Normal { term.curt().editor.cur.disp_x + term.curt().editor.get_rnw_and_margin() } else { term.curt().editor.cur.disp_x })
            } else {
                (y, x)
            };
            term.state.is_ctx_menu = true;
            CtxMenuGroup::set_curt_term_place(term, y);
            term.ctx_menu_group.set_parent_disp_area(y, x);
        } else if term.state.is_ctx_menu {
            term.state.is_ctx_menu = false;
        }
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
    pub fn clear_select_menu(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
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
    pub fn set_parent_disp_area(&mut self, y: usize, x: usize) {
        Log::debug_key("set_parent_disp_area");

        let (cols, rows) = get_term_size();
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

    pub fn set_child_disp_area(&mut self) {
        if let Some((_, Some(child_cont))) = self.curt_cont.menu_vec.get_mut(self.parent_sel_y) {
            let (cols, rows) = get_term_size();
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
        if !self.is_menu_change() {
            return None;
        };
        let mut sy = offset_y + self.disp_sy - hbar_disp_row_num;
        let ey = offset_y + self.disp_ey - hbar_disp_row_num;

        if self.parent_sel_y_cache != USIZE_UNDEFINED {
            sy = max(sy, self.curt_cont.y_area.0 + self.parent_sel_y_cache);
        }
        if let Some((_, Some(child_cont))) = self.get_curt_parent() {
            // -1 is the correspondence when the previous child menu exists.
            sy = min(sy, child_cont.y_area.0 - 1);
        }
        return Some((min(sy, ey), min(max(sy, ey), editor_disp_row_num)));
    }
    pub fn is_menu_change(&mut self) -> bool {
        if self.parent_sel_y != USIZE_UNDEFINED && self.parent_sel_y == self.parent_sel_y_cache && (self.child_sel_y == USIZE_UNDEFINED || self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y == self.child_sel_y_cache) {
            return false;
        }
        return true;
    }

    pub fn get_curt_parent(&self) -> Option<(CtxMenu, Option<CtxMenuCont>)> {
        if let Some((ctx_menu, child_cont_option)) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            return Some((ctx_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }

    pub fn get_curt_child(&mut self) -> Option<(CtxMenu, CtxMenuCont)> {
        if self.child_sel_y != USIZE_UNDEFINED {
            // let child_sel_y = self.child_sel_y;

            if let Some((_, Some(child_cont))) = self.get_curt_parent() {
                return Some((child_cont.menu_vec[self.child_sel_y].0.clone(), child_cont.clone()));
            }
        }
        return None;
    }

    pub fn is_exist_child_curt_parent(&self) -> bool {
        if let Some((_, Some(_))) = self.get_curt_parent() {
            return true;
        }
        return false;
    }

    pub fn cur_move(&mut self, direction: Direction) {
        Log::debug_key("CtxMenuGroup.cur_move");
        if self.child_sel_y != USIZE_UNDEFINED {
            match direction {
                Direction::Down => {
                    if let Some((_, Some(mut child_cont))) = self.get_curt_parent() {
                        self.child_sel_y = if child_cont.menu_vec.get_mut(self.child_sel_y + 1).is_none() { 0 } else { self.child_sel_y + 1 }
                    }
                }
                Direction::Up => {
                    if let Some((_, Some(child_cont))) = self.get_curt_parent() {
                        self.child_sel_y = if self.child_sel_y == 0 { child_cont.menu_vec.len() - 1 } else { self.child_sel_y - 1 };
                    }
                }
                Direction::Left => self.child_sel_y = USIZE_UNDEFINED,
                Direction::Right => {}
            }
        } else {
            self.parent_sel_y_cache = self.parent_sel_y;
            match direction {
                Direction::Down => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.curt_cont.menu_vec.get_mut(self.parent_sel_y + 1).is_none() { 0 } else { self.parent_sel_y + 1 },
                Direction::Up => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y == 0 { self.curt_cont.menu_vec.len() - 1 } else { self.parent_sel_y - 1 },
                Direction::Right => {
                    if self.is_exist_child_curt_parent() {
                        self.child_sel_y = 0;
                    }
                }
                Direction::Left => {}
            }
            self.set_child_disp_area();
        }
    }
}
