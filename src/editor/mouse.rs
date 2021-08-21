use crate::{_cfg::keys::KeyCmd, def::USIZE_UNDEFINED, log::*, model::*, sel_range::*, util::*};
use std::cmp::{max, min};

impl Editor {
    pub fn ctrl_mouse(&mut self) {
        Log::debug_key("ctrl_mouse");
        let (y, mut x, mouse_proc) = match self.keycmd {
            KeyCmd::MouseDownLeft(y, x) => (y, x, MouseProc::DownLeft),
            KeyCmd::MouseDragLeft(y, x) => (y, x, MouseProc::DragLeft),
            KeyCmd::MouseDownBoxLeft(y, x) => (y, x, MouseProc::DownLeftBox),
            KeyCmd::MouseDragBoxLeft(y, x) => (y, x, MouseProc::DragLeftBox),
            _ => return,
        };
        if mouse_proc == MouseProc::DownLeftBox || mouse_proc == MouseProc::DragLeftBox {
            self.sel.mode = SelMode::BoxSelect;
        } else if mouse_proc == MouseProc::DownLeft {
            self.sel.mode = SelMode::Normal;
        }
        // y, x range check
        if y < self.disp_row_posi || self.disp_row_num < y || self.buf.len_lines() < y {
            if self.buf.len_lines() < y {
                // In case of MouseMode::Mouse, this function is not executed, so ignore it.
                self.set_cur_target(self.buf.len_lines() - 1, get_until_x(&self.buf.char_vec_line(self.buf.len_lines() - 1), x - self.get_rnw_and_margin()).0, false)
            }
            self.draw_type = DrawType::All;
            if mouse_proc != MouseProc::DragLeft {
                self.sel.clear();
            }
            return;
        }

        let y = y - self.disp_row_posi;
        if mouse_proc == MouseProc::DownLeft && x < self.get_rnw_and_margin() {
            self.sel.set_s(y, 0, 0);
            let (cur_x, width) = get_row_width(&self.buf.char_vec_line(y)[..], 0, true);
            self.sel.set_e(y, cur_x, width);
            self.set_cur_target(y + self.offset_y, 0, false);
            self.draw_type = DrawType::All;
        } else {
            if x < self.get_rnw_and_margin() {
                x = self.get_rnw_and_margin();
            }
            let x = x - self.get_rnw_and_margin();
            self.cur.y = y + self.offset_y;

            let vec = self.buf.char_vec_line(self.cur.y);

            if self.sel.mode == SelMode::BoxSelect && self.offset_x + x > vec.len() - 1 {
                self.cur.x = x;
                self.cur.disp_x = x;
            } else {
                let (cur_x, width) = get_until_x(&vec, x + self.offset_x);
                self.cur.x = cur_x;
                self.cur.disp_x = width;
                self.scroll_horizontal();
            }

            self.set_mouse_sel(mouse_proc);

            if self.sel.is_selected() {
                match mouse_proc {
                    MouseProc::DownLeft | MouseProc::DownLeftBox | MouseProc::DragLeftBox => self.draw_type = DrawType::All,
                    MouseProc::DragLeft => {
                        if self.sel.mode == SelMode::Normal {
                            let sel = self.sel.get_range();
                            let sel_org = self.sel_org.get_range();
                            self.draw_type = DrawType::Target(min(sel.sy, sel_org.sy), max(sel.ey, if sel_org.ey == USIZE_UNDEFINED { sel.ey } else { sel_org.ey }));
                        } else {
                            self.draw_type = DrawType::All;
                        }
                    }
                }
            }
        }
    }

    pub fn set_mouse_sel(&mut self, mouse_proc: MouseProc) {
        if mouse_proc == MouseProc::DownLeft || mouse_proc == MouseProc::DownLeftBox {
            let click_count = self.history.count_multi_click(&self.keycmd);
            Log::debug("click_count", &click_count);
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
                    self.sel.s_disp_x = s_disp_x;
                    self.sel.e_disp_x = e_disp_x;
                }
                // One line
                3 => {
                    self.sel.ey = self.cur.y;
                    self.sel.sx = 0;
                    self.sel.s_disp_x = 0;
                    let (cur_x, width) = get_row_width(&self.buf.char_vec_line(self.cur.y)[..], self.offset_disp_x, true);
                    self.sel.ex = cur_x;
                    self.sel.e_disp_x = width;
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
        } else if i == target.len() - 1 {
            rtn_x = if is_forward { x - i } else { x + i + 1 };
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
