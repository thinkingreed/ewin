use crate::model::*;
use ewin_com::{def::*, log::Log, model::*, util::*};
use std::cmp::{max, min};

impl PopUp {
    // How far the CtxMenu is from the cursor X
    const EXTRA_FROM_CUR_X: usize = 1;
    const EXTRA_FROM_CUR_Y: usize = 1;

    pub fn get_curt_parent(&self) -> Option<(PopUpMenu, Option<PopUpCont>)> {
        if let Some((ctx_menu, child_cont_option)) = self.curt_cont.menu_vec.get(self.parent_sel_y) {
            return Some((ctx_menu.clone(), child_cont_option.clone()));
        }
        return None;
    }
    pub fn get_curt_child(&mut self) -> Option<(PopUpMenu, PopUpCont)> {
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
        if self.child_sel_y == USIZE_UNDEFINED {
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
    pub fn set_parent_disp_area(&mut self, y: usize, x: usize) {
        Log::debug_key("set_parent_disp_area");

        let (cols, rows) = get_term_size();
        // rows
        let (sy, ey) = if y + PopUp::EXTRA_FROM_CUR_Y + self.curt_cont.height > rows {
            let base_y = y - PopUp::EXTRA_FROM_CUR_Y;
            (base_y - self.curt_cont.height + 1, base_y)
        } else {
            let base_y = y + PopUp::EXTRA_FROM_CUR_Y;
            (base_y, base_y + self.curt_cont.height - 1)
        };
        // cols
        let (sx, ex) = if x + PopUp::EXTRA_FROM_CUR_X + self.curt_cont.width > cols {
            let base_x = x + PopUp::EXTRA_FROM_CUR_Y;
            (base_x - self.curt_cont.width + 1, base_x)
        } else {
            let base_x = x + PopUp::EXTRA_FROM_CUR_X;
            (base_x, base_x + self.curt_cont.width)
        };
        self.curt_cont.y_area = (sy, ey);
        self.curt_cont.x_area = (sx, ex);

        self.disp_sy = min(self.disp_sy, sy);
        self.disp_ey = max(self.disp_ey, ey);
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
    pub fn clear_select_menu(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
    }

    pub fn is_mouse_within_range(&mut self, y: usize, x: usize, is_around: bool) -> bool {
        Log::debug_key("CtxMenuGroup.is_mouse_within_range");
        Log::debug("is_around_check", &is_around);

        Log::debug("yyy", &y);
        Log::debug("xxx", &x);
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
    pub fn is_menu_change(&mut self) -> bool {
        return self.parent_sel_y == USIZE_UNDEFINED || self.parent_sel_y != self.parent_sel_y_cache || self.child_sel_y != USIZE_UNDEFINED && self.child_sel_y != self.child_sel_y_cache;
    }
    pub fn clear(&mut self) {
        self.parent_sel_y = USIZE_UNDEFINED;
        self.parent_sel_y_cache = USIZE_UNDEFINED;
        self.child_sel_y = USIZE_UNDEFINED;
        self.disp_sy = USIZE_UNDEFINED;
        self.disp_ey = 0;
    }
}
