use crate::model::{Editor, Log};
use crate::util::*;

impl Editor {
    pub fn shift_right(&mut self) {
        Log::ep_s("★  shift_right");

        let is_unselected_org = self.sel.is_unselected();
        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_right();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;
        // shift_leftからのshift_right
        if e_disp_x_org == disp_x_org {
            self.sel.ey = self.cur.y;
            self.sel.ex = self.cur.x - self.lnw;
            self.sel.e_disp_x = self.cur.disp_x;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org
        // 行の終端から次行に移る場合の不具合対応でcur.yと比較
            && self.sel.sy == self.cur.y
            // sel.s_disp_x == sel.e_disp_x + 1文字でclear
            && self.sel.s_disp_x + self.get_char_width(self.cur.y, self.cur.x - 1) == self.cur.disp_x
        {
            self.sel.clear();
        }
    }
    pub fn shift_left(&mut self) {
        Log::ep_s("★  shift_left");

        let is_unselected_org = self.sel.is_unselected();
        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_left();

        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;

        // shift_rightからのshift_left
        if e_disp_x_org != 0 && e_disp_x_org < disp_x_org {
            self.sel.e_disp_x -= 1;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }

    pub fn shift_down(&mut self) {
        Log::ep_s("★　shift_down");
        let is_unselected_org = self.sel.is_unselected();

        if self.sel.is_unselected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_down();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;

        Log::ep("sel.ex", self.sel.ex);
        Log::ep("sel.e_disp_x ", self.sel.e_disp_x);

        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.s_disp_x == self.sel.e_disp_x && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }
    pub fn shift_up(&mut self) {
        Log::ep_s("★　shift_up");

        Log::ep("cur.x", self.cur.x);

        let is_unselected_org = self.sel.is_unselected();

        if self.cur.y == 0 {
            return;
        } else {
            if self.sel.is_unselected() {
                self.sel.sy = self.cur.y;
                self.sel.sx = self.cur.x - 1;
                self.sel.s_disp_x = self.cur.disp_x;
                // 行頭の場合に先頭文字を含めない
                if self.cur.x == self.lnw {
                    self.sel.s_disp_x = self.cur.disp_x - 1;
                }
            }
            self.cursor_up();
            self.sel.ey = self.cur.y;
            self.sel.ex = self.cur.x - self.lnw;
            self.sel.e_disp_x = self.cur.disp_x;
        }
        // 選択開始位置とカーソルが重なった場合
        if !is_unselected_org && self.sel.s_disp_x == self.sel.e_disp_x && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
    }
    pub fn shift_home(&mut self) {
        Log::ep_s("★　shift_home");
        self.sel.sy = self.cur.y;
        self.sel.sx = self.cur.x - self.lnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.ey = self.cur.y;
        self.sel.ex = self.lnw;
        self.sel.e_disp_x = self.lnw;
        self.cur.x = self.lnw;
        self.cur.disp_x = self.lnw + 1;
    }
    pub fn shift_end(&mut self) {
        Log::ep_s("★  shift_end");

        self.sel.sy = self.cur.y;
        self.sel.sx = self.cur.x - self.lnw;
        self.sel.s_disp_x = self.cur.disp_x;
        self.sel.ey = self.cur.y;
        self.sel.ex = self.buf[self.cur.y].len();
        let (_, width) = get_row_width(&self.buf[self.cur.y], self.cur.x - self.lnw, self.buf[self.cur.y].len());
        self.sel.e_disp_x = self.cur.disp_x + width;

        self.cur.disp_x = self.sel.e_disp_x;
        self.cur.x = self.buf[self.cur.y].len() + self.lnw;
    }
}
