use crate::cur::Cur;
use ewin_cfg::{lang::lang_cfg::Lang, log::*};
use ewin_const::def::*;
use std::{
    cmp::{max, min},
    fmt,
};

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
        // !(self.sy == USIZE_UNDEFINED && self.ey == USIZE_UNDEFINED)

        if self.mode == SelMode::Normal {
            return self.sy != USIZE_UNDEFINED && self.ey != USIZE_UNDEFINED && !(self.sy == self.ey && self.s_disp_x == self.e_disp_x);
        } else {
            // SelMode::BoxSelect
            return !(self.sy == USIZE_UNDEFINED && self.ey == USIZE_UNDEFINED);
        }
    }

    pub fn is_selected_width(&self) -> bool {
        if self.mode == SelMode::Normal {
            self.is_selected()
        } else {
            // SelMode::BoxSelect
            self.is_selected() && self.s_disp_x != self.e_disp_x
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
        SelRange { sy, ey, sx, ex, s_disp_x, e_disp_x, mode: self.mode }
    }

    pub fn set_s(&mut self, y: usize, x: usize, disp_x: usize) {
        if self.mode == SelMode::Normal {
            self.sy = y;
        }
        // SelMode::BoxSelect
        if self.sy == USIZE_UNDEFINED {
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
        }
    }
    pub fn set_sel_posi(&mut self, is_start: bool, cur: Cur) {
        Log::debug_key("box_select_mode");
        Log::debug("is_start", &is_start);
        Log::debug("cur", &cur);

        if is_start {
            if !self.is_selected() {
                self.set_s(cur.y, cur.x, cur.disp_x);
            }
        } else {
            self.set_e(cur.y, cur.x, cur.disp_x);
        }
    }

    pub fn is_contain_y(&self, y: usize) -> bool {
        if self.sy <= y && y <= self.ey {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
/// SelectRange
pub struct SelRange {
    pub mode: SelMode,
    // y 0-indexed
    pub sy: usize,
    pub ey: usize,
    // x 0-indexed (Not included row width)
    pub sx: usize,
    pub ex: usize,
    // 0-indexed
    pub s_disp_x: usize,
    pub e_disp_x: usize,
}
impl Default for SelRange {
    fn default() -> Self {
        SelRange { mode: SelMode::default(), sy: USIZE_UNDEFINED, ey: USIZE_UNDEFINED, sx: USIZE_UNDEFINED, ex: USIZE_UNDEFINED, s_disp_x: USIZE_UNDEFINED, e_disp_x: USIZE_UNDEFINED }
    }
}
impl fmt::Display for SelRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelRange sy:{}, ey:{}, sx:{}, ex:{}, s_disp_x:{}, e_disp_x:{},", self.sy, self.ey, self.sx, self.ex, self.s_disp_x, self.e_disp_x)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub enum SelMode {
    #[default]
    Normal,
    BoxSelect,
}

impl fmt::Display for SelMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelMode::Normal => write!(f, ""),
            SelMode::BoxSelect => write!(f, "{}", Lang::get().box_select),
        }
    }
}
