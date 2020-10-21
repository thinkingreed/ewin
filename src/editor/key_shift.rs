use crate::model::*;
use crate::util::*;
use std::cmp::{max, min};

impl Editor {
    pub fn shift_right(&mut self) {
        Log::ep_s("★  shift_right");

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
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
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();

        self.d_range = DRnage {
            sy: self.cur.y,
            ey: self.cur.y,
            e_type: EType::Mod,
        };
    }
    pub fn shift_left(&mut self) {
        Log::ep_s("★  shift_left");

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
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
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }
        self.scroll();
        self.scroll_horizontal();

        self.d_range = DRnage {
            sy: self.cur.y,
            ey: self.cur.y,
            e_type: EType::Mod,
        };
    }

    pub fn shift_down(&mut self) {
        Log::ep_s("★　shift_down");

        if self.cur.y == self.buf.len() - 1 {
            return;
        }
        let y_offset_org: usize = self.y_offset;

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_down();
        self.sel.ey = self.cur.y;
        self.sel.ex = self.cur.x - self.lnw;
        self.sel.e_disp_x = self.cur.disp_x;

        // ShiftUp,Down繰り返す場合の対応
        let mut sy = min(self.sel.sy, self.sel.ey);
        if sy > 0 {
            sy -= 1;
        }
        self.d_range = DRnage {
            sy: sy,
            ey: max(self.sel.sy, self.sel.ey),
            e_type: EType::Mod,
        };
        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();

        if y_offset_org != self.y_offset {
            self.d_range = DRnage { e_type: EType::All, ..DRnage::default() };
        }
    }

    pub fn shift_up(&mut self) {
        Log::ep_s("★　shift_up");

        if self.cur.y == 0 {
            return;
        }

        if !self.sel.is_selected() {
            self.sel.sy = self.cur.y;
            self.sel.sx = self.cur.x - self.lnw;
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

        // ShiftUp,Down繰り返す場合の対応
        let mut sy = max(self.sel.sy, self.sel.ey);
        if self.buf.len() > sy + 1 {
            sy += 1;
        }
        self.d_range = DRnage { sy: sy, ey: self.sel.ey, e_type: EType::Mod };

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex && self.sel.sy == self.sel.ey {
            self.sel.clear();
        }

        self.scroll();
        self.scroll_horizontal();
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

        self.d_range = DRnage {
            sy: self.cur.y,
            ey: self.cur.y,
            e_type: EType::Mod,
        };
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

        self.d_range = DRnage {
            sy: self.cur.y,
            ey: self.cur.y,
            e_type: EType::Mod,
        };
    }
}
