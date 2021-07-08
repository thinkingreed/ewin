use crate::{_cfg::keys::KeyCmd, prompt::cont::promptcont::*, util::*};

impl PromptCont {
    pub fn shift_move_com(&mut self) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match self.keycmd {
            KeyCmd::CursorLeftSelect => self.cur_left(),
            KeyCmd::CursorRightSelect => self.cur_right(),
            KeyCmd::CursorRowHomeSelect => {
                self.cur.x = 0;
                self.cur.disp_x = 0;
            }
            KeyCmd::CursorRowEndSelect => {
                let (cur_x, width) = get_row_width(&self.buf[..], 0, false);
                self.cur.x = cur_x;
                self.cur.disp_x = width;
            }
            _ => {}
        }

        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.sel.check_overlap();
    }
}
