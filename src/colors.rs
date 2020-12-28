use crate::model::*;
use termion::color::*;

impl Colors {
    pub fn set_rownum_color(str_vec: &mut Vec<String>) {
        str_vec.push(Fg(Rgb(110, 110, 110)).to_string());

        str_vec.push(Bg(Rgb(0, 0, 0)).to_string());
    }

    pub fn set_textarea_color(str_vec: &mut Vec<String>) {
        str_vec.push(Fg(Rgb(255, 255, 255)).to_string());
        str_vec.push(Bg(Rgb(0, 0, 0)).to_string());
    }

    pub fn set_select_color(str_vec: &mut Vec<String>) {
        str_vec.push(Fg(Rgb(0, 0, 0)).to_string());
        // オレンジ
        str_vec.push(Bg(Rgb(221, 72, 20)).to_string());
    }

    pub fn set_search_color(str_vec: &mut Vec<String>) {
        str_vec.push(Fg(Rgb(0, 0, 0)).to_string());
        // オレンジ
        str_vec.push(Bg(Rgb(221, 72, 20)).to_string());
    }

    pub fn set_new_line_color(str_vec: &mut Vec<String>) {
        str_vec.push(Fg(Rgb(110, 110, 110)).to_string());

        str_vec.push(Bg(Rgb(0, 0, 0)).to_string());
    }

    pub fn get_sber_bg() -> String {
        return Bg(Rgb(0, 0, 0)).to_string();
    }

    pub fn get_sber_fg() -> String {
        return Fg(Rgb(135, 65, 31)).to_string();
    }

    pub fn get_default_fg() -> String {
        return Fg(Rgb(255, 255, 255)).to_string();
    }

    pub fn get_default_bg() -> String {
        return Bg(Rgb(0, 0, 0)).to_string();
    }

    pub fn get_msg_fg() -> String {
        // lime
        return Fg(Rgb(0, 255, 0)).to_string();
    }
    pub fn get_msg_warning_fg() -> String {
        // orange
        return Fg(Rgb(255, 165, 0)).to_string();
    }

    pub fn get_msg_err_fg() -> String {
        return Fg(Rgb(255, 0, 0)).to_string();
    }
}
