use crate::{model::*, util::*};
use crossterm::event::{Event::*, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};

impl Editor {
    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  ctrl_mouse");
        if y >= self.disp_row_num || y >= self.buf.len_lines() {
            return;
        }
        Log::ep("yyy", y);

        let mut x = x;
        if x <= self.rnw {
            x = self.rnw;
        }
        let (cur_x, width) = get_until_x(&self.buf.char_vec_line(y + self.offset_y), x + self.offset_x - self.rnw - 1);
        self.cur.y = y + self.offset_y;
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        if is_mouse_left_down {
            self.sel.clear();
            self.sel.set_s(y + self.offset_y, self.cur.x - self.rnw, self.cur.disp_x);
        }

        self.sel.set_e(y + self.offset_y, self.cur.x - self.rnw, self.cur.disp_x);

        Log::ep("sel 111", self.sel);

        /*
        if !is_mouse_left_down {
            self.sel.check_overlap();
        }*/
        Log::ep("sel 222", self.sel);

        self.scroll_horizontal();

        Log::ep("self.cur.y", self.cur.y);

        
    }
}
