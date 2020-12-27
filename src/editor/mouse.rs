use crate::{model::*, util::*};

impl Editor {
    pub fn mouse_left_press(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_left_press");

        if y >= self.disp_row_num || y >= self.buf.len() {
            self.sel.clear();
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

        // Log::ep("self.cur.x", self.cur.x);
        // Log::ep(" self.cur.disp_x", self.cur.disp_x);

        self.sel.clear();
        self.sel.sy = y;
        self.sel.ey = y;
        self.sel.sx = self.cur.x - self.rnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.e_disp_x = self.cur.disp_x;
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_hold");
        if !self.sel.is_selected() || y >= self.disp_row_num || y >= self.buf.len() {
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

        self.sel.ey = y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;
    }

    pub fn mouse_release(&mut self, x: usize, y: usize) {
        Log::ep_s("　　　　　　　  mouse_release");
        if !self.sel.is_selected() || y >= self.disp_row_num || y >= self.buf.len() {
            return;
        }

        self.cur.y = y;
        let mut x = x;
        if x <= self.rnw {
            x = self.rnw + 1;
        }
        let (cur_x, width) = get_until_x(&self.buf[y], x - self.rnw - 1);
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        // Log::ep("self.cur.x ", self.cur.x);
        // Log::ep(" self.cur.disp_x ", self.cur.disp_x);
        self.sel.ey = y;
        self.sel.ex = self.cur.x - self.rnw;
        self.sel.e_disp_x = self.cur.disp_x;
    }
}
