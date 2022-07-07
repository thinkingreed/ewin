use crate::model::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_com::_cfg::key::cmd::*;
use ewin_const::def::*;
use std::cmp::min;

impl Editor {
    pub fn calc_editor_scrlbar_v(&mut self) {
        Log::debug_key("calc_scrlbar_v");
        Log::debug(" self.scrl_v.row_posi before", &self.win_mgr.curt().scrl_v.row_posi);
        for vec_v in self.win_mgr.win_list.iter_mut() {
            for win in vec_v.iter_mut() {
                Log::debug("window", &win);
                if win.scrl_v.is_show {
                    Log::debug("win.area_v", &win.area_v);
                    if win.scrl_v.bar_len == USIZE_UNDEFINED || self.buf_len_rows_org != self.buf.len_rows() || win.row_len_org != win.area_v.1 - win.area_v.0 {
                        let row_len = win.area_v.1 - win.area_v.0;
                        win.scrl_v.calc_com_scrlbar_v(true, row_len, self.buf.len_rows());
                    }
                    Log::debug("self.cmd", &self.cmd);
                    win.scrl_v.row_posi = match self.cmd.cmd_type {
                        CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) if win.scrl_v.is_enable => win.scrl_v.row_posi,
                        _ => (win.offset.y as f64 / win.scrl_v.move_len as f64).ceil() as usize,
                    };
                    Log::debug(" self.scrl_v.row_posi after", &win.scrl_v.row_posi);
                }
            }
        }
    }

    pub fn set_scrlbar_v_posi(&mut self, y: usize) {
        Log::debug_key("set_cur_scrlbar_v");
        Log::debug("self.scrl_v.row_posi 111", &self.win_mgr.curt().scrl_v.row_posi);
        Log::debug("self.scrl_v.bar_len 111", &self.win_mgr.curt().scrl_v.bar_len);

        // MouseDownLeft
        if matches!(self.cmd.cmd_type, CmdType::MouseDownLeft(_, _)) {
            self.win_mgr.curt().scrl_v.is_enable = true;
            // Except on scrl_v
            if !(self.get_curt_row_posi() + self.win_mgr.curt().scrl_v.row_posi <= y && y < self.get_curt_row_posi() + self.win_mgr.curt().scrl_v.row_posi + self.win_mgr.curt().scrl_v.bar_len) {
                self.win_mgr.curt().scrl_v.row_posi = if y + self.win_mgr.curt().scrl_v.bar_len > self.get_curt_row_posi() + self.get_curt_row_len() - 1 { self.get_curt_row_posi() + self.get_curt_row_len() - 1 - self.win_mgr.curt().scrl_v.bar_len } else { y - self.get_curt_row_posi() };
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if self.win_mgr.curt().scrl_v.is_enable {
            if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) && self.get_curt_row_len() >= self.win_mgr.curt().scrl_v.row_posi + self.win_mgr.curt().scrl_v.bar_len {
                self.win_mgr.curt().scrl_v.row_posi = if self.win_mgr.curt().scrl_v.row_posi + self.win_mgr.curt().scrl_v.bar_len >= self.get_curt_row_len() { self.win_mgr.curt().scrl_v.row_posi } else { self.win_mgr.curt().scrl_v.row_posi + 1 };
            } else if (matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _))) && self.get_curt_row_posi() <= y && y < self.get_curt_row_posi() + self.get_curt_row_len() {
                self.win_mgr.curt().scrl_v.row_posi = if self.win_mgr.curt().scrl_v.row_posi == 0 { self.win_mgr.curt().scrl_v.row_posi } else { self.win_mgr.curt().scrl_v.row_posi - 1 };
            }
        }
        if self.is_move_cur_posi_scrolling_enable() {
            self.win_mgr.curt().cur.y = if self.win_mgr.curt().scrl_v.row_posi == 0 {
                0
            } else if self.win_mgr.curt().scrl_v.row_posi + self.win_mgr.curt().scrl_v.bar_len == self.get_curt_row_len() {
                self.buf.len_rows() - 1
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftDown(_, _)) {
                min(self.win_mgr.curt().cur.y + self.win_mgr.curt().scrl_v.move_len, self.buf.len_rows() - 1)
            } else if matches!(self.cmd.cmd_type, CmdType::MouseDragLeftUp(_, _)) {
                self.win_mgr.curt().cur.y - self.win_mgr.curt().scrl_v.move_len
            } else {
                self.win_mgr.curt().cur.y
            };
            self.cur_updown_com();
            self.set_cur_target_by_x(self.win_mgr.curt_ref().cur.y, self.win_mgr.curt_ref().cur.x, false);
        }
        Log::debug("self.scrl_v.row_posi 222", &self.win_mgr.curt().scrl_v.row_posi);
        Log::debug("self.scrl_v.bar_len 222", &self.win_mgr.curt().scrl_v.bar_len);

        self.scroll();
    }

    pub fn draw_scrlbar_v(&mut self, str_vec: &mut Vec<String>, win: &Window) {
        Log::debug_key("Editor.draw_scrlbar_v");
        if win.scrl_v.is_show {
            Log::debug("window.area_h", &win.area_h);
            Log::debug("win.scrl_v", &win.scrl_v);
            for i in win.area_v.0..win.area_v.1 {
                Log::debug("iii", &i);
                str_vec.push(MoveTo(win.area_h.1 as u16, i as u16).to_string());
                str_vec.push(if win.area_v.0 + win.scrl_v.row_posi <= i && i < win.area_v.0 + win.scrl_v.row_posi + win.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                str_vec.push(" ".to_string().repeat(win.scrl_v.bar_width));
            }
        }
        str_vec.push(Colors::get_default_bg());
    }
}

#[cfg(test)]
mod tests {

    use ewin_cfg::model::default::CfgLog;

    use super::*;

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&CfgLog { level: "test".to_string() });
        //  let e = Editor::new();
    }
}
