use crate::{
    ewin_com::{_cfg::key::keycmd::*, colors::*, def::*, global::*, log::*},
    model::*,
};
use crossterm::cursor::MoveTo;
use std::cmp::{max, min};

impl Editor {
    pub fn set_scrlbar_v_posi(&mut self, y: usize) {
        self.set_cur_scrlbar_v(y);
        self.set_offset_y_move_row();
        self.scroll_horizontal();
    }

    pub fn set_cur_scrlbar_v(&mut self, y: usize) {
        Log::debug_key("set_cur_scrlbar_v");

        let scrl_range = self.row_disp_len - self.scrl_v.bar_len;
        let move_len = (self.buf.len_rows() as f64 / scrl_range as f64).ceil() as usize;

        // MouseDownLeft
        if matches!(self.e_cmd, E_Cmd::MouseDownLeft(_, _)) {
            self.scrl_v.is_enable = true;
            // Except on scrl_v
            if !(self.row_posi + self.scrl_v.row_posi <= y && y < self.row_posi + self.scrl_v.row_posi + self.scrl_v.bar_len) {
                self.scrl_v.row_posi = if y + self.scrl_v.bar_len > self.row_posi + self.row_disp_len - 1 { self.row_posi + self.row_disp_len - 1 - self.scrl_v.bar_len } else { y - self.row_posi };
            } else {
                return;
            }
            // MouseDragLeftDown・MouseDragLeftUp
        } else if self.scrl_v.is_enable {
            if matches!(self.e_cmd, E_Cmd::MouseDragLeftDown(_, _)) && self.row_disp_len >= self.scrl_v.row_posi + self.scrl_v.bar_len {
                self.scrl_v.row_posi = if self.scrl_v.row_posi + self.scrl_v.bar_len >= self.row_disp_len { self.scrl_v.row_posi } else { self.scrl_v.row_posi + 1 };
            } else if (matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftLeft(_, _)) || matches!(self.e_cmd, E_Cmd::MouseDragLeftRight(_, _))) && self.row_posi <= self.scrl_v.row_posi {
                Log::debug("row_posi 111", &self.scrl_v.row_posi);

                self.scrl_v.row_posi = if self.scrl_v.row_posi >= self.row_posi { self.scrl_v.row_posi - 1 } else { self.row_posi };

                Log::debug("row_posi 222", &self.scrl_v.row_posi);
            }
        }
        self.cur.y = min(self.scrl_v.row_posi * move_len, self.buf.len_rows() - 1);
        self.cur_updown_com();
        self.set_cur_target(self.cur.y, self.cur.x, false);
    }

    pub fn draw_scrlbar_v(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("Editor.draw_scrlbar_v");

        if self.scrl_v.is_show {
            if self.scrl_v.bar_len == USIZE_UNDEFINED || self.row_len_org != self.buf.len_rows() {
                self.scrl_v.bar_len = max(1, (self.row_disp_len as f64 / self.buf.len_rows() as f64 * self.row_disp_len as f64) as usize);
            }
            self.scrl_v.row_posi = match self.e_cmd {
                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) => self.scrl_v.row_posi,
                _ => (self.cur.y as f64 / (self.buf.len_rows() - 1) as f64 * (self.row_disp_len - self.scrl_v.bar_len) as f64) as usize,
            };

            for i in self.row_posi..=self.row_posi + self.row_disp_len {
                str_vec.push(MoveTo((self.get_rnw_and_margin() + self.col_len) as u16, i as u16).to_string());
                str_vec.push(if self.row_posi + self.scrl_v.row_posi <= i && i < self.row_posi + self.scrl_v.row_posi + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                str_vec.push(" ".to_string().repeat(CFG.get().unwrap().try_lock().unwrap().general.editor.scrollbar.vertical.width));
            }
            str_vec.push(Colors::get_default_bg());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ewin_com::_cfg::cfg::*;

    #[test]
    fn test_editor_proc_base_edit() {
        Log::set_logger(&Some(CfgLog { level: Some("test".to_string()) }));
        let mut e = Editor::new();
        e.buf.insert_end(&EOF_MARK.to_string());
    }
}
