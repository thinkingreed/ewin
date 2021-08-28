use crate::{def::*, log::Log, model::*};
use std::cmp::{max, min};

impl SelRange {
    pub fn clear(&mut self) {
        Log::debug_key("SelRange.clear");

        self.sy = USIZE_UNDEFINED;
        self.ey = USIZE_UNDEFINED;
        self.sx = USIZE_UNDEFINED;
        self.ex = USIZE_UNDEFINED;
        self.s_disp_x = USIZE_UNDEFINED;
        self.e_disp_x = USIZE_UNDEFINED;
    }

    // For prompt buf
    pub fn clear_prompt(&mut self) {
        self.sx = USIZE_UNDEFINED;
        self.ex = USIZE_UNDEFINED;
        self.s_disp_x = USIZE_UNDEFINED;
        self.e_disp_x = USIZE_UNDEFINED;
    }

    pub fn is_selected(&self) -> bool {
        if self.mode == SelMode::Normal {
            return self.sy != self.ey || self.s_disp_x != self.e_disp_x;
        } else {
            // SelMode::BoxSelect
            return !(self.sy == USIZE_UNDEFINED && self.ey == USIZE_UNDEFINED);
        }
    }

    pub fn is_selected_width(&self) -> bool {
        if self.mode == SelMode::Normal {
            return self.is_selected();
        } else {
            // SelMode::BoxSelect
            return self.is_selected() && !(self.s_disp_x == self.e_disp_x);
        }
    }

    // Convert to start position < end position
    pub fn get_range(&self) -> Self {
        let mut sy = self.sy;
        let mut ey = self.ey;
        let mut sx = self.sx;
        let mut ex = self.ex;
        let mut s_disp_x = self.s_disp_x;
        let mut e_disp_x = self.e_disp_x;
        if self.mode == SelMode::Normal {
            if sy > ey || (sy == ey && s_disp_x > e_disp_x) {
                sy = self.ey;
                ey = self.sy;
                sx = self.ex;
                ex = self.sx;
                s_disp_x = self.e_disp_x;
                e_disp_x = self.s_disp_x;
            }
        // SelMode::BoxSelect
        } else {
            sy = min(self.sy, self.ey);
            ey = max(self.sy, self.ey);
            sx = min(self.sx, self.ex);
            ex = max(self.sx, self.ex);
            s_disp_x = min(self.s_disp_x, self.e_disp_x);
            e_disp_x = max(self.s_disp_x, self.e_disp_x);
        }
        return SelRange { sy, ey, sx, ex, s_disp_x, e_disp_x, mode: self.mode };
    }

    pub fn set_s(&mut self, y: usize, x: usize, disp_x: usize) {
        if self.mode == SelMode::Normal {
            self.sy = y;

            // SelMode::BoxSelect
        } else if self.sy == USIZE_UNDEFINED {
            self.sy = y;
        }
        self.sx = x;
        self.s_disp_x = disp_x;
    }

    pub fn set_e(&mut self, y: usize, x: usize, disp_x: usize) {
        self.ey = y;
        self.ex = x;
        self.e_disp_x = disp_x;
    }

    pub fn check_overlap(&mut self) {
        // selectio start position and cursor overlap
        if self.mode == SelMode::Normal {
            if self.sy == self.ey && self.s_disp_x == self.e_disp_x {
                Log::debug_s("sel check_overlap");
                self.clear();
            }
        } else {
            /*
            if self.s_disp_x == self.e_disp_x {
                Log::debug_s("sel overlap_box_sel");
                self.overlap_box_sel();
            }
            */
        }
    }
    pub fn set_sel_posi(&mut self, is_start: bool, y: usize, x: usize, disp_x: usize) {
        if is_start {
            if !self.is_selected() {
                self.set_s(y, x, disp_x);
            }
        } else {
            self.set_e(y, x, disp_x);
        }
    }
    pub fn is_another_select(&mut self, sel_org: SelRange) -> bool {
        if self.sy == sel_org.sy && self.s_disp_x == sel_org.s_disp_x {
            return false;
        }
        return true;
    }
}

impl BoxInsert {
    pub fn clear_clipboard(&mut self) {
        self.vec = vec![]
    }
    pub fn get_str(&mut self, nl: &String) -> String {
        let mut str = String::new();
        for (_, s) in self.vec.iter() {
            str.push_str(&s);
            str.push_str(&nl);
        }
        return str;
    }
}
