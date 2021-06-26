use crate::{log::*, model::*, util::*};
use std::cmp::min;

impl Editor {
    pub fn ctrl_mouse(&mut self, y: usize, x: usize, is_left_down: bool) {
        Log::debug_key("ctrl_mouse");
        if y < self.disp_row_posi || y > self.disp_row_num || y > self.buf.len_lines() {
            if self.sel.is_selected() {
                self.sel.clear();
                self.d_range.draw_type = DrawType::All;
            } else {
                self.d_range.draw_type = DrawType::Not;
            }
            return;
        }
        Log::debug("y", &y);
        Log::debug("x", &x);

        let y = y - self.disp_row_posi;
        if x < self.get_rnw() + Editor::RNW_MARGIN {
            self.sel.set_s(y, 0, 0);
            let (cur_x, width) = get_row_width(&self.buf.char_vec_line(y)[..], 0, true);
            self.sel.set_e(y, cur_x, width);
            self.set_cur_target(y + self.offset_y, 0, false);
            self.d_range.draw_type = DrawType::All;
        } else {
            let x = if self.mode == EditerMode::Normal { x - self.get_rnw() - Editor::RNW_MARGIN } else { x };
            self.cur.y = y + self.offset_y;
            let (cur_x, width) = get_until_x(&self.buf.char_vec_line(y + self.offset_y), x + self.offset_x);
            self.cur.x = cur_x;
            self.cur.disp_x = width;

            self.set_mouse_sel(is_left_down);
            self.scroll_horizontal();
            if is_left_down {
                if self.sel_org.is_selected() {
                    self.d_range.draw_type = DrawType::All;
                }
            // Drag
            } else {
                if self.sel.is_selected() {
                    let sy = self.sel.get_diff_y_mouse_drag(self.sel_org, self.cur.y);
                    self.d_range = DRange::new(sy, min(sy + 1, self.buf.len_lines() - 1), DrawType::Target);
                }
            }
        }
    }

    pub fn set_mouse_sel(&mut self, is_mouse_left_down: bool) {
        if is_mouse_left_down {
            let click_count = self.history.count_multi_click(&self.keycmd);
            match click_count {
                1 => {
                    self.sel.clear();
                    self.sel.set_s(self.cur.y, self.cur.x, self.cur.disp_x);
                    self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
                }
                2 => {
                    self.sel.ey = self.cur.y;
                    let row = &self.buf.char_vec_line(self.cur.y);
                    let (sx, ex) = get_delim_x(&row, self.cur.x);
                    self.sel.sx = sx;
                    self.sel.ex = ex;
                    let (_, s_disp_x) = get_row_width(&row[..sx], self.offset_disp_x, false);
                    let (_, e_disp_x) = get_row_width(&row[..ex], self.offset_disp_x, false);
                    self.sel.disp_x_s = s_disp_x;
                    self.sel.disp_x_e = e_disp_x;
                }
                // One line
                3 => {
                    self.sel.ey = self.cur.y;
                    self.sel.sx = 0;
                    self.sel.disp_x_s = 0;
                    let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], self.offset_disp_x, true);
                    self.sel.ex = cur_x;
                    self.sel.disp_x_e = width;
                }
                _ => {}
            }
        } else {
            self.sel.set_e(self.cur.y, self.cur.x, self.cur.disp_x);
        }
    }
}

fn get_delim(target: &Vec<char>, x: usize, is_forward: bool) -> usize {
    let mut rtn_x = 0;

    let mut char_type_org = CharType::Nomal;
    for (i, c) in (0_usize..).zip(target) {
        let char_type = get_char_type(*c);
        if i == 0 {
            char_type_org = char_type;
        }
        if char_type != char_type_org {
            rtn_x = if is_forward { x - i + 1 } else { x + i };
            break;
        } else {
            if i == target.len() - 1 {
                rtn_x = if is_forward { x - i } else { x + i + 1 };
            }
        }
        char_type_org = char_type;
    }
    return rtn_x;
}

pub fn get_delim_x(row: &Vec<char>, x: usize) -> (usize, usize) {
    let mut forward = row[..x + 1].to_vec();
    forward.reverse();
    let sx = get_delim(&forward, x, true);
    let backward = row[x..].to_vec();
    let ex = get_delim(&backward, x, false);
    return (sx, ex);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_delim_1() {
        let vec: Vec<char> = "<12345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 0), (0, 1));
    }
    #[test]
    fn test_get_delim_2() {
        let vec: Vec<char> = "<12345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 1), (1, 6));
    }
    #[test]
    fn test_get_delim_3() {
        let vec: Vec<char> = "<12345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 6), (6, 7));
    }
    #[test]
    fn test_get_delim_4() {
        let vec: Vec<char> = "  12345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 0), (0, 2));
    }
    #[test]
    fn test_get_delim_5() {
        let vec: Vec<char> = "  　　12345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 1), (0, 2));
    }
    #[test]
    fn test_get_delim_6() {
        let vec: Vec<char> = "<12345>>".chars().collect();
        assert_eq!(get_delim_x(&vec, 6), (6, 8));
    }
    #[test]
    fn test_get_delim_7() {
        let vec: Vec<char> = "<12345  ".chars().collect();
        assert_eq!(get_delim_x(&vec, 6), (6, 8));
    }
    #[test]
    fn test_get_delim_8() {
        let vec: Vec<char> = "<12345　　".chars().collect();
        assert_eq!(get_delim_x(&vec, 6), (6, 8));
    }
    #[test]
    fn test_get_delim_9() {
        let vec: Vec<char> = "<12345".chars().collect();
        assert_eq!(get_delim_x(&vec, 1), (1, 6));
    }
    #[test]
    fn test_get_delim_10() {
        let vec: Vec<char> = "<12<<345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 4), (3, 5));
    }
    #[test]
    fn test_get_delim_11() {
        let vec: Vec<char> = "<12  345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 4), (3, 5));
    }
    #[test]
    fn test_get_delim_12() {
        let vec: Vec<char> = "<12　　345>".chars().collect();
        assert_eq!(get_delim_x(&vec, 4), (3, 5));
    }
}
