use crate::model::*;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
};
use ewin_com::{colors::*, def::*, log::Log, model::*, util::*};
use std::{
    cmp::{max, min},
    io::Write,
};

impl Window {
    // How far the CtxMenu is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn set_init_menu(&mut self) {
        if self.parent_sel_y == USIZE_UNDEFINED && !self.curt_cont.menu_vec.is_empty() {
            self.parent_sel_y = 0;
        };
    }

    pub fn get_curt_parent(&self) -> Option<(WindowMenu, Option<WindowCont>)> {
        Log::debug_key("Window.get_curt_parent");
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        if let Some((ctx_menu, child_cont_option)) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            return Some((ctx_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }
    pub fn get_curt_child(&mut self) -> Option<(WindowMenu, WindowCont)> {
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
        Log::debug_key("Window.cur_move");
        if self.child_sel_y == USIZE_UNDEFINED {
            self.parent_sel_y_org = self.parent_sel_y;
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
        } else {
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
        }
        self.set_child_disp_area();
    }

    pub fn set_child_disp_area(&mut self) {
        if let Some((_, Some(child_cont))) = self.curt_cont.menu_vec.get_mut(self.parent_sel_y) {
            let (cols, rows) = get_term_size();
            let editor_disp_y_len = rows - HEADERBAR_ROW_NUM - STATUSBAR_ROW_NUM;
            child_cont.height = min(child_cont.menu_vec.len(), editor_disp_y_len);
            // rows
            let (sy, ey) = if self.curt_cont.y_area.0 + self.parent_sel_y + child_cont.height > editor_disp_y_len {
                let base_y = if child_cont.height > editor_disp_y_len { HEADERBAR_ROW_NUM } else { HEADERBAR_ROW_NUM + editor_disp_y_len - child_cont.height };
                (base_y, base_y + child_cont.height)
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
    pub fn set_parent_disp_area(&mut self, y: usize, x: usize) {
        Log::debug_key("Window.set_parent_disp_area");
        Log::debug("self.curt_cont.height", &self.curt_cont.height);

        let (cols, rows) = get_term_size();
        let editor_disp_y_len = rows - HEADERBAR_ROW_NUM - STATUSBAR_ROW_NUM;
        // rows
        self.curt_cont.height = match self.window_type {
            WindowType::CtxMenu => min(self.curt_cont.menu_vec.len(), editor_disp_y_len),
            WindowType::InputComple => min(self.curt_cont.menu_vec.len(), InputComple::MAX_HEIGHT),
        };
        let (sy, ey) = if y + Window::EXTRA_FROM_CUR_Y + self.curt_cont.height > editor_disp_y_len {
            let base_y = if self.curt_cont.height > editor_disp_y_len { HEADERBAR_ROW_NUM } else { HEADERBAR_ROW_NUM + editor_disp_y_len - self.curt_cont.height };
            (base_y, base_y + self.curt_cont.height)
        } else {
            let base_y = y + Window::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.curt_cont.height - 1)
        };

        // cols
        let (sx, ex) = if x + Window::EXTRA_FROM_CUR_X + self.curt_cont.width > cols {
            let base_x = x + Window::EXTRA_FROM_CUR_Y;
            (base_x - self.curt_cont.width + 1, base_x)
        } else {
            let base_x = x + Window::EXTRA_FROM_CUR_X;
            (base_x, base_x + self.curt_cont.width)
        };
        self.curt_cont.y_area = (sy, ey);
        self.curt_cont.x_area = (sx, ex);

        self.disp_sy = min(self.disp_sy, sy);
        self.disp_ey = max(self.disp_ey, ey);
        Log::debug("set_parent_disp_area window.self ", &self);
    }
    pub fn ctrl_mouse_move(&mut self, y: usize, x: usize) {
        Log::debug_key("ctrl_mouse_move");
        if self.curt_cont.y_area.0 <= y && y <= self.curt_cont.y_area.1 && self.curt_cont.x_area.0 <= x && x <= self.curt_cont.x_area.1 {
            Log::debug_key("set self.parent_sel_y ");
            self.parent_sel_y_org = self.parent_sel_y;
            self.parent_sel_y = y - self.curt_cont.y_area.0 + self.offset_y;
        }
        Log::debug("self.parent_sel_y", &self.parent_sel_y);

        self.set_child_disp_area();

        if let Some((_, Some(child_cont))) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
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

    pub fn is_mouse_within_range(&mut self, y: usize, x: usize, is_around: bool) -> bool {
        Log::debug_key("Window.is_mouse_within_range");
        Log::debug("is_around_check", &is_around);
        Log::debug("y", &y);
        Log::debug("x", &x);
        Log::debug("self.curt_cont.y_area", &self.curt_cont.y_area);
        Log::debug("self.curt_cont.x_area", &self.curt_cont.x_area);

        if is_around {
            if self.curt_cont.y_area.0 - 1 == y || y == self.curt_cont.y_area.1 + 1 || self.curt_cont.x_area.0 - 1 == x || x == self.curt_cont.x_area.1 + 1 {
                return true;
            };
            if self.parent_sel_y != USIZE_UNDEFINED {
                if let Some(child_cont) = &mut self.curt_cont.menu_vec[self.parent_sel_y].1 {
                    if child_cont.y_area.0 - 1 == y && y == child_cont.y_area.1 + 1 || child_cont.x_area.0 - 1 == x || x == child_cont.x_area.1 + 1 {
                        return true;
                    };
                }
            }
        } else {
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
        };

        return false;
    }
    pub fn is_menu_change(&self) -> bool {
        return self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y != self.parent_sel_y_org || self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y != self.child_sel_y_org;
    }
    /*
    pub fn clear(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
        self.parent_sel_y_org = USIZE_UNDEFINED;
        self.child_sel_y = USIZE_UNDEFINED;
        self.disp_sy = USIZE_UNDEFINED;
        self.disp_ey = 0;
    }
     */

    pub fn draw(&mut self, str_vec: &mut Vec<String>, sel_color: &str, not_sel_color: &str) {
        Log::debug_key("Window.draw");
        Log::debug("self.curt_cont.menu_vec.len()", &self.curt_cont.menu_vec.len());

        // calc offset
        self.set_offset_y();
        if self.window_type == WindowType::InputComple {
            self.calc_scrlbar_v();
        }
        Log::debug("self.scrl_v", &self.scrl_v);
        Log::debug("self.curt_cont.menu_vec.len()", &self.curt_cont.menu_vec.len());
        Log::debug(" self.parent_sel_y", &self.parent_sel_y);
        Log::debug("self.offset_y", &self.offset_y);

        for (parent_idx, (parent_menu, child_cont_option)) in (0..self.curt_cont.height).zip(self.curt_cont.menu_vec[self.offset_y..].iter()) {
            let color = if parent_idx == self.parent_sel_y - self.offset_y { sel_color } else { not_sel_color };
            let name = format!("{}{}", color, parent_menu.name_disp,);

            // Parent menu
            str_vec.push(format!("{}{}", MoveTo((self.curt_cont.x_area.0) as u16, (self.curt_cont.y_area.0 + parent_idx) as u16), name));

            if self.window_type == WindowType::InputComple {
                // Parent scrl_v
                if self.scrl_v.is_show && self.scrl_v.row_posi <= parent_idx && parent_idx < self.scrl_v.row_posi + self.scrl_v.bar_len {
                    str_vec.push(format!("{}{}{}", Colors::get_scrollbar_v_bg(), MoveTo((self.curt_cont.x_area.1) as u16 + 1, (self.curt_cont.y_area.0 + parent_idx) as u16), "  ",));
                }
            }
            if parent_idx == self.parent_sel_y {
                if let Some(cont) = child_cont_option {
                    for (child_idx, (child_menu, _)) in (0..cont.height).zip(cont.menu_vec.iter()) {
                        let c_name = cut_str(&child_menu.name_disp, cont.x_area.1 + 1 - cont.x_area.0, false, false);

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

    pub fn get_draw_range_y(&self, editor_offset_y: usize, hbar_disp_row_num: usize, editor_row_len: usize) -> E_DrawRange {
        Log::debug_key("Window.get_draw_range");
        Log::debug("Window.self", &self);
        if !self.is_menu_change() {
            return E_DrawRange::Not;
        };
        let mut sy = self.disp_sy - hbar_disp_row_num;
        let ey = self.disp_ey - hbar_disp_row_num;

        if self.parent_sel_y_org != USIZE_UNDEFINED {
            sy = min(sy, self.curt_cont.y_area.0 + self.parent_sel_y_org);
        }
        if let Some((_, Some(child_cont))) = self.get_curt_parent() {
            // -1 is the correspondence when the previous child menu exists.
            sy = min(sy, child_cont.y_area.0 - 1);
        }

        return if let Some((sy, ey)) = Some((editor_offset_y + min(sy, ey), min(editor_offset_y + max(sy, ey), editor_offset_y + editor_row_len))) { E_DrawRange::TargetRange(sy, ey) } else { E_DrawRange::Not };
    }

    pub fn calc_scrlbar_v(&mut self) {
        Log::debug_key("calc_scrlbar_v");
        if self.scrl_v.is_show {
            if self.scrl_v.bar_len == USIZE_UNDEFINED {
                self.scrl_v.calc_com_scrlbar_v(false, self.curt_cont.height, self.curt_cont.menu_vec.len());
            }
            // self.scrl_v.calc_com_scrlbar_v_roe_posi(false, self.curt_cont.height, &self.e_cmd, self.offset_y, self.scrl_v.move_len);

            Log::debug("self.offset_y + self.curt_cont.height == self.curt_cont.menu_vec.len()", &(self.offset_y + self.curt_cont.height == self.curt_cont.menu_vec.len()));
            Log::debug("self.offset_y ", &self.offset_y);
            Log::debug("self.curt_cont.height", &self.curt_cont.height);
            Log::debug("self.curt_cont.menu_vec.len()", &self.curt_cont.menu_vec.len());

            self.scrl_v.row_posi = if self.offset_y + self.curt_cont.height == self.curt_cont.menu_vec.len() {
                self.curt_cont.height - self.scrl_v.bar_len
            } else {
                let mut row_posi = (self.offset_y as f64 / self.scrl_v.move_len as f64).ceil() as usize;
                if row_posi + self.scrl_v.bar_len == self.curt_cont.height && self.offset_y + self.curt_cont.height < self.curt_cont.menu_vec.len() {
                    row_posi -= 1;
                }
                row_posi
            }
        }
    }
    pub fn set_offset_y(&mut self) {
        if self.window_type == WindowType::InputComple && self.curt_cont.menu_vec.len() > InputComple::MAX_HEIGHT {
            if self.parent_sel_y == 0 {
                self.offset_y = 0;
            } else if self.parent_sel_y == self.curt_cont.menu_vec.len() - 1 {
                self.offset_y = self.curt_cont.menu_vec.len() - InputComple::MAX_HEIGHT;
            } else if self.parent_sel_y == InputComple::MAX_HEIGHT + self.offset_y - 1 {
                self.offset_y += 1;
            } else if self.parent_sel_y == self.offset_y {
                self.offset_y -= 1;
            }
        }
    }
}

pub trait WindowTrait {
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
