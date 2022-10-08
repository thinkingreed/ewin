use crate::sidebar::*;
use ewin_cfg::log::Log;
use ewin_view::scrollbar::{horizontal::*, scrl_h_trait::*};
use unicode_width::UnicodeWidthStr;

impl ScrlHTrait for SideBar {
    fn get_row_chars(&mut self, idx: usize) -> usize {
        self.cont.get_cont_vec()[idx].get_path(false).width()
    }

    fn get_row_width(&mut self, idx: usize) -> usize {
        self.cont.get_cont_vec()[idx].get_path(false).chars().count()
    }

    fn get_scrl_h_info(&mut self) -> &mut ScrlHInfo {
        &mut self.scrl_h_info
    }

    fn get_vec_len(&mut self) -> usize {
        self.cont.get_cont_vec_len()
    }

    fn get_row_max_chars(&mut self) -> usize {
        self.cont.get_cont_vec()[self.scrl_h_info.row_max_width_idx].get_path(false).chars().count()
    }
}
