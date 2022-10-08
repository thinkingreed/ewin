use crossterm::cursor::MoveTo;
use ewin_cfg::log::*;
use ewin_const::models::view::*;
use ewin_key::{cur::*, sel_range::*};
use ewin_view::{
    model::*,
    scrollbar::{horizontal::*, vertical::*},
    view::*,
};

impl Window {
    pub fn clear_draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("Window.clear_all_win");
        for i in self.view.y..=self.view.y_height() {
            str_vec.push(format!("{}{}", MoveTo(self.view_all.x as u16, i as u16), get_space(self.width_all())));
        }
        str_vec.push(format!("{}", MoveTo(self.view.x as u16, self.view.y as u16)));
    }

    pub fn width(&self) -> usize {
        return self.view.x_width() - self.view.x;
    }

    pub fn height(&self) -> usize {
        return self.view.y_height() - self.view.y;
    }

    pub fn width_all(&self) -> usize {
        return self.view_all.x_width() - self.view_all.x;
    }

    pub fn new() -> Self {
        Window {
            v_idx: 0,
            h_idx: 0,
            view: View::default(),
            view_all: View::default(),
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
    pub view: View,
    // view + scrollbar
    pub view_all: View,
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
