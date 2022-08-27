use crate::filebar::*;
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
};
use ewin_cfg::{colors::*, log::*};
use ewin_state::term::*;
use std::fmt::Write;

impl FileBar {
    pub fn draw(str_vec: &mut Vec<String>) {
        Log::info_key("FileBar.draw");

        if let Ok(fbar) = FileBar::get_result() {
            let menu_btn = format!(" {} ", "⠇");
            let left_arrow_btn = "< ".to_string();
            let right_arrow_btn = " >".to_string();

            let mut hber_str = format!("{}{}{}", MoveTo(0, fbar.row_posi as u16), Clear(ClearType::CurrentLine), Colors::get_default_fg_bg());
            if fbar.is_left_arrow_disp {
                let _ = write!(hber_str, "{}{}{}", &Colors::get_filebar_active_fg_bg(), left_arrow_btn, &Colors::get_default_fg_bg());
            }
            for (i, h_file) in fbar.file_vec.iter().enumerate() {
                if !h_file.is_disp {
                    continue;
                }
                Log::debug("State::get().tabs.idx", &State::get().tabs.idx);

                let state_color = if i == State::get().tabs.idx { Colors::get_filebar_active_fg_bg() } else { Colors::get_filebar_passive_fg_bg() };
                let _ = write!(hber_str, "{}{}{}", &state_color, &h_file.filenm_disp.clone(), &Colors::get_default_fg_bg());
            }

            let _ = write!(hber_str, "{}{}", &Colors::get_filebar_default_bg(), &" ".repeat(fbar.all_filenm_rest));

            if fbar.is_right_arrow_disp {
                hber_str.push_str(&right_arrow_btn);
            }
            hber_str = format!("{}{}{}", hber_str, menu_btn, Colors::get_default_bg(),);
            str_vec.push(hber_str);
        }
    }

    pub fn draw_only<T: std::io::Write>(out: &mut T) {
        Log::debug_key("FileBar::draw_only");
        let mut v: Vec<String> = vec![];
        FileBar::draw(&mut v);
        let _ = out.write(v.concat().as_bytes());
        out.flush().unwrap();
    }
}
