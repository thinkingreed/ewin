use crate::{model::*, window::*};
use crossterm::cursor::*;
use ewin_cfg::{colors::*, log::*};
use ewin_const::def::*;
use ewin_state::term::*;

impl Editor {
    pub fn draw_scale(&self, str_vec: &mut Vec<String>, win: &Window) {
        if State::get().curt_mut_state().editor.scale.is_enable {
            Log::debug_key("draw_scale");
            Log::debug("win.area_all_v.0", &win.area_v_all.0);
            str_vec.push(Colors::get_scale_fg_bg());
            str_vec.push(format!("{}", MoveTo(win.area_h_all.0 as u16, win.area_v_all.0 as u16)));

            if State::get().curt_state().editor.row_no.is_enable {
                let rnw = self.get_rnw();
                let rnw_and_margin = self.get_rnw_and_margin();

                str_vec.push(" ".repeat(if rnw > 0 { rnw_and_margin } else { Editor::RNW_MARGIN }));
            }

            Log::debug("win.width()", &win.width());
            let scale_str = Editor::get_scale_str(win.width(), win.offset.disp_x);

            if win.offset.disp_x <= win.cur.disp_x && win.cur.disp_x <= win.offset.disp_x + self.get_curt_col_len() {
                let tgt_1 = &scale_str[..scale_str.char_indices().nth(win.cur.disp_x - win.offset.disp_x).unwrap().0];
                Log::debug("tgt_1", &tgt_1);
                let tgt_2 = &scale_str[scale_str.char_indices().nth(win.cur.disp_x - win.offset.disp_x + 1).unwrap().0..];
                Log::debug("tgt_2", &tgt_2);
                str_vec.push(format!("{}{}", tgt_1, Colors::get_default_fg(),));
                str_vec.push("|".to_string());
                str_vec.push(format!("{}{}", Colors::get_scale_fg(), tgt_2));
            } else {
                str_vec.push(scale_str);
            }

            if win.scrl_v.is_show {
                str_vec.push(" ".repeat(win.scrl_v.bar_width));
            }

            // for split line width
            if self.win_mgr.split_line_v > 0 && win.h_idx == 0 {
                #[allow(clippy::repeat_once)]
                str_vec.push(" ".repeat(WINDOW_SPLIT_LINE_WIDTH));
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
        return break_str;
    }
}
