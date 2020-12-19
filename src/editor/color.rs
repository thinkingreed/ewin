use crate::{def::*, model::*, util::*};

impl Editor {
    /// 選択箇所のhighlight
    pub fn ctl_selcolor_eof(&mut self, str_vec: &mut Vec<String>, ranges: SelRange, y: usize, _x: usize) {
        if ranges.sy == 0 && ranges.s_disp_x == 0 {
            self.ctl_new_line_mark_color(str_vec, y, _x);
            return;
        }
        // Log::ep("ctl_select_color.ranges.sy", ranges.sy);
        // Log::ep("                 ranges.ey", ranges.ey);
        // Log::ep("                 ranges.s_disp_x", ranges.s_disp_x);
        // Log::ep("                 ranges.e_disp_x", ranges.e_disp_x);

        // １行または下に複数行選択
        if ranges.sy <= y && y <= ranges.ey {
            let (_, width) = get_row_width(&self.buf[y], 0, _x);
            // １行または下に複数行選択
            let x = width + self.rnw + 1;
            // 開始・終了が同じ行
            if ranges.sy == ranges.ey {
                if ranges.s_disp_x <= x && x < ranges.e_disp_x {
                    Colors::set_select_color(str_vec);
                } else {
                    // new line char color Control
                    self.ctl_new_line_mark_color(str_vec, y, _x);

                    // Colors::set_textarea_color(str_vec)
                }
            // 開始行
            } else if ranges.sy == y && ranges.s_disp_x <= x {
                Colors::set_select_color(str_vec);
            // 終了行
            } else if ranges.ey == y && x < ranges.e_disp_x {
                Colors::set_select_color(str_vec);
            // 中間行
            } else if ranges.sy < y && y < ranges.ey {
                Colors::set_select_color(str_vec);
            } else {
                // new line char color Control
                self.ctl_new_line_mark_color(str_vec, y, _x);

                //                Colors::set_textarea_color(str_vec)
            }
        } else {
            // new line char color Control
            self.ctl_new_line_mark_color(str_vec, y, _x);

            //            Colors::set_textarea_color(str_vec)
        }
    }

    /// 検索箇所のhighlight
    pub fn ctl_searchcolor_eof(&mut self, str_vec: &mut Vec<String>, ranges: &Vec<SearchRange>, y: usize, x: usize) {
        for range in ranges {
            if range.y != y {
                continue;
            } else {
                if range.sx <= x && x <= range.ex {
                    Colors::set_search_color(str_vec);
                    break;
                } else {
                    self.ctl_new_line_mark_color(str_vec, y, x);

                    // Colors::set_textarea_color(str_vec)
                }
            }
        }
    }

    /// 検索箇所のhighlight
    pub fn ctl_new_line_mark_color(&mut self, str_vec: &mut Vec<String>, y: usize, x: usize) {
        if self.buf[y].len() - 1 == x {
            Colors::set_new_line_color(str_vec);
        } else {
            Colors::set_textarea_color(str_vec);
        }
    }

    pub fn set_eof(&mut self, str_vec: &mut Vec<String>) {
        Colors::set_new_line_color(str_vec);
        str_vec.push(EOF_MARK.to_string());
        Colors::set_textarea_color(str_vec);
    }
}
