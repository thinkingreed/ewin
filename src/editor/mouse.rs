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

    /*
    pub fn mouse_left_press(&mut self, x: usize, y: usize) {
        // x:1 index, y:0 index
        Log::ep_s("　　　　　　　  mouse_left_press");
        Log::ep("xxxxx", x);

        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }

        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        Log::ep("cur_x", cur_x);
        Log::ep("width", width);

        if self.buf.len() - 1 >= y && width >= x - self.rnw {
            self.cur.y = y;
            self.cur.x = cur_x + self.rnw - 1;
            self.cur.disp_x = width + self.rnw + 1;

            self.sel.clear();
            self.sel.sy = y;
            //     self.sel.sx = self.cur.x - self.rnw;
            //     self.sel.s_disp_x = self.cur.disp_x;
            self.sel.s_disp_x = x;

            self.sel.ey = y;
            //    self.sel.ex = self.cur.x - self.rnw;
            // self.sel.e_disp_x = self.cur.disp_x;
            self.sel.e_disp_x = x;
        }
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_hold");
        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }
        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw - 1;
        self.cur.disp_x = width + self.rnw + 1;

        self.sel.ey = y;
        //    self.sel.ex = self.cur.x - self.rnw;
        // self.sel.e_disp_x = self.cur.disp_x;
        self.sel.e_disp_x = x;
    }

    pub fn mouse_release(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_release");
        Log::ep("xxxxx", x);

        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw);
        Log::ep("cur_x", cur_x);
        Log::ep("width", width);
        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw - 1;
        self.cur.disp_x = width + self.rnw + 1;

        self.sel.ey = y;
        //    self.sel.ex = self.cur.x - self.rnw;
        // self.sel.e_disp_x = self.cur.disp_x;
        self.sel.e_disp_x = x;
    }
    */
    pub fn mouse_left_press(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_left_press");
        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }
        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;
        self.sel.clear();

        self.sel.sy = y;
        self.sel.ey = y;
        self.sel.s_disp_x = x;
        self.sel.e_disp_x = x;
        self.sel.sx = self.cur.x - self.rnw;
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_hold");
        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }
        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        self.sel.ey = y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;
    }

    pub fn mouse_release(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_release");
        if x <= self.rnw || y >= self.disp_row_num {
            return;
        }
        Log::ep("xxxxx", x);

        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        Log::ep("self.cur.x ", self.cur.x);
        Log::ep(" self.cur.disp_x ", self.cur.disp_x);

        self.sel.ey = y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;
    }
}
