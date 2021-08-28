use crate::{cont::promptcont::PromptCont, ewin_core::_cfg::keys::KeyCmd, ewin_core::model::*, ewin_core::util::*};
use std::cmp::{max, min};

impl PromptCont {
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
    pub fn delete(&mut self, ep: &mut Proc) {
        if self.cur.x < self.buf.len() {
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
    }

    pub fn backspace(&mut self, ep: &mut Proc) {
        if self.cur.x > 0 {
            self.cur.x -= 1;
            self.cur.disp_x -= get_char_width_not_tab(&self.buf[self.cur.x]);
            ep.str = self.buf[self.cur.x].to_string();
            self.buf.remove(self.cur.x);
        }
    }
    pub fn cur_end(&mut self) {
        self.cur.x = self.buf.len();
        let (_, width) = get_row_width(&self.buf[..], 0, false);
        self.cur.disp_x = width;
    }

    pub fn cur_home(&mut self) {
        self.cur.x = 0;
        self.cur.disp_x = 0;
    }

    pub fn cur_move(&mut self) {
        match self.keycmd {
            KeyCmd::CursorLeft => self.cur_left(),
            KeyCmd::CursorRight => self.cur_right(),
            KeyCmd::CursorRowHome => self.cur_home(),
            KeyCmd::CursorRowEnd => self.cur_end(),
            _ => {}
        }
    }
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match self.keycmd {
            KeyCmd::CursorLeftSelect => self.cur_left(),
            KeyCmd::CursorRightSelect => self.cur_right(),
            KeyCmd::CursorRowHomeSelect => self.set_cur_target(0),
            KeyCmd::CursorRowEndSelect => self.set_cur_target(self.buf.len()),
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.sel.check_overlap();
    }
}
