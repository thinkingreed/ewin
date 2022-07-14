use crate::window::*;
use ewin_com::model::*;

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

        /*
        for vec_v in self.win_list.iter_mut() {
            for win in vec_v.iter_mut() {
                win.scrl_h.bar_len = 0;
            }
        }
         */
    }
}

#[derive(Debug, Clone)]
pub struct WindowMgr {
    // Vertical, horizontal
    pub win_list: Vec<Vec<Window>>,
    pub win_v_idx: usize,
    pub win_h_idx: usize,
    pub split_type: WindowSplitType,
    pub split_line_v: usize,
    pub split_line_h: usize,

    pub row_max_width_idx: usize,

    pub row_max_width: usize,
    pub row_max_width_org: usize,

    pub row_max_chars: usize,
    pub row_width_chars_vec: Vec<(usize, usize)>,
}

impl Default for WindowMgr {
    fn default() -> Self {
        WindowMgr { win_list: vec![vec![Window::new()]], win_v_idx: 0, win_h_idx: 0, split_type: WindowSplitType::default(), split_line_v: 0, split_line_h: 0, row_max_width_idx: 0, row_max_width: 0, row_max_width_org: 0, row_max_chars: 0, row_width_chars_vec: vec![] }
    }
}

impl WindowMgr {
    pub const SPLIT_LINE_V_WIDTH: usize = 1;
    pub fn curt(&mut self) -> &mut Window {
        return self.win_list.get_mut(self.win_v_idx).unwrap().get_mut(self.win_h_idx).unwrap();
    }
    pub fn curt_ref(&self) -> &Window {
        return self.win_list.get(self.win_v_idx).unwrap().get(self.win_h_idx).unwrap();
    }
}
