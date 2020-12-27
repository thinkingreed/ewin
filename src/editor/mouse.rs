use crate::{model::*, util::*};

impl Editor {
    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  ctrl_mouse");
        if y >= self.disp_row_num || y >= self.buf.len() {
            if is_mouse_left_down {
                self.sel.clear();
            }
            return;
        }
        self.cur.y = y;
        // row num location
        let mut x = x;
        if x <= self.rnw {
            x = self.rnw + 1;
        }
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);

        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        if is_mouse_left_down {
            self.sel.clear();
            self.sel.sy = y;
            self.sel.sx = self.cur.x - self.rnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.sel.ey = y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;

        self.scroll_horizontal();
    }
}
