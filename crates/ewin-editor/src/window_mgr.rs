use crate::model::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_com::model::*;
use ewin_const::def::WINDOW_SPLIT_LINE_WIDTH;

impl WindowMgr {
    pub fn split_window(&mut self, split_type: &WindowSplitType) {
        if self.split_type != WindowSplitType::None {
            self.clear();
        } else {
            match split_type {
                WindowSplitType::Vertical => self.win_list.get_mut(0).unwrap().push(Window::default()),
                WindowSplitType::Horizontal => {}
                WindowSplitType::None => {}
            };
            self.split_type = *split_type;
        }
    }
    pub fn clear(&mut self) {
        self.win_list[0] = vec![Window::default()];
        self.split_type = WindowSplitType::None;
        self.split_line_v = 0;
        self.split_line_h = 0;
    }
}

impl Editor {
    pub fn draw_window_split_line(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_window_split_line");
        if self.win_mgr.split_line_v > 0 {
            Log::debug("self.row_posi", &self.row_posi);
            for i in self.row_posi..self.row_posi + self.row_num {
                str_vec.push(format!("{}{}{}", MoveTo(self.win_mgr.split_line_v as u16, i as u16), Colors::get_window_split_line_bg(), " ".repeat(WINDOW_SPLIT_LINE_WIDTH)));
            }
        }
    }

    pub fn get_editor_row_posi(&self) -> usize {
        return self.row_posi;
    }

    pub fn get_curt_ref_win(&self) -> &Window {
        return self.win_mgr.curt_ref();
    }

    pub fn get_curt_col_len(&self) -> usize {
        return self.win_mgr.curt_ref().area_h.1 - self.win_mgr.curt_ref().area_h.0;
    }

    pub fn get_curt_row_posi(&self) -> usize {
        return self.win_mgr.curt_ref().area_v.0;
    }

    pub fn get_curt_row_len(&self) -> usize {
        return self.win_mgr.curt_ref().area_v.1 - self.win_mgr.curt_ref().area_v.0;
    }
}
