use crate::window::window::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::{def::*, models::view::*};
use ewin_state::term::*;

impl Window {
    pub fn draw_scale(&self, str_vec: &mut Vec<String>, split_line_v: usize) {
        if State::get().curt_mut_state().editor.scale.is_enable {
            Log::debug_key("draw_scale");
            Log::debug("win.area_all_v.0", &self.view_all.y);
            str_vec.push(Colors::get_scale_fg_bg());
            str_vec.push(MoveTo(self.view_all.x as u16, self.view_all.y as u16).to_string());

            if State::get().curt_ref_state().editor.row_no.is_enable {
                //       str_vec.push(get_space(if rnw > 0 { rnw_and_margin } else { Editor::RNW_MARGIN }));
                str_vec.push(get_space(self.view.x - self.view_all.x));
            }

            Log::debug("win.width()", &self.width());
            let scale_str = Window::get_scale_str(self.width(), self.offset.disp_x);

            if self.offset.disp_x <= self.cur.disp_x && self.cur.disp_x <= self.offset.disp_x + self.view.width {
                let tgt_1 = &scale_str[..scale_str.char_indices().nth(self.cur.disp_x - self.offset.disp_x).unwrap().0];
                Log::debug("tgt_1", &tgt_1);
                let tgt_2 = &scale_str[scale_str.char_indices().nth(self.cur.disp_x - self.offset.disp_x + 1).unwrap().0..];
                Log::debug("tgt_2", &tgt_2);
                str_vec.push(format!("{}{}", tgt_1, Colors::get_default_fg(),));
                str_vec.push("|".to_string());
                str_vec.push(format!("{}{}", Colors::get_scale_fg(), tgt_2));
            } else {
                str_vec.push(scale_str);
            }

            if self.scrl_v.is_show {
                str_vec.push(get_space(self.scrl_v.view.width));
            }

            // for split line width
            if split_line_v > 0 && self.h_idx == 0 {
                #[allow(clippy::repeat_once)]
                str_vec.push(get_space(WINDOW_SPLIT_LINE_WIDTH));
            }
        }

        str_vec.push(Colors::get_default_fg_bg());
    }

    pub fn get_scale_str(range_len: usize, offset_disp_x: usize) -> String {
        Log::debug_key("get_scale_str");

        let fixed_str = "----∣----∣";
        let is_unexpected_length = offset_disp_x.to_string().chars().count() >= 10;
        let div: usize = offset_disp_x / 10;
        let rest = if is_unexpected_length { 0 } else { offset_disp_x % 10 };
        let div_str = if is_unexpected_length { "".to_string() } else { div.to_string() };
        let div_str_len = if div == 0 || is_unexpected_length { 0 } else { div_str.chars().count() };

        // First str
        let mut break_str = if div == 0 {
            fixed_str.chars().collect::<Vec<char>>()[rest..].iter().collect::<String>()
        } else {
            format!("{}{}", div_str, &fixed_str.chars().collect::<Vec<char>>()[div_str_len..].iter().collect::<String>()).chars().collect::<Vec<char>>()[rest..].iter().collect::<String>()
        };

        if range_len > break_str.chars().count() {
            let diff = range_len - break_str.chars().count();
            let delim_10 = diff / 10;
            let delim_10_rest = diff % 10;
            let delim_10_rest_time = if delim_10_rest > 0 { 1 } else { 0 };

            let last_idx = div + delim_10 + delim_10_rest_time;

            for i in div + 1..=last_idx {
                let i_str = if is_unexpected_length { "".to_string() } else { i.to_string() };
                let rest_str = &fixed_str.chars().collect::<Vec<char>>()[i_str.chars().count()..].iter().collect::<String>();
                let mut join_str = format!("{}{}", &i_str, &rest_str);
                if i == last_idx {
                    let r = range_len - break_str.chars().count();
                    join_str = join_str.chars().collect::<Vec<char>>()[..r].iter().collect::<String>();
                }
                break_str.push_str(&join_str);
            }
        } else {
            break_str = break_str.chars().collect::<Vec<char>>()[..range_len].iter().collect::<String>();
        }
        return break_str;
    }
}
