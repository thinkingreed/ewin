use crate::model::*;
use crate::util::*;

impl PromptCont {
    pub fn shift_home(&mut self) {
        Log::ep_s("　　　　　　　　shift_home");
        if !self.sel.is_selected() {
            self.sel.sx = self.cur.x;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cur.x = 0;
        self.cur.disp_x = 1;

        self.sel.ex = 0;
        self.sel.e_disp_x = 1;

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex {
            self.sel.clear();
        }
    }
    pub fn shift_end(&mut self) {
        Log::ep_s("　　　　　　　  shift_end");

        if !self.sel.is_selected() {
            self.sel.sx = self.cur.x;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        let (_, width) = get_row_width(&self.buf[self.cur.x..], false);
        self.cur.disp_x = self.cur.disp_x + width;
        self.cur.x = self.buf.len();

        self.sel.ex = self.buf.len();
        self.sel.e_disp_x = self.cur.disp_x;

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex {
            self.sel.clear();
        }
    }

    pub fn shift_right(&mut self) {
        Log::ep_s("　　　　　　　  shift_right");

        Log::ep("sel.s_disp_x", self.sel.s_disp_x);
        Log::ep("sel.e_disp_x", self.sel.e_disp_x);

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
            self.sel.sx = self.cur.x;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_right();
        self.sel.ex = self.cur.x;
        self.sel.e_disp_x = self.cur.disp_x;
        // shift_leftからのshift_right
        if e_disp_x_org == disp_x_org {
            self.sel.ex = self.cur.x;
            self.sel.e_disp_x = self.cur.disp_x;
        }

        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex {
            self.sel.clear();
        }
        Log::ep("sel.s_disp_x", self.sel.s_disp_x);
        Log::ep("sel.e_disp_x", self.sel.e_disp_x);
    }

    pub fn shift_left(&mut self) {
        Log::ep_s("　　　　　　　  shift_left");

        let e_disp_x_org = self.sel.e_disp_x;
        let disp_x_org = self.cur.disp_x;

        if !self.sel.is_selected() {
            self.sel.sx = self.cur.x;
            self.sel.s_disp_x = self.cur.disp_x;
        }
        self.cursor_left();

        self.sel.ex = self.cur.x;
        self.sel.e_disp_x = self.cur.disp_x;

        // shift_rightからのshift_left
        if e_disp_x_org != 0 && e_disp_x_org < disp_x_org {
            self.sel.e_disp_x -= 1;
        }
        // 選択開始位置とカーソルが重なった場合
        if self.sel.sx == self.sel.ex {
            self.sel.clear();
        }
    }
}
