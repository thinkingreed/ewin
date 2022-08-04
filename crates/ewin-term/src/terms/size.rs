use super::term::*;
use crate::{bar::filebar::*, ewin_editor::window_mgr::*, global_term::*, help::*};
use ewin_cfg::{log::*, model::default::*};
use ewin_const::{def::*, term::*};
use ewin_key::model::*;
use ewin_state::tabs::*;
use std::usize;

impl Term {
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));
        Log::debug("self.curt().state", &self.curt().state);

        self.menubar.set_posi(cols);
        self.menubar.set_menunm();

        self.fbar.set_posi(cols);
        FileBar::set_filenm(self);

        let mut hlep = HELP_DISP.get().unwrap().try_lock().unwrap();

        hlep.col_num = cols;
        hlep.row_num = if hlep.is_disp { Help::DISP_ROW_NUM } else { 0 };
        hlep.row_posi = if hlep.is_disp { rows - hlep.row_num } else { 0 };

        let help_disp_row_num = if hlep.row_num > 0 { hlep.row_num + 1 } else { 0 };
        self.curt().sbar.row_posi = if help_disp_row_num == 0 { rows - 1 } else { rows - help_disp_row_num };
        self.curt().sbar.col_num = cols;
        self.curt().prom.col_num = cols;

        self.curt().prom.row_posi = rows - self.curt().prom.row_num - hlep.row_num - self.curt().sbar.row_num;
        self.curt().msgbar.col_num = cols;
        self.curt().msgbar.row_num = MSGBAR_ROW_NUM;

        self.curt().msgbar.row_posi = rows - self.curt().prom.row_num - hlep.row_num - self.curt().sbar.row_num - 1;

        if self.curt().state.prom == PromState::OpenFile {
            self.menubar.row_num = 0;
            self.curt().editor.row_num = 0;
        } else {
            self.menubar.row_num = MSGBAR_ROW_NUM;
            let scale_row_num = if Tabs::get().curt_state().editor.scale.is_enable { SCALE_ROW_NUM } else { 0 };
            Log::debug("scale_row_num", &scale_row_num);
            Log::debug(" self.curt().menubar.row_num", &self.menubar.row_num);
            Log::debug(" self.fbar.row_num", &self.fbar.row_num);
            Log::debug(" self.curt().msgbar.row_num", &self.curt().msgbar.row_num);
            Log::debug(" self.curt().prom.row_num", &self.curt().prom.row_num);
            Log::debug(" self.curt().sbar.row_num", &self.curt().sbar.row_num);
            let editor_row = rows - self.menubar.row_num - self.fbar.row_num - self.curt().prom.row_num - self.curt().msgbar.row_num - hlep.row_num - self.curt().sbar.row_num;
            self.set_size_editor(cols, editor_row, scale_row_num);
        }

        return true;
    }

    pub fn set_size_editor(&mut self, cols: usize, editor_row: usize, scale_row_num: usize) {
        Log::debug_key("set_size_editor");
        Log::debug("editor_row", &editor_row);
        Log::debug("Tabs::get().idx", &Tabs::get().idx);
        Log::debug("Tabs::get().curt_state().editor", &Tabs::get().curt_state().editor);

        let rnw_and_margin = self.curt().editor.get_rnw_and_margin();
        let row_posi = MENUBAR_ROW_NUM + FILEBAR_ROW_NUM + scale_row_num;
        self.curt().editor.row_posi = row_posi;
        Log::debug("self.curt().editor.row_posi", &self.curt().editor.row_posi);

        self.curt().editor.row_num = editor_row - 1;
        Log::debug("self.curt().editor.row_num", &self.curt().editor.row_num);

        let editor_tgt_row = editor_row - ((self.curt().editor.win_mgr.win_list.len() - 1) * WindowMgr::SPLIT_LINE_H_HEIGHT) - self.curt().editor.win_mgr.win_list.len() * scale_row_num;
        Log::debug("editor_tgt_row", &editor_tgt_row);

        let mut v_posi = row_posi;
        let editor_buf_len_rows = self.curt().editor.buf.len_rows();
        let is_tab_state_normal = self.curt().state.is_nomal();

        let win_height_base = editor_tgt_row / self.curt().editor.win_mgr.win_list.len();
        let win_height_rest = editor_tgt_row % self.curt().editor.win_mgr.win_list.len();

        let win_v_list_len = self.curt().editor.win_mgr.win_list.get(0).unwrap().len();

        let tgt_h_len = cols - win_v_list_len * rnw_and_margin - (win_v_list_len - 1) * WindowMgr::SPLIT_LINE_V_WIDTH;
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
        let row_max_width = self.curt().editor.win_mgr.row_max_width;

        for (v_idx, vec_v) in self.curt().editor.win_mgr.win_list.iter_mut().enumerate() {
            let mut win_height = win_height_base;
            if v_idx == 0 {
                win_height += win_height_rest;
                Log::debug("win_height 111", &win_height);
            } else {
                v_posi += WindowMgr::SPLIT_LINE_H_HEIGHT;
                split_line_h = v_posi;
                v_posi += SCALE_ROW_NUM + 1;
            }

            let mut h_posi = 0;

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
                if row_max_width > win_width && is_tab_state_normal {
                    win.scrl_h.is_show = true;
                    win.scrl_h.bar_height = scrl_h_bar_height;
                    if h_idx == 0 {
                        win_height -= win.scrl_h.bar_height;
                        Log::debug("win_height 222", &win_height);
                    }
                } else {
                    win.scrl_h.is_show = false;
                }

                // scrl_v
                if win_height < editor_buf_len_rows {
                    win.scrl_v.is_show = true;
                    win.scrl_v.bar_width = Cfg::get().general.editor.scrollbar.vertical.width;
                    win_width -= win.scrl_v.bar_width;
                } else {
                    win.scrl_v.clear();
                }

                win.area_v = (v_posi, v_posi + win_height);
                Log::debug("win.area_v", &win.area_v);

                win.area_v_all = (v_posi - scale_row_num, v_posi + win_height + win.scrl_h.bar_height);
                Log::debug("win.area_all_v", &win.area_v_all);
                win.area_h = (h_posi, h_posi + win_width);
                if row_max_width > win_width && is_tab_state_normal {
                    win.scrl_h.row_posi = win.area_v.1;
                }
                win.area_h_all = (h_posi - rnw_and_margin, h_posi + win_width + win.scrl_v.bar_width);

                h_posi += win_width;
                if win.scrl_v.is_show {
                    h_posi += win.scrl_v.bar_width;
                }
            }
            v_posi += win_height - 1;
        }

        if split_line_v > 0 {
            self.curt().editor.win_mgr.split_line_v = split_line_v;
        }
        if split_line_h > 0 {
            self.curt().editor.win_mgr.split_line_h = split_line_h;
        }
    }
}
