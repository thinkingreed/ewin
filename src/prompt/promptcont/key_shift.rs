use crate::{log::*, model::*, prompt::promptcont::promptcont::*, util::*};

impl PromptCont {
    fn shift_move_com(&mut self, do_type: EvtType) {
        self.sel.set_sel_posi(true, self.cur.y, self.cur.x, self.cur.disp_x);

        match do_type {
            EvtType::ShiftRight => self.cur_right(),
            EvtType::ShiftLeft => self.cur_left(),
            EvtType::ShiftHome => {
                self.cur.x = 0;
                self.cur.disp_x = 0;
            }
            EvtType::ShiftEnd => {
                let (cur_x, width) = get_row_width(&self.buf[..], 0, false);
                self.cur.x = cur_x;
                self.cur.disp_x = width;
            }
            _ => {}
        }
        self.sel.set_sel_posi(false, self.cur.y, self.cur.x, self.cur.disp_x);
        self.sel.check_overlap();
    }

    pub fn shift_home(&mut self) {
        Log::debug_s("              　shift_home");
        self.shift_move_com(EvtType::ShiftHome);
    }
    pub fn shift_end(&mut self) {
        Log::debug_s("                shift_end");
        self.shift_move_com(EvtType::ShiftEnd);
    }

    pub fn shift_right(&mut self) {
        Log::debug_s("                shift_right");
        self.shift_move_com(EvtType::ShiftRight);
    }

    pub fn shift_left(&mut self) {
        Log::debug_s("                shift_left");
        self.shift_move_com(EvtType::ShiftLeft);
    }
}
