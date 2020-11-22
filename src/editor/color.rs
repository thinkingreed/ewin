use crate::model::*;
use crate::util::*;

impl Editor {
    /// 選択箇所のhighlight
    pub fn ctl_select_color(&mut self, str_vec: &mut Vec<String>, ranges: SelRange, y: usize, _x: usize) {
        if ranges.sy == 0 && ranges.s_disp_x == 0 {
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
                    Colors::set_textarea_color(str_vec)
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
                Colors::set_textarea_color(str_vec)
            }
        } else {
            Colors::set_textarea_color(str_vec)
        }
    }

    /// 検索箇所のhighlight
    pub fn ctl_search_color(&mut self, str_vec: &mut Vec<String>, ranges: &Vec<SearchRange>, y: usize, x: usize) {
        for range in ranges {
            if range.y != y {
                continue;
            } else {
                if range.sx <= x && x <= range.ex {
                    Colors::set_search_color(str_vec);
                    break;
                } else {
                    Colors::set_textarea_color(str_vec)
                }
            }
        }
    }
}
