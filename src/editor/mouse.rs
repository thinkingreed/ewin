use crate::{model::*, util::*};

impl Editor {
    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  ctrl_mouse");
        if y >= self.disp_row_num || y >= self.buf.len_lines() {
            return;
        }
        Log::ep("yyy", &y);

        let mut x = x;
        if x <= self.rnw {
            x = self.rnw;
        }
        let (cur_x, width) = get_until_x(&self.buf.char_vec_line(y + self.offset_y), x + self.offset_x - self.rnw - 1);
        self.cur.y = y + self.offset_y;
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        Log::ep("is_mouse_left_down", &is_mouse_left_down);

        self.set_mouse_sel(is_mouse_left_down);

        self.scroll_horizontal();

        Log::ep("self.sel", &self.sel);

        Log::ep("self.history.mouse_click_vec", &self.history.mouse_click_vec);
    }

    pub fn set_mouse_sel(&mut self, is_mouse_left_down: bool) {
        if is_mouse_left_down {
            let click_count = self.history.check_multi_click(&self.evt);
            match click_count {
                1 => {
                    self.sel.clear();
                    self.sel.set_s(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x + 1);
                    self.sel.set_e(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x + 1);
                }
                2 => {
                    self.sel.ey = self.cur.y;

                    //  let self.cur.x - self.rnw

                    self.sel.s_disp_x = 5;
                    self.sel.e_disp_x = 8;
                }
                // One line
                3 => {
                    self.sel.ey = self.cur.y;
                    self.sel.sx = self.rnw;
                    self.sel.s_disp_x = self.rnw + 1;
                    let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], true);
                    self.sel.ex = cur_x;
                    self.sel.e_disp_x = width + self.rnw + 1;
                }
                _ => {}
            }
        } else {
            self.sel.set_e(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x + 1);
        }
    }
}
