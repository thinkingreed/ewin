use crate::{model::*, util::*};
use std::io::Write;

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

    pub fn mouse_left_press<T: Write>(&mut self, out: &mut T, sbar: &mut StatusBar, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_left_press");
        Log::ep("xxx", x);

        if x <= self.rnw || y >= self.disp_row_num || y >= self.buf.len() {
            return;
        }
        self.cur.y = y;
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        Log::ep("self.cur.x", self.cur.x);
        Log::ep(" self.cur.disp_x", self.cur.disp_x);

        self.sel.clear();

        self.sel.sy = y;
        self.sel.ey = y;
        self.sel.sx = self.cur.x - self.rnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.e_disp_x = self.cur.disp_x;
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_hold");
        if x <= self.rnw || y >= self.disp_row_num || y >= self.buf.len() {
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
        if x <= self.rnw || y >= self.disp_row_num || y >= self.buf.len() {
            return;
        }

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
