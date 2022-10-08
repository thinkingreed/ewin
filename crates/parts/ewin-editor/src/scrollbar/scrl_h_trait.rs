use crate::model::*;
use ewin_view::scrollbar::{horizontal::*, scrl_h_trait::*};
use unicode_width::UnicodeWidthStr;

impl ScrlHTrait for Editor {
    fn get_row_chars(&mut self, idx: usize) -> usize {
        self.buf.line(idx).to_string().width()
    }

    fn get_row_width(&mut self, idx: usize) -> usize {
        self.buf.line(idx).len_chars()
    }

    fn get_scrl_h_info(&mut self) -> &mut ScrlHInfo {
        &mut self.win_mgr.scrl_h_info
    }

    fn get_vec_len(&mut self) -> usize {
        self.buf.len_rows()
    }

    fn get_row_max_chars(&mut self) -> usize {
        self.buf.char_vec_row(self.win_mgr.scrl_h_info.row_max_width_idx).len()
    }
}
