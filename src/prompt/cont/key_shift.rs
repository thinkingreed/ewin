use crate::{_cfg::keys::KeyCmd, prompt::cont::promptcont::*};

impl PromptCont {
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
