use crate::{def::*, model::*, util::*};

impl Editor {
    /// 選択箇所のhighlight
    pub fn ctl_color(&mut self, str_vec: &mut Vec<String>, row_vec: &Vec<char>, sel_ranges: SelRange, search_ranges: &Vec<SearchRange>, y: usize, x: usize) {
        /*
                Log::ep("ctl_select_color.ranges.sy", sel_ranges.sy);
                Log::ep("                 ranges.ey", sel_ranges.ey);
                Log::ep("                 ranges.s_disp_x", sel_ranges.s_disp_x);
                Log::ep("                 ranges.e_disp_x", sel_ranges.e_disp_x);
                Log::ep("                 xxxxxxxxxxxxxxx", x);
                Log::ep("                 yyyyyyyyyyyyyyy", y);
        */
        if sel_ranges.sy <= y && y <= sel_ranges.ey {
            let (_, width) = get_row_width(&row_vec[..x], true);

            let disp_x = width + self.rnw + 1;

            // 開始・終了が同じ行
            if sel_ranges.sy == sel_ranges.ey {
                if sel_ranges.s_disp_x <= disp_x && disp_x < sel_ranges.e_disp_x {
                    Colors::set_select_color(str_vec);
                    self.is_default_color = false;
                } else {
                    // new line char color Control
                    self.ctl_new_line_mark_color(str_vec, row_vec[x]);
                }
            // 開始行
            } else if sel_ranges.sy == y && sel_ranges.s_disp_x <= disp_x {
                Colors::set_select_color(str_vec);
                self.is_default_color = false;
            // 終了行
            } else if sel_ranges.ey == y && disp_x < sel_ranges.e_disp_x {
                Colors::set_select_color(str_vec);
                self.is_default_color = false;
            // 中間行
            } else if sel_ranges.sy < y && y < sel_ranges.ey {
                Colors::set_select_color(str_vec);
                self.is_default_color = false;
            } else {
                // new line char color Control
                self.ctl_new_line_mark_color(str_vec, row_vec[x]);
            }
        } else {
            // new line char color Control
            self.ctl_new_line_mark_color(str_vec, row_vec[x]);
        }

        for range in search_ranges {
            if range.y != y {
                continue;
            } else {
                if range.sx <= x && x < range.ex {
                    Colors::set_search_color(str_vec);
                    self.is_default_color = false;
                    break;
                }
            }
        }
    }

    pub fn ctl_new_line_mark_color(&mut self, str_vec: &mut Vec<String>, c: char) {
        if c == NEW_LINE {
            // Log::ep_s("NEW_LINE_MARK NEW_LINE_MARK NEW_LINE_MARK");
            Colors::set_new_line_color(str_vec);
            self.is_default_color = false;
        } else {
            if !self.is_default_color {
                // Log::ep_s("textarea_color textarea_color textarea_color");
                Colors::set_textarea_color(str_vec);
                self.is_default_color = true;
            }
        }
    }

    pub fn set_eof(&mut self, str_vec: &mut Vec<String>) {
        Colors::set_new_line_color(str_vec);
        str_vec.push(EOF_STR.to_string());
        Colors::set_textarea_color(str_vec);
    }
}
