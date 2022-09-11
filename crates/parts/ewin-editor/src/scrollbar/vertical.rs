use crate::{model::*, window::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_key::key::cmd::*;
use std::cmp::min;

impl Editor {
    pub fn calc_editor_scrlbar_v(&mut self) {
        Log::debug_key("calc_scrlbar_v");

        Log::debug(" self.scrl_v.row_posi before", &self.win_mgr.curt().scrl_v.view.y);
        for vec_v in self.win_mgr.win_list.iter_mut() {
            for win in vec_v.iter_mut() {
                let is_calc_com = self.buf_len_rows_org != self.buf.len_rows() || win.row_len_org != win.height();
                win.scrl_v.calc_scrlbar_v(&self.cmd.cmd_type, win.offset, win.height(), self.buf.len_rows(), is_calc_com);

                Log::debug(" self.scrl_v.row_posi after", &win.scrl_v.view.y);
            }
        }
    }

    pub fn set_cur_at_scrlbar_v_posi(&mut self) {
        Log::debug_key("set_cur_scrlbar_v");

        if self.is_move_cur_posi_scrolling_enable() {
            self.win_mgr.curt().cur.y = if self.win_mgr.curt().scrl_v.view.y == 0 {
                0
            } else if self.win_mgr.curt().scrl_v.view.y + self.win_mgr.curt().scrl_v.bar_len == self.get_curt_row_len() {
                self.buf.len_rows() - 1
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) {
                min(self.win_mgr.curt().cur.y + self.win_mgr.curt().scrl_v.move_len, self.buf.len_rows() - 1)
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                if self.win_mgr.curt().cur.y > self.win_mgr.curt().scrl_v.move_len {
                    self.win_mgr.curt().cur.y - self.win_mgr.curt().scrl_v.move_len
                } else {
                    0
                }
            } else {
                self.win_mgr.curt().cur.y
            };
            self.cur_updown_com();
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.win_mgr.curt_ref().cur.x, false);
        }

        self.scroll();
    }

    pub fn draw_scrlbar_v(&self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("Editor.draw_scrlbar_v");
        Log::debug("win", &win);
        if win.scrl_v.is_show {
            for i in win.area_v.0..win.area_v.1 {
                str_vec.push(MoveTo(win.area_h.1 as u16, i as u16).to_string());
                str_vec.push(if win.area_v.0 + win.scrl_v.view.y <= i && i < win.area_v.0 + win.scrl_v.view.y + win.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                str_vec.push(" ".to_string().repeat(win.scrl_v.bar_width));
            }
        }
        str_vec.push(Colors::get_default_bg());
    }
}

#[cfg(test)]
mod tests {

    use ewin_cfg::model::general::default::*;

    use super::*;

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        //  let e = Editor::new();
    }
}
