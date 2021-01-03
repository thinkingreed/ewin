use crate::{model::*, util::*};

impl Editor {
    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  ctrl_mouse");
        Log::ep("yyy", y);
        Log::ep("xxx", x);
        Log::ep("self.sel", self.sel);
        if y >= self.disp_row_num || y >= self.buf.len_lines() {
            return;
        }
        // row num location
        let mut x = x;
        if x <= self.rnw {
            x = self.rnw + 1;
        }

        Log::ep("y + self.offset_y", y + self.offset_y);

        let (cur_x, width) = get_until_x(&self.buf.char_vec(y + self.offset_y), x + self.offset_x - self.rnw - 1);

        self.cur.y = y + self.offset_y;
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        if is_mouse_left_down {
            self.sel.clear();
            self.sel.sy = y + self.offset_y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.sel.ey = y + self.offset_y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;

        self.scroll_horizontal();
    }
}
