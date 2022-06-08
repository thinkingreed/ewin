use crate::{ewin_com::util::*, model::*};
use crossterm::cursor::*;
use ewin_cfg::{colors::*, log::*, model::default::*};

impl Editor {
    pub fn draw_scale(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_scale");
        Log::debug("self.row_posi", &self.row_posi);
        str_vec.push(Colors::get_scale_fg_bg());
        str_vec.push(format!("{}", MoveTo(0, (self.row_posi - 1) as u16)));
        if CfgEdit::get().general.editor.row_no.is_enable {
            if self.get_rnw() > 0 {
                str_vec.push(" ".repeat(self.get_rnw()));
            }
            #[allow(clippy::repeat_once)]
            str_vec.push(" ".repeat(Editor::RNW_MARGIN));
        }

        Log::debug("self.col_len", &self.col_len);
        Log::debug("self.offset_disp_x", &self.offset_disp_x);

        str_vec.push(Editor::get_scale_str(self.col_len, self.offset_disp_x));
        Log::debug(" Cfg::get().general.editor.scrollbar.vertical.width", &Cfg::get().general.editor.scrollbar.vertical.width);
        if self.scrl_v.is_show {
            str_vec.push(" ".repeat(Cfg::get().general.editor.scrollbar.vertical.width));
        }
       // str_vec.push(Colors::get_default_fg_bg())
    }

    pub fn get_scale_str(col_len: usize, offset_disp_x: usize) -> String {
        Log::debug_key("get_scale_str");

        let fixed_str = "----∣----∣";
        let is_unexpected_length = offset_disp_x.to_string().chars().count() >= 10;
        let div: usize = offset_disp_x / 10;
        let rest = if is_unexpected_length { 0 } else { offset_disp_x % 10 };
        let div_str = if is_unexpected_length { "".to_string() } else { div.to_string() };
        let div_str_len = if div == 0 || is_unexpected_length { 0 } else { div_str.chars().count() };

        Log::debug("div", &div);
        Log::debug("div_str_len", &div_str_len);
        Log::debug("rest", &rest);

        // First str
        let mut break_str = if div == 0 {
            fixed_str.chars().collect::<Vec<char>>()[rest..].iter().collect::<String>()
        } else {
            format!("{}{}", div_str, &fixed_str.chars().collect::<Vec<char>>()[div_str_len..].iter().collect::<String>()).chars().collect::<Vec<char>>()[rest..].iter().collect::<String>()
        };

        Log::debug("col_len", &col_len);
        Log::debug("break_str", &break_str);

        let diff = col_len - break_str.chars().count();
        let delim_10 = diff / 10;
        let delim_10_rest = diff % 10;
        let delim_10_rest_time = if delim_10_rest > 0 { 1 } else { 0 };

        let last_idx = div + delim_10 + delim_10_rest_time;
        for i in div + 1..=last_idx {
            let i_str = if is_unexpected_length { "".to_string() } else { i.to_string() };
            let rest_str = &fixed_str[i_str.chars().count()..];
            let mut join_str = format!("{}{}", &i_str, &rest_str);
            if i == last_idx {
                let r = col_len - break_str.chars().count();
                join_str = cut_str(&join_str, r, false, false);
            }
            break_str.push_str(&join_str);
        }
        Log::debug("break_str", &break_str);

        return break_str;
    }
}
