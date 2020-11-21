use crate::model::*;
use termion::color;

impl Editor {
    pub fn set_rownum_color(&mut self, str_vec: &mut Vec<String>) {
        str_vec.push(color::Fg(color::Rgb(0, 100, 0)).to_string());

        //str_vec.push(color::Fg(color::Rgb(80, 80, 80)).to_string());

        str_vec.push(color::Bg(color::Black).to_string());
    }

    pub fn set_textarea_color(&mut self, str_vec: &mut Vec<String>) {
        str_vec.push(color::Fg(color::White).to_string());
        str_vec.push(color::Bg(color::Black).to_string());
    }

    pub fn set_select_color(&mut self, str_vec: &mut Vec<String>) {
        str_vec.push(color::Fg(color::Black).to_string());
        // 薄黄色
        // str_vec.push(color::Bg(color::Rgb(255, 250, 205)).to_string());
        // オレンジ色
        str_vec.push(color::Bg(color::Rgb(221, 72, 20)).to_string());
    }

    pub fn set_search_color(&mut self, str_vec: &mut Vec<String>) {
        str_vec.push(color::Fg(color::Black).to_string());
        // オレンジ色
        str_vec.push(color::Bg(color::Rgb(221, 72, 20)).to_string());
        // 薄黄色
        // str_vec.push(color::Bg(color::Rgb(255, 250, 205)).to_string());
    }
}
