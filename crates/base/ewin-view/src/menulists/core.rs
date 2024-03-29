use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
};
use ewin_cfg::{colors::*, log::*};
use ewin_const::{
    def::*,
    models::{model::*, view::*},
    term::*,
};
use ewin_utils::{str_edit::*, util::*};
use std::{
    cmp::{max, min},
    io::Write,
    ops::Range,
};

use super::menulist::*;

impl MenuList {
    // How far the CtxMenu is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn init_menu(&mut self, y: usize, x: usize, height: usize) {
        self.set_parent_disp_area(y, x, height);
        self.set_init_menu();
    }

    pub fn set_init_menu(&mut self) {
        if self.parent_sel_y == USIZE_UNDEFINED && !self.cont.cont_vec.is_empty() {
            self.parent_sel_y = 0;
        };
    }

    pub fn get_curt_parent(&self) -> Option<(MenuListMenu, Option<MenuListCont>)> {
        Log::debug_key("Window.get_curt_parent");
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        if let Some((widget_menu, child_cont_option)) = self.cont.cont_vec.get(self.parent_sel_y) {
            return Some((widget_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }
    pub fn get_curt_child(&mut self) -> Option<(MenuListMenu, MenuListCont)> {
        if self.child_sel_y != USIZE_UNDEFINED {
            // let child_sel_y = self.child_sel_y;

            if let Some((_, Some(child_cont))) = self.get_curt_parent() {
                return Some((child_cont.cont_vec[self.child_sel_y].0.clone(), child_cont.clone()));
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
        Log::debug_key("Window.cur_move");
        if self.child_sel_y == USIZE_UNDEFINED {
            self.parent_sel_y_org = self.parent_sel_y;
            match direction {
                Direction::Down => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.cont.cont_vec.get_mut(self.parent_sel_y + 1).is_none() { 0 } else { self.parent_sel_y + 1 },
                Direction::Up => self.parent_sel_y = if self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y == 0 { self.cont.cont_vec.len() - 1 } else { self.parent_sel_y - 1 },
                Direction::Right => {
                    if self.is_exist_child_curt_parent() {
                        self.child_sel_y = 0;
                    }
                }
                Direction::Left => {}
            }
        } else {
            match direction {
                Direction::Down => {
                    if let Some((_, Some(mut child_cont))) = self.get_curt_parent() {
                        self.child_sel_y = if child_cont.cont_vec.get_mut(self.child_sel_y + 1).is_none() { 0 } else { self.child_sel_y + 1 }
                    }
                }
                Direction::Up => {
                    if let Some((_, Some(child_cont))) = self.get_curt_parent() {
                        self.child_sel_y = if self.child_sel_y == 0 { child_cont.cont_vec.len() - 1 } else { self.child_sel_y - 1 };
                    }
                }
                Direction::Left => self.child_sel_y = USIZE_UNDEFINED,
                Direction::Right => {}
            }
        }
        self.set_child_disp_area();
    }

    pub fn set_parent_disp_area(&mut self, y: usize, x: usize, height: usize) {
        Log::debug_key("Window.set_parent_disp_area");
        Log::debug("self.curt_cont.height", &self.cont.view.height);
        Log::debug("self.curt_cont.width", &self.cont.view.width);

        let (cols, rows) = get_term_size();
        // rows
        self.cont.view.height = height;
        let (sy, ey) = if y + MenuList::EXTRA_FROM_CUR_Y + self.cont.view.height > rows {
            let base_y = match self.config.menulist_type {
                MenuListType::MenuList => get_term_size().1 - self.cont.view.height,
                MenuListType::Pulldown => y - self.cont.view.height,
            };
            (base_y, base_y + self.cont.view.height)
        } else {
            let base_y = y + MenuList::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.cont.view.height - 1)
        };

        // cols
        let (sx, ex) = match self.config.disp_type {
            MenuListDispType::Dynamic => {
                if x + MenuList::EXTRA_FROM_CUR_X + self.cont.view.width > cols {
                    let base_x = x + MenuList::EXTRA_FROM_CUR_Y;
                    (base_x - self.cont.view.width + 1, base_x)
                } else {
                    let base_x = x + MenuList::EXTRA_FROM_CUR_X;
                    (base_x, base_x + self.cont.view.width)
                }
            }
            MenuListDispType::Fixed => (x, x + self.cont.view.width),
        };
        self.cont.view.y = sy;

        self.cont.view.x = sx;
        self.cont.view.width = ex - sx;
        Log::debug("self.cont.view", &self.cont.view);

        self.disp_sy_org = self.disp_sy;
        self.disp_ey_org = self.disp_ey;
        self.disp_sy = min(self.disp_sy, sy);
        self.disp_ey = max(self.disp_ey, ey);
        Log::debug("set_parent_disp_area window.self ", &self);
    }

    pub fn set_child_disp_area(&mut self) {
        Log::debug("self.contttttttttttttt", &self.cont.clone());

        if let Some((_, Some(child_cont))) = self.cont.cont_vec.get_mut(self.parent_sel_y) {
            let (cols, rows) = get_term_size();
            //  child_cont.height = min(child_cont.cont_vec.len(), self.max_vertical_range);
            child_cont.view.height = child_cont.cont_vec.len(); // min(child_cont.cont_vec.len(), self.max_vertical_range);
                                                                // rows
            let (sy, ey) = if self.cont.view.y + self.parent_sel_y + child_cont.view.height > rows {
                // let base_y = if child_cont.height > self.max_vertical_range { self.highest_posi } else { self.highest_posi + self.max_vertical_range - child_cont.height };
                let base_y = rows - child_cont.view.height;
                (base_y, base_y + child_cont.view.height)
            } else {
                let child_base_y = self.cont.view.y + self.parent_sel_y;
                (child_base_y, child_base_y + child_cont.view.height - 1)
            };
            // cols

            let (sx, ex) = if self.cont.view.x_width() + 1 + child_cont.view.width + 1 > cols {
                if child_cont.view.width > self.cont.view.x {
                    // Adjust ex to fit in range
                    (0, self.cont.view.x - 1)
                } else {
                    (self.cont.view.x - child_cont.view.width, self.cont.view.x - 1)
                }
            } else {
                Log::debug("child_cont", &child_cont.clone());
                (self.cont.view.x_width(), self.cont.view.x_width() + child_cont.view.width)
            };

            // child_cont.y_area = (min(sy, ey), max(sy, ey));
            child_cont.view.y = min(sy, ey);

            // child_cont.x_area = (min(sx, ex), max(sx, ex));
            child_cont.view.x = min(sx, ex);
            child_cont.view.width = max(sx, ex) - child_cont.view.x;
            Log::debug("child_cont.view", &child_cont.view);

            self.disp_sy_org = self.disp_sy;
            self.disp_ey_org = self.disp_ey;
            self.disp_sy = min(self.disp_sy, sy);
            self.disp_ey = max(self.disp_ey, ey);
        }
    }

    pub fn ctrl_mouse_move(&mut self, y: usize, x: usize) {
        Log::debug_key("ctrl_mouse_move");
        Log::debug("yyy", &y);
        Log::debug("self.cont.y_area.0", &self.cont.view.y);
        Log::debug("self.offset_y", &self.offset_y);
        if self.cont.view.y <= y && y < self.cont.view.y_height() && self.cont.view.x <= x && x <= self.cont.view.x_width() {
            Log::debug_key("set self.parent_sel_y ");
            self.parent_sel_y_org = self.parent_sel_y;
            //     self.parent_sel_y = y - self.cont.y_area.0 + self.offset_y;
            self.parent_sel_y = y - self.cont.view.y + self.offset_y;
        }
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        self.set_child_disp_area();

        if let Some((_, Some(child_cont))) = self.cont.cont_vec.get(self.parent_sel_y) {
            if child_cont.view.y <= y && y < child_cont.view.y_height() && child_cont.view.x <= x && x <= child_cont.view.x_width() {
                self.child_sel_y_org = self.child_sel_y;
                self.child_sel_y = y - child_cont.view.y;
            } else {
                self.child_sel_y = USIZE_UNDEFINED;
            }
        }
    }
    pub fn clear_select_menu(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
    }

    pub fn is_mouse_within_area(&self, y: usize, x: usize) -> bool {
        if self.cont.view.y <= y && y < self.cont.view.y_height() && self.cont.view.x <= x && x <= self.cont.view.x_width() {
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(ref child_cont) = self.cont.cont_vec[self.parent_sel_y].1 {
                if child_cont.view.y <= y && y < child_cont.view.y_height() && child_cont.view.x <= x && x <= child_cont.view.x_width() {
                    return true;
                };
            }
        }
        return false;
    }
    pub fn is_mouse_area_around(&self, y: usize, x: usize) -> bool {
        Log::debug_key("Window.is_mouse_within_range");
        Log::debug("y", &y);
        Log::debug("x", &x);

        if self.cont.view.y - 1 == y || y == self.cont.view.y_height() || (self.cont.view.x != 0 && self.cont.view.x - 1 == x) || x == self.cont.view.x_width() + 1 {
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(ref child_cont) = self.cont.cont_vec[self.parent_sel_y].1 {
                if child_cont.view.y - 1 == y && y == child_cont.view.y_height() || child_cont.view.x - 1 == x || x == child_cont.view.x_width() + 1 {
                    return true;
                };
            }
        }

        return false;
    }
    pub fn is_menu_change(&self) -> bool {
        return self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y != self.parent_sel_y_org || self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y != self.child_sel_y_org;
    }

    pub fn draw(&self, str_vec: &mut Vec<String>, sel_color: &str, not_sel_color: &str) {
        Log::debug_key("Window.draw");
        Log::debug("self.curt_cont.menu_vec.len()", &self.cont.cont_vec.len());

        Log::debug("self.scrl_v", &self.scrl_v);
        Log::debug("self.curt_cont.menu_vec.len()", &self.cont.cont_vec.len());
        Log::debug(" self.parent_sel_y", &self.parent_sel_y);
        Log::debug("self.offset_y", &self.offset_y);
        for (parent_idx, (parent_menu, child_cont_option)) in (0..self.cont.view.height).zip(self.cont.cont_vec[self.offset_y..].iter()) {
            let color = if parent_idx == self.parent_sel_y - self.offset_y { sel_color } else { not_sel_color };
            let name = format!("{}{}", color, parent_menu.disp_name,);

            // Parent menu
            str_vec.push(format!("{}{}", MoveTo((self.cont.view.x) as u16, (self.cont.view.y + parent_idx) as u16), name));

            // Parent scrl_v
            // for Inputcomple ..
            if self.scrl_v.is_show {
                let color = if self.scrl_v.view.y <= parent_idx && parent_idx < self.scrl_v.view.y + self.scrl_v.view.height { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() };
                str_vec.push(format!("{}{}{}", color, MoveTo((self.cont.view.x_width()) as u16 + 1, (self.cont.view.y + parent_idx) as u16), "  ",));
            };

            if parent_idx == self.parent_sel_y {
                if let Some(cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in (0..cont.view.height).zip(cont.cont_vec.iter()) {
                        let c_name = cut_str(&child_menu.disp_name, cont.view.x_width() + 1 - cont.view.x, false, false);

                        Log::debug("child_idx", &child_idx);
                        Log::debug("self.child_sel_y", &self.child_sel_y);

                        let color = if child_idx == self.child_sel_y { sel_color } else { not_sel_color };
                        let name = format!("{}{}", color, c_name,);
                        str_vec.push(format!("{}{}", MoveTo(cont.view.x as u16, (cont.view.y + child_idx) as u16), name));
                    }
                }
            }
        }
        str_vec.push(Colors::get_default_fg_bg());
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T, sel_color: &str, not_sel_color: &str) {
        Log::debug_key("Window.draw_only");
        let mut v: Vec<String> = vec![];
        self.draw(&mut v, sel_color, not_sel_color);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn calc_scrlbar_v(&mut self) {
        Log::debug_key("calc_scrlbar_v");
        if self.scrl_v.is_show {
            if self.scrl_v.view.height == 0 {
                self.scrl_v.calc_com_scrlbar_v(self.cont.view.height, self.cont.cont_vec.len());
            }

            self.scrl_v.view.y = if self.offset_y + self.cont.view.height == self.cont.cont_vec.len() {
                self.cont.view.height - self.scrl_v.view.height
            } else {
                let mut row_posi = (self.offset_y as f64 / self.scrl_v.move_len as f64).ceil() as usize;
                if row_posi + self.scrl_v.view.height == self.cont.view.height && self.offset_y + self.cont.view.height < self.cont.cont_vec.len() {
                    row_posi -= 1;
                }
                row_posi
            }
        }
    }
    pub fn set_offset_y(&mut self, max_height: usize) {
        if self.cont.cont_vec.len() > max_height {
            if self.parent_sel_y == 0 {
                self.offset_y = 0;
            } else if self.parent_sel_y == self.cont.cont_vec.len() - 1 {
                self.offset_y = self.cont.cont_vec.len() - max_height;
            } else if self.parent_sel_y == max_height + self.offset_y - 1 {
                self.offset_y += 1;
            } else if self.parent_sel_y == self.offset_y {
                self.offset_y -= 1;
            }
        }
    }

    pub fn get_disp_range_y(&self) -> Range<usize> {
        return Range { start: min(self.disp_sy, self.disp_sy_org), end: max(self.disp_ey, self.disp_ey_org) + 1 };
    }
    pub fn get_menunm_max_len() -> usize {
        // Dividing by 2 is parent, child
        // -8 is extra
        return get_term_size().0 as usize / 2 - 8;
    }

    pub fn set_disp_name_single_menulist(&mut self, iter: impl IntoIterator<Item = String>, menunm_add_str_len_opt: Option<isize>) {
        let mut cont = MenuListCont { ..MenuListCont::default() };
        for menu_str in iter {
            cont.cont_vec.push((MenuListMenu { name: menu_str.clone(), disp_name: cut_str(&menu_str, MenuList::get_menunm_max_len(), false, true), ..MenuListMenu::default() }, None));
        }
        self.cont = cont;
        let mut parent_max_len = 0;
        for (parent_menu, _) in self.cont.cont_vec.iter() {
            let parent_name_len = get_str_width(&parent_menu.disp_name);
            parent_max_len = if parent_name_len > parent_max_len { parent_name_len } else { parent_max_len };
        }
        if let Some(menunm_add_str_len) = menunm_add_str_len_opt {
            if parent_max_len > 0 {
                parent_max_len = (parent_max_len as isize + menunm_add_str_len) as usize;
            }
        }
        // set name_disp
        for (parent_menu, _) in self.cont.cont_vec.iter_mut() {
            let perent_str = get_cfg_lang_name(&parent_menu.disp_name);
            let space = parent_max_len - get_str_width(perent_str);
            parent_menu.disp_name = format!(" {}{} ", perent_str, get_space(space),);
        }
        // +1 is Extra
        self.cont.view.width = parent_max_len + 1;
    }
}

pub trait MenuListTrait {
    // fn init(&mut self);
    // fn set_disp_name(&mut self);
    fn clear(&mut self);
    fn draw(&mut self, str_vec: &mut Vec<String>);
    fn draw_only<T: Write>(&mut self, out: &mut T) {
        let mut v: Vec<String> = vec![];
        self.draw(&mut v);
        execute!(out, Hide).unwrap();
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
