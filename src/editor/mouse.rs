use crate::model::{Editor, Log};
use crate::util::*;

impl Editor {
    pub fn wheel_down(&mut self) {
        if self.cur.y == self.buf.len() - 1 {
            return;
        }
        self.cursor_down();
    }

    pub fn wheel_up(&mut self) {
        if self.cur.y == 0 {
            return;
        }
        self.cursor_up();
    }

    pub fn mouse_left_press(&mut self, x: usize, y: usize) {
        // x:1 index, y:0 index
        Log::ep_s("　　　　　　　  mouse_left_press");

        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }

        let (cur_x, width) = get_row_width(&self.buf[y], 0, x - self.rnw);
        if self.buf.len() - 1 >= y && width >= x - self.rnw {
            self.sel.clear();
            self.sel.sy = y;
            self.sel.ey = y;
            self.sel.s_disp_x = x;
            self.sel.e_disp_x = x;

            self.cur.y = y;
            self.cur.x = cur_x + self.rnw;
            self.cur.disp_x = width + self.rnw;
        }
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_hold");
        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }
        self.sel.e_disp_x = x;
        self.sel.ey = y;

        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
    }

    pub fn mouse_release(&mut self, x: usize, y: usize) {
        self.sel.e_disp_x = x;
        self.sel.ey = y;
    }
}
