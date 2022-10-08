use crate::model::*;
use crate::window::window_mgr::*;
use ewin_activity_bar::activitybar::*;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{def::*, term::*};
use ewin_file_bar::filebar::*;
use ewin_help::help::*;
use ewin_key::model::*;
use ewin_prom::model::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for Editor {
    fn view(&self) -> &View {
        &self.view
    }

    fn set_size(&mut self) {
        Log::debug_key("Editor.set_size");

        let (cols, rows) = get_term_size();
        let scale_row_num = if State::get().curt_ref_state().editor.scale.is_enable { SCALE_HEIGHT } else { 0 };
        let editor_row = rows - if State::get().curt_ref_state().prom == PromState::OpenFile { 0 } else { MSGBAR_HEIGHT } - FileBar::get().view.height - Prom::get().view.height - MSGBAR_HEIGHT - Help::get().view.height - STATUSBAR_HEIGHT;

        let tabs_cols = cols - ActivityBar::get().get_width() - SideBar::get().get_width_all();
        Log::debug("tabs_cols", &tabs_cols);

        let rnw_and_margin = self.get_rnw_and_margin();
        let row_posi = MENUBAR_HEIGHT + FILEBAR_HEIGHT + scale_row_num;
        self.view.y = row_posi;

        let activity_bar_width = ActivityBar::get().get_width();
        let side_bar_width = SideBar::get().get_width_all();
        Log::debug("side_bar_width", &side_bar_width);
        self.view.x = activity_bar_width + side_bar_width;
        Log::debug(" self.view.y", &self.view.y);

        self.view.height = editor_row;
        self.view.width = tabs_cols;

        let editor_tgt_row = editor_row - ((self.win_mgr.win_list.len() - 1) * WindowMgr::SPLIT_LINE_H_HEIGHT) - self.win_mgr.win_list.len() * scale_row_num;
        Log::debug("editor_tgt_row", &editor_tgt_row);

        let mut v_posi = row_posi;
        let editor_buf_len_rows = self.buf.len_rows();
        // let is_tab_state_normal = self.curt().state.is_nomal();

        let win_height_base = editor_tgt_row / self.win_mgr.win_list.len();
        let win_height_rest = editor_tgt_row % self.win_mgr.win_list.len();

        let win_v_list_len = self.win_mgr.win_list.get(0).unwrap().len();

        let tgt_h_len = tabs_cols - win_v_list_len * rnw_and_margin - (win_v_list_len - 1) * WindowMgr::SPLIT_LINE_V_WIDTH;
        Log::debug("tgt_h_len", &tgt_h_len);
        let win_width_base = tgt_h_len / win_v_list_len;
        let win_width_rest = tgt_h_len % win_v_list_len;

        Log::debug("window_width_base", &win_width_base);
        Log::debug("window_width_rest", &win_width_rest);
        Log::debug("window_height_base", &win_height_base);
        Log::debug("win_height_rest", &win_height_rest);
        let mut split_line_v = 0;
        let mut split_line_h = 0;
        let scrl_h_bar_height = Cfg::get().general.editor.scrollbar.horizontal.height;
        let row_max_width = self.win_mgr.scrl_h_info.row_max_width;

        for (v_idx, vec_v) in self.win_mgr.win_list.iter_mut().enumerate() {
            let mut win_height = win_height_base;
            if v_idx == 0 {
                win_height += win_height_rest;
                Log::debug("win_height 111", &win_height);
            } else {
                v_posi += WindowMgr::SPLIT_LINE_H_HEIGHT;
                split_line_h = v_posi;
                v_posi += SCALE_HEIGHT + 1;
            }

            let mut h_posi = ActivityBar::get().get_width() + SideBar::get().get_width_all();

            for (h_idx, win) in vec_v.iter_mut().enumerate() {
                win.v_idx = v_idx;
                win.h_idx = h_idx;
                let mut win_width = win_width_base;
                if h_idx == 0 {
                    win_width += win_width_rest;
                } else {
                    split_line_v = h_posi;
                    h_posi += WindowMgr::SPLIT_LINE_V_WIDTH;
                }
                h_posi += rnw_and_margin;

                // scrl_h
                // if row_max_width > win_width && is_tab_state_normal {
                if row_max_width > win_width {
                    win.scrl_h.is_show = true;
                    win.scrl_h.view.height = scrl_h_bar_height;
                    if h_idx == 0 {
                        win_height -= win.scrl_h.view.height;
                    }
                } else {
                    win.scrl_h.is_show = false;
                }

                // scrl_v
                if win_height < editor_buf_len_rows {
                    win.scrl_v.is_show = true;
                    win.scrl_v.view.width = Cfg::get().general.editor.scrollbar.vertical.width;
                    win.scrl_v.view.x = win.view.x_width();
                    win_width -= win.scrl_v.view.width;
                } else {
                    win.scrl_v.clear();
                }

                win.view.y = v_posi;
                win.view.height = win_height;

                // win.area_v_all = (v_posi - scale_row_num, v_posi + win_height + win.scrl_h.bar_height);

                win.view_all.y = v_posi - scale_row_num;
                win.view_all.height = v_posi + win_height + win.scrl_h.view.height - (v_posi - scale_row_num);

                win.view.x = h_posi;
                win.view.width = win_width;

                //  if row_max_width > win_width && is_tab_state_normal {
                if row_max_width > win_width {
                    win.scrl_h.view.y = win.view.y_height();
                }
                // win.area_h_all = (h_posi - rnw_and_margin, h_posi + win_width + win.scrl_v.bar_width);

                win.view_all.x = h_posi - rnw_and_margin;
                win.view_all.width = h_posi + win_width + win.scrl_v.view.width - (h_posi - rnw_and_margin);
                h_posi += win_width;
                if win.scrl_v.is_show {
                    h_posi += win.scrl_v.view.width;
                }
            }
            v_posi += win_height - 1;
        }

        if split_line_v > 0 {
            self.win_mgr.split_line_v = split_line_v;
        }
        if split_line_h > 0 {
            self.win_mgr.split_line_h = split_line_h;
        }

        self.calc_scrlbar();
    }
}
