use crate::model::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::log::*;
use ewin_const::models::view::*;
use ewin_key::{cur::*, sel_range::*};
use ewin_view::{model::*, scrollbar_v::*};

impl Window {
    pub fn clear_draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("Window.clear_all_win");
        for i in self.area_v.0..=self.area_v.1 {
            str_vec.push(format!("{}{}", MoveTo(self.area_h_all.0 as u16, i as u16), get_space(self.width_all())));
        }
        str_vec.push(format!("{}", MoveTo(self.area_h.0 as u16, self.area_v.0 as u16)));
    }

    pub fn width(&self) -> usize {
        return self.area_h.1 - self.area_h.0;
    }

    pub fn height(&self) -> usize {
        return self.area_v.1 - self.area_v.0;
    }

    pub fn width_all(&self) -> usize {
        return self.area_h_all.1 - self.area_h_all.0;
    }

    pub fn new() -> Self {
        Window {
            v_idx: 0,
            h_idx: 0,
            area_v: (0, 0),
            area_v_all: (0, 0),
            area_h: (0, 0),
            area_h_all: (0, 0),
            area_v_org: (0, 0),
            area_h_org: (0, 0),
            cur: Cur::default(),
            cur_org: Cur::default(),
            updown_x: 0,
            row_posi: 0,
            row_len_org: 0,
            offset: Offset::default(),
            scrl_v: ScrollbarV::default(),
            scrl_h: ScrollbarH::default(),
            sel: SelRange::default(),
            sel_org: SelRange::default(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Window {
    pub v_idx: usize,
    pub h_idx: usize,
    pub area_h: (usize, usize),
    // area_h + rnw + scrollbar
    pub area_h_all: (usize, usize),
    pub area_h_org: (usize, usize),
    pub area_v: (usize, usize),
    // area_v + scrollbar
    pub area_v_all: (usize, usize),
    pub area_v_org: (usize, usize),
    /// current cursor position
    pub cur: Cur,
    pub cur_org: Cur,
    // Basic x position when moving the cursor up and down
    pub updown_x: usize,
    pub row_posi: usize,
    pub row_len_org: usize,
    pub offset: Offset,
    pub scrl_h: ScrollbarH,
    pub scrl_v: ScrollbarV,
    pub sel: SelRange,
    pub sel_org: SelRange,
}
