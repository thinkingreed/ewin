use super::parts::input_area::*;
use crate::ewin_com::{_cfg::key::keycmd::*, util::*};
use ewin_cfg::log::*;
use std::cmp::{max, min};

impl PromContInputArea {
    pub fn cur_left(&mut self) {
        if self.cur.x != 0 {
            self.cur.x = max(self.cur.x - 1, 0);
            self.cur.disp_x -= get_char_width_not_tab(&self.buf[self.cur.x]);
        }
    }
    pub fn cur_right(&mut self) {
        if self.cur.x < self.buf.len() {
            self.cur.disp_x += get_char_width_not_tab(&self.buf[self.cur.x]);
            self.cur.x = min(self.cur.x + 1, self.buf.len());
        }
    }
    pub fn cur_end(&mut self) {
        self.cur.x = self.buf.len();
        let (_, width) = get_row_cur_x_disp_x(&self.buf[..], 0, false);
        self.cur.disp_x = width;
    }

    pub fn cur_home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
    }

    pub fn cur_move(&mut self) {
        match self.base.p_cmd {
            P_Cmd::CursorLeft => self.cur_left(),
            P_Cmd::CursorRight => self.cur_right(),
            P_Cmd::CursorRowHome => self.cur_home(),
            P_Cmd::CursorRowEnd => self.cur_end(),
            _ => {}
        }
    }
    pub fn shift_move_com(&mut self) {
        Log::debug_key("shift_move_com");
        Log::debug("self.cur", &self.cur);
        self.sel.set_sel_posi(true, self.cur);
        Log::debug("self.sel", &self.sel);

        match self.base.p_cmd {
            P_Cmd::CursorLeftSelect => self.cur_left(),
            P_Cmd::CursorRightSelect => self.cur_right(),
            P_Cmd::CursorRowHomeSelect => self.set_cur_target(0),
            P_Cmd::CursorRowEndSelect => self.set_cur_target(self.buf.len()),
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur);
        self.sel.check_overlap();

        Log::debug("self.sel", &self.sel);
    }
}
