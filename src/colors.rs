use crate::{cfg::cfg::*, def::*, global::*};
use colors_transform::{Color as transform_Color, Rgb as transform_Rgb};
use crossterm::style::{Color as CrosstermColor, SetBackgroundColor, SetForegroundColor};
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub rgb: Rgb,
}
impl Default for Color {
    fn default() -> Self {
        Color { rgb: Rgb::default() }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Default for Rgb {
    fn default() -> Self {
        Rgb { r: 0, g: 0, b: 0 }
    }
}
impl Colors {
    pub fn set_rownum_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg_ex(CFG.get().unwrap().colors.editor.line_number.fg));
        str_vec.push(Colors::bg_ex(CFG.get().unwrap().colors.editor.line_number.bg));
    }
    pub fn set_text_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg_ex(CFG.get().unwrap().colors.editor.fg));
        str_vec.push(Colors::bg_ex(CFG.get().unwrap().colors.editor.bg));
    }
    pub fn set_select_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg_ex(CFG.get().unwrap().colors.editor.selection.fg));
        str_vec.push(Colors::bg_ex(CFG.get().unwrap().colors.editor.selection.bg));
    }
    pub fn set_new_line_color(str_vec: &mut Vec<String>) {
        str_vec.push(Colors::fg_ex(CFG.get().unwrap().colors.editor.fg));
        str_vec.push(Colors::bg_ex(CFG.get().unwrap().colors.editor.bg));
    }
    pub fn get_sber_bg() -> String {
        return Colors::bg_ex(CFG.get().unwrap().colors.status_bar.bg);
    }
    pub fn get_sber_fg() -> String {
        return Colors::fg_ex(CFG.get().unwrap().colors.status_bar.fg);
    }
    pub fn get_default_fg() -> String {
        return Colors::fg_ex(CFG.get().unwrap().colors.editor.fg);
    }
    pub fn get_default_bg() -> String {
        return Colors::bg_ex(CFG.get().unwrap().colors.editor.bg);
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
        Colors::set_text_color(str_vec);
    }

    pub fn hex2rgb(hex: &str) -> Color {
        let rgb2 = transform_Rgb::from_hex_str(hex).unwrap();
        let t = rgb2.as_tuple();
        return Color { rgb: Rgb { r: t.0 as u8, g: t.1 as u8, b: t.2 as u8 } };
    }

    pub fn fg_ex(c: Color) -> String {
        SetForegroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }

    pub fn bg_ex(c: Color) -> String {
        SetBackgroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }

    pub fn fg(rgb: (u8, u8, u8)) -> String {
        SetForegroundColor(CrosstermColor::Rgb { r: rgb.0, g: rgb.1, b: rgb.2 }).to_string()
    }

    pub fn bg(rgb: (u8, u8, u8)) -> String {
        SetBackgroundColor(CrosstermColor::Rgb { r: rgb.0, g: rgb.1, b: rgb.2 }).to_string()
    }
}
