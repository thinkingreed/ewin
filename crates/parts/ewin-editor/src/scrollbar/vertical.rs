use crate::model::*;
use ewin_cfg::log::*;
use ewin_key::key::cmd::*;
use std::cmp::min;

impl Editor {
    pub fn ctrl_scrl_v(&mut self) {
        Log::debug_key("Editor.ctrl_scrl_v");

        let view_y = self.get_curt_row_posi();
        let view_height = self.get_curt_row_len();
        self.win_mgr.curt_mut().scrl_v.ctrl_scrollbar_v(&self.cmd.cmd_type, view_y, view_height);
        self.set_cur_at_scrlbar_v_posi();

        self.scroll();
    }
    pub fn calc_scrlbar_v(&mut self) {
        Log::debug_key("calc_scrlbar_v");

        Log::debug(" self.scrl_v.row_posi before", &self.win_mgr.curt_mut().scrl_v.view.y);
        for vec_v in self.win_mgr.win_list.iter_mut() {
            for win in vec_v.iter_mut() {
                win.scrl_v.calc_scrlbar_v(&self.cmd.cmd_type, win.offset, win.height(), self.buf.len_rows());

                Log::debug(" self.scrl_v.row_posi after", &win.scrl_v.view.y);
            }
        }
    }

    pub fn set_cur_at_scrlbar_v_posi(&mut self) {
        Log::debug_key("set_cur_scrlbar_v");

        if self.is_move_cur_posi_scrolling_enable() {
            self.win_mgr.curt_mut().cur.y = if self.win_mgr.curt_mut().scrl_v.view.y == 0 {
                0
            } else if self.win_mgr.curt_mut().scrl_v.view.y + self.win_mgr.curt_mut().scrl_v.view.height == self.get_curt_row_len() {
                self.buf.len_rows() - 1
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) {
                min(self.win_mgr.curt_mut().cur.y + self.win_mgr.curt_mut().scrl_v.move_len, self.buf.len_rows() - 1)
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                if self.win_mgr.curt_mut().cur.y > self.win_mgr.curt_mut().scrl_v.move_len {
                    self.win_mgr.curt_mut().cur.y - self.win_mgr.curt_mut().scrl_v.move_len
                } else {
                    0
                }
            } else {
                self.win_mgr.curt_mut().cur.y
            };
            self.cur_updown_com();
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.win_mgr.curt_ref().cur.x, false);
        }
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
