use crate::{def::*, model::*};
use crossterm::style::{Color, SetBackgroundColor, SetForegroundColor};

impl Colors {
    pub fn set_rownum_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg((110, 110, 110)));
        str_vec.push(Colors::bg((0, 0, 0)));
    }

    pub fn fg(rgb: (u8, u8, u8)) -> String {
        SetForegroundColor(Color::Rgb { r: rgb.0, g: rgb.1, b: rgb.2 }).to_string()
    }

    pub fn bg(rgb: (u8, u8, u8)) -> String {
        SetBackgroundColor(Color::Rgb { r: rgb.0, g: rgb.1, b: rgb.2 }).to_string()
    }

    pub fn set_textarea_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg((255, 255, 255)));
        str_vec.push(Colors::bg((0, 0, 0)));
    }

    pub fn set_select_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg((0, 0, 0)));
        str_vec.push(Colors::bg((221, 72, 20)));
    }

    pub fn set_search_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg((0, 0, 0)));
        str_vec.push(Colors::bg((221, 72, 20)));
    }

    pub fn set_new_line_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg((110, 110, 110)));
        str_vec.push(Colors::bg((0, 0, 0)));
    }

    pub fn get_sber_bg() -> String {
        return Colors::bg((0, 0, 0));
    }

    pub fn get_sber_fg() -> String {
        return Colors::fg((135, 65, 31));
    }

    pub fn get_default_fg() -> String {
        return Colors::fg((255, 255, 255));
    }

    pub fn get_default_bg() -> String {
        return Colors::bg((0, 0, 0));
    }

    pub fn get_msg_fg() -> String {
        // lime
        return Colors::fg((0, 255, 0));
    }
    pub fn get_msg_warning_fg() -> String {
        // orange
        return Colors::fg((255, 165, 0));
    }

    pub fn get_msg_err_fg() -> String {
        return Colors::fg((255, 0, 0));
    }

    pub fn set_eof(str_vec: &mut Vec<String>) {
        Colors::set_new_line_color(str_vec);
        str_vec.push(EOF_STR.to_string());
        Colors::set_textarea_color(str_vec);
    }
}
