use ewin_cfg::{log::Log, model::default::Cfg};

use crate::model::*;
use std::cmp::{max, min};

impl ScrollbarV {
    pub fn calc_com_scrlbar_v(&mut self, is_editor_scrlbar_v: bool, row_len: usize, cont_len: usize) {
        Log::debug_key("calc_com_scrlbar_v");
        Log::debug("row_len - 1", &(row_len - 1));
        Log::debug("(row_len as f64 / cont_len as f64 * row_len as f64).ceil()", &(row_len as f64 / cont_len as f64 * row_len as f64).ceil());

        let bar_len = max(1, min((row_len as f64 / cont_len as f64 * row_len as f64).ceil() as usize, row_len - 1));
        let scrl_range = row_len - bar_len;
        let move_len = if is_editor_scrlbar_v {
            if Cfg::get().general.editor.cursor.move_position_by_scrolling_enable {
                (cont_len as f64 / scrl_range as f64).ceil() as usize
            } else {
                ((cont_len - row_len) as f64 / scrl_range as f64).ceil() as usize
            }
            // input comple scrlbar_v
        } else {
            Log::debug("cont_len", &cont_len);
            Log::debug("area_len", &row_len);

            ((cont_len - row_len) as f64 / scrl_range as f64).ceil() as usize
        };
        self.bar_len = bar_len;
        self.move_len = move_len;
    }
}
