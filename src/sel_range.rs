use crate::{def::*, global::*, log::Log};
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
        //  Log::ep_s("SelRange.clear");
        self.sx = USIZE_UNDEFINED;
        self.ex = USIZE_UNDEFINED;
        self.s_disp_x = USIZE_UNDEFINED;
        self.e_disp_x = USIZE_UNDEFINED;
    }

    pub fn is_selected(&self) -> bool {
        if self.mode == SelMode::Normal {
            return !(self.sy == USIZE_UNDEFINED && self.ey == USIZE_UNDEFINED && self.s_disp_x == USIZE_UNDEFINED && self.e_disp_x == USIZE_UNDEFINED);
        } else {
            // SelMode::BoxSelect
            // return !(self.s_disp_x == USIZE_UNDEFINED && self.e_disp_x == USIZE_UNDEFINED);
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
        } else {
            if self.sy == USIZE_UNDEFINED {
                self.sy = y;
            }
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
    pub fn get_diff_y_mouse_drag(&mut self, sel_org: SelRange, cur_y: usize) -> usize {
        let sel = self.get_range();
        let sel_org = sel_org.get_range();

        if sel.sy < sel_org.sy {
            return sel.sy;
        } else if sel.sy > sel_org.sy {
            return sel.sy - 1;
        } else if sel.ey > sel_org.ey {
            return sel.ey - 1;
        } else if sel.ey < sel_org.ey {
            return sel.ey;
        } else if sel.sy == cur_y {
            return sel.sy;
        //sel.ey == cur_y
        } else {
            return sel.ey - 1;
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SelMode {
    Normal,
    BoxSelect,
}
impl Default for SelMode {
    fn default() -> Self {
        SelMode::Normal
    }
}
impl fmt::Display for SelMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelMode::Normal => write!(f, ""),
            SelMode::BoxSelect => write!(f, "{}", LANG.box_select),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxInsert {
    pub mode: BoxInsertMode,
    pub vec: Vec<(SelRange, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxInsertMode {
    Normal,
    Insert,
}

impl Default for BoxInsert {
    fn default() -> Self {
        BoxInsert { vec: vec![], mode: BoxInsertMode::Normal }
    }
}

impl fmt::Display for BoxInsertMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoxInsertMode::Normal => write!(f, ""),
            BoxInsertMode::Insert => write!(f, "{}", LANG.box_insert),
        }
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
