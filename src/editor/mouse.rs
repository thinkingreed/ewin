// 描画処理
use crate::model::Editor;
use crate::terminal::Log;

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
        Log::ep_s("★  mouse_left_press");

        self.sel.clear();
        self.sel.sy = y;
        self.sel.ey = y;
        self.sel.s_disp_x = x;
        self.sel.e_disp_x = x;
        if y as usize > self.buf.len() {
            return;
        }
    }

    pub fn mouse_hold(&mut self, x: usize, y: usize) {
        if y as usize > self.buf.len() {
            return;
        }
        //  let (_, width) = self.get_row_width(y, x);
        self.sel.e_disp_x = x;
        self.sel.ey = y;
    }

    pub fn mouse_release(&mut self, x: usize, y: usize) {
        self.sel.e_disp_x = x;
        self.sel.ey = y;
    }
}
