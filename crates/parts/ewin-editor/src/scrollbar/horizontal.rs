use crate::model::*;
use ewin_cfg::log::*;
use ewin_view::scrollbar::{horizontal::*, scrl_h_trait::*};
use std::collections::BTreeSet;

impl Editor {
    pub fn ctrl_scrl_h(&mut self) {
        Log::debug_key("Editor.ctrl_scrl_h");
        let view_x = self.get_curt_col_posi() + self.get_rnw_and_margin();
        let view_width = self.get_curt_col_len();
        self.win_mgr.curt_mut().scrl_h.ctrl_scrollbar_h(&self.cmd.cmd_type, view_x, view_width);

        self.scroll_horizontal();
    }
    pub fn calc_scrlbar_h(&mut self) {
        Log::debug_key("Editor.calc_editor_scrlbar_h");

        self.win_mgr.curt_mut().scrl_h.is_show = self.win_mgr.scrl_h_info.row_max_width > self.get_curt_col_len();
        if self.win_mgr.curt_mut().scrl_h.is_show {
            for vec_v in self.win_mgr.win_list.iter_mut() {
                for win in vec_v.iter_mut() {
                    Log::debug("win.width()", &win.width());

                    win.scrl_h.calc_scrlbar_h(win.width(), &self.win_mgr.scrl_h_info, win.offset.disp_x);
                }
            }
        }
    }

    pub fn set_row_width_chars_vec(&mut self, idxs: BTreeSet<usize>) {
        Log::debug_key("recalc_scrlbar_h");
        for i in idxs {
            if self.get_scrl_h_info().row_width_chars_vec.get(i).is_some() {
                self.get_scrl_h_info().row_width_chars_vec[i] = (self.get_row_width(i) + ScrollbarH::SCROLL_BAR_H_END_LINE_MARGIN, self.get_row_chars(i) + ScrollbarH::SCROLL_BAR_H_END_LINE_MARGIN);
            }
        }
    }
}

#[cfg(test)]
mod tests {}
