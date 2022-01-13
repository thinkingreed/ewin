use crate::{
    ewin_com::{_cfg::key::keycmd::*, colors::*, def::*, log::*},
    model::*,
};
use crossterm::cursor::MoveTo;
use ewin_com::_cfg::cfg::Cfg;
use std::cmp::max;

impl Editor {
    pub fn calc_scrlbar_v(&mut self) {
        if self.scrl_v.is_show {
            if self.scrl_v.bar_len == USIZE_UNDEFINED || self.row_len_org != self.buf.len_rows() {
                self.scrl_v.bar_len = max(1, (self.row_disp_len as f64 / self.buf.len_rows() as f64 * self.row_disp_len as f64).ceil() as usize);
                Log::debug("Editor.draw_scrlbar_v", &self.scrl_v.bar_len);
                let scrl_range = self.row_disp_len - self.scrl_v.bar_len;
                Log::debug("scrl_range", &scrl_range);
                self.scrl_v.move_len = ((self.buf.len_rows() - self.row_disp_len) as f64 / scrl_range as f64).ceil() as usize;
                Log::debug("move_len", &self.scrl_v.move_len);
            }
            self.scrl_v.row_posi = match self.e_cmd {
                E_Cmd::MouseDownLeft(_, _) | E_Cmd::MouseDragLeftUp(_, _) | E_Cmd::MouseDragLeftDown(_, _) | E_Cmd::MouseDragLeftLeft(_, _) | E_Cmd::MouseDragLeftRight(_, _) if self.scrl_v.is_enable => self.scrl_v.row_posi,
                _ => {
                    if self.is_cur_y_in_screen() {
                        (self.cur.y as f64 / (self.buf.len_rows() - 1) as f64 * (self.row_disp_len - self.scrl_v.bar_len) as f64) as usize
                    } else {
                        (self.offset_y as f64 / self.scrl_v.move_len as f64).ceil() as usize
                    }
                }
            };
        }
    }
    pub fn set_scrlbar_v_posi(&mut self, y: usize) {
        Log::debug_key("set_cur_scrlbar_v");

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
            } else if (matches!(self.e_cmd, E_Cmd::MouseDragLeftUp(_, _))) && self.row_posi <= y && y < self.row_posi + self.row_disp_len {
                self.scrl_v.row_posi = if self.scrl_v.row_posi == 0 { self.scrl_v.row_posi } else { self.scrl_v.row_posi - 1 };
            }
        }
        self.cur_updown_com();
        if self.is_move_cur_posi_scrolling_enable() {
            self.set_cur_target(self.cur.y, self.cur.x, false);
        }
        self.scroll();
    }

    pub fn draw_scrlbar_v(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("Editor.draw_scrlbar_v");

        if self.scrl_v.is_show {
            self.calc_scrlbar_v();

            Log::debug("self.scrl_v.row_posi 111", &self.scrl_v.row_posi);
            Log::debug("self.scrl_v.row_posi 222", &self.scrl_v.row_posi);
            for i in self.row_posi..=self.row_posi + self.row_disp_len {
                str_vec.push(MoveTo((self.get_rnw_and_margin() + self.col_len) as u16, i as u16).to_string());
                //  str_vec.push(if self.row_posi + self.scrl_v.row_posi <= i && i < self.row_posi + self.scrl_v.row_posi + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                // let posi = (self.offset_y as f64 / self.scrl_v.move_len as f64).ceil() as usize;
                str_vec.push(if self.row_posi + self.scrl_v.row_posi <= i && i < self.row_posi + self.scrl_v.row_posi + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                str_vec.push(" ".to_string().repeat(Cfg::get().general.editor.scrollbar.vertical.width));
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
        //  let e = Editor::new();
    }
}
