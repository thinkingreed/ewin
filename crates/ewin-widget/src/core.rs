use crate::model::*;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
};
use ewin_cfg::{colors::*, log::*};
use ewin_com::{model::*, util::*};
use ewin_const::def::*;
use std::{
    cmp::{max, min},
    io::Write,
    ops::Range,
};

impl Widget {
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

    pub fn get_curt_parent(&self) -> Option<(WidgetMenu, Option<WidgetCont>)> {
        Log::debug_key("Window.get_curt_parent");
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        if let Some((widget_menu, child_cont_option)) = self.cont.cont_vec.get(self.parent_sel_y) {
            return Some((widget_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }
    pub fn get_curt_child(&mut self) -> Option<(WidgetMenu, WidgetCont)> {
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

    pub fn set_child_disp_area(&mut self) {
        if let Some((_, Some(child_cont))) = self.cont.cont_vec.get_mut(self.parent_sel_y) {
            let (cols, rows) = get_term_size();
            //  child_cont.height = min(child_cont.cont_vec.len(), self.max_vertical_range);
            child_cont.height = child_cont.cont_vec.len(); // min(child_cont.cont_vec.len(), self.max_vertical_range);
                                                           // rows
            let (sy, ey) = if self.cont.y_area.0 + self.parent_sel_y + child_cont.height > rows {
                // let base_y = if child_cont.height > self.max_vertical_range { self.highest_posi } else { self.highest_posi + self.max_vertical_range - child_cont.height };
                let base_y = rows - child_cont.height;
                (base_y, base_y + child_cont.height)
            } else {
                let child_base_y = self.cont.y_area.0 + self.parent_sel_y;
                (child_base_y, child_base_y + child_cont.height - 1)
            };
            // cols
            let (sx, ex) = if self.cont.x_area.1 + 1 + child_cont.width + 1 > cols {
                if child_cont.width > self.cont.x_area.0 {
                    // Adjust ex to fit in range
                    (0, self.cont.x_area.0 - 1)
                } else {
                    (self.cont.x_area.0 - child_cont.width, self.cont.x_area.0 - 1)
                }
            } else {
                (self.cont.x_area.1 + 1, self.cont.x_area.1 + child_cont.width)
            };

            child_cont.y_area = (min(sy, ey), max(sy, ey));
            child_cont.x_area = (min(sx, ex), max(sx, ex));

            self.disp_sy_org = self.disp_sy;
            self.disp_ey_org = self.disp_ey;
            self.disp_sy = min(self.disp_sy, sy);
            self.disp_ey = max(self.disp_ey, ey);
        }
    }
    pub fn set_parent_disp_area(&mut self, y: usize, x: usize, height: usize) {
        Log::debug_key("Window.set_parent_disp_area");
        Log::debug("self.curt_cont.height", &self.cont.height);
        Log::debug("self.curt_cont.width", &self.cont.width);

        let (cols, rows) = get_term_size();
        // rows
        self.cont.height = height;
        let (sy, ey) = if y + Widget::EXTRA_FROM_CUR_Y + self.cont.height > rows {
            let base_y = match self.config.widget_type {
                WidgetType::Widget => get_term_size().1 - self.cont.height,
                WidgetType::Pulldown => y - self.cont.height,
            };
            (base_y, base_y + self.cont.height)
        } else {
            let base_y = y + Widget::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.cont.height - 1)
        };

        // cols
        let (sx, ex) = match self.config.disp_type {
            WidgetDispType::Dynamic => {
                if x + Widget::EXTRA_FROM_CUR_X + self.cont.width > cols {
                    let base_x = x + Widget::EXTRA_FROM_CUR_Y;
                    (base_x - self.cont.width + 1, base_x)
                } else {
                    let base_x = x + Widget::EXTRA_FROM_CUR_X;
                    (base_x, base_x + self.cont.width)
                }
            }
            WidgetDispType::Fixed => (x, x + self.cont.width),
        };
        self.cont.y_area = (sy, ey);
        self.cont.x_area = (sx, ex);

        self.disp_sy_org = self.disp_sy;
        self.disp_ey_org = self.disp_ey;
        self.disp_sy = min(self.disp_sy, sy);
        self.disp_ey = max(self.disp_ey, ey);
        Log::debug("set_parent_disp_area window.self ", &self);
    }

    pub fn ctrl_mouse_move(&mut self, y: usize, x: usize) {
        Log::debug_key("ctrl_mouse_move");
        Log::debug("yyy", &y);
        Log::debug("self.cont.y_area.0", &self.cont.y_area.0);
        Log::debug("self.offset_y", &self.offset_y);
        if self.cont.y_area.0 <= y && y <= self.cont.y_area.1 && self.cont.x_area.0 <= x && x <= self.cont.x_area.1 {
            Log::debug_key("set self.parent_sel_y ");
            self.parent_sel_y_org = self.parent_sel_y;
            //     self.parent_sel_y = y - self.cont.y_area.0 + self.offset_y;
            self.parent_sel_y = y - self.cont.y_area.0 + self.offset_y;
        }
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        self.set_child_disp_area();

        if let Some((_, Some(child_cont))) = self.cont.cont_vec.get(self.parent_sel_y) {
            if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
                self.child_sel_y_org = self.child_sel_y;
                self.child_sel_y = y - child_cont.y_area.0;
            } else {
                self.child_sel_y = USIZE_UNDEFINED;
            }
        }
    }
    pub fn clear_select_menu(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
    }

    pub fn is_mouse_within_area(&self, y: usize, x: usize) -> bool {
        Log::debug_key("Widget.is_mouse_within_area");
        Log::debug("self.cont.y_area", &self.cont.y_area);
        Log::debug("self.cont.x_area", &self.cont.x_area);
        if self.cont.y_area.0 <= y && y <= self.cont.y_area.1 && self.cont.x_area.0 <= x && x <= self.cont.x_area.1 {
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(ref child_cont) = self.cont.cont_vec[self.parent_sel_y].1 {
                if child_cont.y_area.0 <= y && y <= child_cont.y_area.1 && child_cont.x_area.0 <= x && x <= child_cont.x_area.1 {
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
        Log::debug("self.curt_cont.y_area", &self.cont.y_area);
        Log::debug("self.curt_cont.x_area", &self.cont.x_area);

        if self.cont.y_area.0 - 1 == y || y == self.cont.y_area.1 + 1 || (self.cont.x_area.0 != 0 && self.cont.x_area.0 - 1 == x) || x == self.cont.x_area.1 + 1 {
            return true;
        };
        if self.parent_sel_y != USIZE_UNDEFINED {
            if let Some(ref child_cont) = self.cont.cont_vec[self.parent_sel_y].1 {
                if child_cont.y_area.0 - 1 == y && y == child_cont.y_area.1 + 1 || child_cont.x_area.0 - 1 == x || x == child_cont.x_area.1 + 1 {
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

        for (parent_idx, (parent_menu, child_cont_option)) in (0..self.cont.height).zip(self.cont.cont_vec[self.offset_y..].iter()) {
            let color = if parent_idx == self.parent_sel_y - self.offset_y { sel_color } else { not_sel_color };
            let name = format!("{}{}", color, parent_menu.disp_name,);

            // Parent menu
            str_vec.push(format!("{}{}", MoveTo((self.cont.x_area.0) as u16, (self.cont.y_area.0 + parent_idx) as u16), name));

            // Parent scrl_v
            // for Inputcomple ..
            if self.scrl_v.is_show {
                let color = if self.scrl_v.row_posi <= parent_idx && parent_idx < self.scrl_v.row_posi + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() };
                str_vec.push(format!("{}{}{}", color, MoveTo((self.cont.x_area.1) as u16 + 1, (self.cont.y_area.0 + parent_idx) as u16), "  ",));
            };

            if parent_idx == self.parent_sel_y {
                if let Some(cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in (0..cont.height).zip(cont.cont_vec.iter()) {
                        let c_name = cut_str(&child_menu.disp_name, cont.x_area.1 + 1 - cont.x_area.0, false, false);

                        Log::debug("child_idx", &child_idx);
                        Log::debug("self.child_sel_y", &self.child_sel_y);

                        let color = if child_idx == self.child_sel_y { sel_color } else { not_sel_color };
                        let name = format!("{}{}", color, c_name,);
                        str_vec.push(format!("{}{}", MoveTo(cont.x_area.0 as u16, (cont.y_area.0 + child_idx) as u16), name));
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
            if self.scrl_v.bar_len == USIZE_UNDEFINED {
                self.scrl_v.calc_com_scrlbar_v(false, self.cont.height, self.cont.cont_vec.len());
            }
            // self.scrl_v.calc_com_scrlbar_v_roe_posi(false, self.curt_cont.height, &self.e_cmd, self.offset_y, self.scrl_v.move_len);

            Log::debug("self.scrl_v.bar_len", &self.scrl_v.bar_len);
            Log::debug("self.offset_y + self.curt_cont.height == self.curt_cont.menu_vec.len()", &(self.offset_y + self.cont.height == self.cont.cont_vec.len()));
            Log::debug("self.offset_y ", &self.offset_y);
            Log::debug("self.curt_cont.height", &self.cont.height);
            Log::debug("self.curt_cont.menu_vec.len()", &self.cont.cont_vec.len());

            self.scrl_v.row_posi = if self.offset_y + self.cont.height == self.cont.cont_vec.len() {
                self.cont.height - self.scrl_v.bar_len
            } else {
                let mut row_posi = (self.offset_y as f64 / self.scrl_v.move_len as f64).ceil() as usize;
                if row_posi + self.scrl_v.bar_len == self.cont.height && self.offset_y + self.cont.height < self.cont.cont_vec.len() {
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
        return Range { start: min(self.disp_sy, self.disp_sy_org), end: max(self.disp_ey, self.disp_ey_org) };
    }
    pub fn get_menunm_max_len() -> usize {
        // Dividing by 2 is parent, child
        // -8 is extra
        return get_term_size().0 as usize / 2 - 8;
    }

    pub fn set_disp_name_single_widget(&mut self, iter: impl IntoIterator<Item = String>, menunm_add_str_len_opt: Option<isize>) {
        let mut cont = WidgetCont { ..WidgetCont::default() };
        for menu_str in iter {
            cont.cont_vec.push((WidgetMenu { name: menu_str.clone(), disp_name: cut_str(&menu_str, Widget::get_menunm_max_len(), false, true), ..WidgetMenu::default() }, None));
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
            parent_menu.disp_name = format!(" {}{} ", perent_str, " ".repeat(space),);
        }
        // +1 is Extra
        self.cont.width = parent_max_len + 1;
    }
}

pub trait WidgetTrait {
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
