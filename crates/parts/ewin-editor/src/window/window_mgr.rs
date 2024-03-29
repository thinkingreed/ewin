use super::window::*;
use crate::{model::*, window::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::{
    def::*,
    models::{model::*, view::*},
    term::*,
};
use ewin_state::term::*;
use ewin_view::scrollbar::horizontal::*;

impl WindowMgr {
    pub const SPLIT_LINE_V_WIDTH: usize = 1;
    pub const SPLIT_LINE_H_HEIGHT: usize = 1;

    pub fn split_window(&mut self, split_type: WindowSplitType) {
        if self.split_type != WindowSplitType::None {
            self.clear();
            State::get().curt_mut_state().editor.window_split_type = WindowSplitType::None
        } else {
            match split_type {
                WindowSplitType::Vertical => self.win_list.get_mut(0).unwrap().push(Window::default()),
                WindowSplitType::Horizontal => self.win_list.push(vec![Window::default()]),
                WindowSplitType::None => {}
            };
            self.split_type = split_type;
            State::get().curt_mut_state().editor.window_split_type = split_type;
        }
    }

    pub fn clear(&mut self) {
        self.win_list = vec![vec![Window::default()]];
        self.split_type = WindowSplitType::None;
        self.split_line_v = 0;
        self.split_line_h = 0;
        self.win_v_idx = 0;
        self.win_h_idx = 0;
    }

    pub fn curt_mut(&mut self) -> &mut Window {
        return self.win_list.get_mut(self.win_v_idx).unwrap().get_mut(self.win_h_idx).unwrap();
    }

    pub fn curt_ref(&self) -> &Window {
        return self.win_list.get(self.win_v_idx).unwrap().get(self.win_h_idx).unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowMgr {
    // Vertical, horizontal
    pub win_list: Vec<Vec<Window>>,
    pub win_v_idx: usize,
    pub win_h_idx: usize,
    pub split_type: WindowSplitType,
    pub split_type_org: WindowSplitType,
    pub split_line_v: usize,
    pub split_line_h: usize,
    pub scrl_h_info: ScrlHInfo,
}

impl Default for WindowMgr {
    fn default() -> Self {
        WindowMgr { win_list: vec![vec![Window::new()]], win_v_idx: 0, win_h_idx: 0, split_type: WindowSplitType::default(), split_type_org: WindowSplitType::default(), split_line_v: 0, split_line_h: 0, scrl_h_info: ScrlHInfo::default() }
    }
}

impl Editor {
    pub fn draw_window_split_line(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_window_split_line");
        if self.win_mgr.split_line_v > 0 {
            for i in self.view.y..self.view.y + self.view.height - 1 {
                #[allow(clippy::repeat_once)]
                str_vec.push(format!("{}{}{}", MoveTo(self.win_mgr.split_line_v as u16, i as u16), Colors::get_window_split_line_bg(), get_space(WINDOW_SPLIT_LINE_WIDTH)));
            }
        }
        if self.win_mgr.split_line_h > 0 {
            str_vec.push(format!("{}{}{}", MoveTo(0, self.win_mgr.split_line_h as u16), Colors::get_window_split_line_bg(), get_space(get_term_size().0)));
        }
    }
}
