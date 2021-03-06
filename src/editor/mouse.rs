use crate::{log::*, model::*, util::*};

impl Editor {
    pub fn ctrl_mouse(&mut self, x: usize, y: usize, is_mouse_left_down: bool) {
        Log::ep_s("　　　　　　　  ctrl_mouse");
        if y >= self.disp_row_num || y >= self.buf.len_lines() {
            self.d_range.draw_type = DrawType::Not;
            return;
        }
        let mut x = x;
        if x <= self.rnw {
            x = self.rnw;
        }
        let (cur_x, width) = get_until_x(&self.buf.char_vec_line(y + self.offset_y), x + self.offset_x - self.rnw - 1);
        self.cur.y = y + self.offset_y;
        self.cur.x = cur_x + self.rnw;
        self.cur.disp_x = width + self.rnw + 1;

        self.set_mouse_sel(is_mouse_left_down);
        self.scroll_horizontal();
        if is_mouse_left_down {
            if self.sel_org.is_selected() {
                let sel_org = self.sel_org.get_range();
                self.d_range = DRange::new(sel_org.sy, sel_org.ey, DrawType::Target);
            }
        // Drag
        } else {
            if self.sel.is_selected() {
                let sy = self.sel.get_diff_y_mouse_drag(self.sel_org, self.cur.y);
                self.d_range = DRange::new(sy, sy + 1, DrawType::Target);
            }
        }

        Log::ep("self.history.mouse_click_vec", &self.history.mouse_click_vec);
    }

    pub fn set_mouse_sel(&mut self, is_mouse_left_down: bool) {
        if is_mouse_left_down {
            let click_count = self.history.check_multi_click(&self.evt);
            match click_count {
                1 => {
                    self.sel.clear();
                    self.sel.set_s(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x);
                    self.sel.set_e(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x);
                }
                2 => {
                    self.sel.ey = self.cur.y;
                    let row = &self.buf.char_vec_line(self.cur.y);
                    let (sx, ex) = get_delim_x(&row, self.cur.x - self.rnw);
                    self.sel.sx = sx;
                    self.sel.ex = ex;
                    let (_, s_disp_x) = get_row_width(&row[..sx], false);
                    let (_, e_disp_x) = get_row_width(&row[..ex], false);
                    self.sel.s_disp_x = s_disp_x + self.rnw + 1;
                    self.sel.e_disp_x = e_disp_x + self.rnw + 1;
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
            self.sel.set_e(self.cur.y, self.cur.x - self.rnw, self.cur.disp_x);
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
