use crate::model::*;
use crate::util::*;

impl PromptCont {
    fn shift_move_com(&mut self, do_type: DoType) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match do_type {
            DoType::ShiftRight => self.cur_right(),
            DoType::ShiftLeft => self.cur_left(),
            DoType::ShiftHome => {
                self.cur.x = 0;
                self.cur.disp_x = 1;
            }
            DoType::ShiftEnd => {
                let (cur_x, width) = get_row_width(&self.buf[..], false);
                self.cur.x = cur_x;
                self.cur.disp_x = width + 1;
            }
            _ => {}
        }
        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.sel.check_sel_overlap();
    }

    pub fn shift_home(&mut self) {
        Log::ep_s("　　　　　　　　shift_home");

        self.shift_move_com(DoType::ShiftHome);
    }
    pub fn shift_end(&mut self) {
        Log::ep_s("　　　　　　　  shift_end");

        self.shift_move_com(DoType::ShiftEnd);
    }

    pub fn shift_right(&mut self) {
        Log::ep_s("　　　　　　　  shift_right");
        self.shift_move_com(DoType::ShiftRight);
    }

    pub fn shift_left(&mut self) {
        Log::ep_s("　　　　　　　  shift_left");
        self.shift_move_com(DoType::ShiftLeft);
    }
}
