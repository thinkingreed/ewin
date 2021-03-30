use crate::{def::*, global::*};
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
pub struct Colors {}

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
    pub fn set_text_color(str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        str_vec.push(Colors::fg(cfg.colors.editor.fg));
        str_vec.push(Colors::bg(cfg.colors.editor.bg));
    }
    pub fn set_rownum_curt_color(str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        str_vec.push(Colors::fg(cfg.colors.editor.line_number.fg));
        str_vec.push(Colors::bg(cfg.colors.editor.bg));
    }
    pub fn set_rownum_color(str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        str_vec.push(Colors::fg(cfg.colors.editor.line_number.fg));
        str_vec.push(Colors::bg(cfg.colors.editor.line_number.bg));
    }

    pub fn set_select_color(str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        str_vec.push(Colors::fg(cfg.colors.editor.selection.fg));
        str_vec.push(Colors::bg(cfg.colors.editor.selection.bg));
    }
    //
    // default
    //
    pub fn get_default_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.fg);
    }
    fn get_default_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_default_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_default_fg_bg() -> String {
        return format!("{}{}", Colors::get_default_bg(), Colors::get_default_fg());
    }
    //
    // sber
    //
    pub fn get_sber_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_sber_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.status_bar.fg);
    }
    pub fn get_sber_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.status_bar.fg);
    }
    pub fn get_sber_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_sber_inversion_bg(), Colors::get_default_inversion_fg());
    }
    //
    // msg
    //
    pub fn get_msg_highlight_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.msg.highlight_fg);
    }
    pub fn get_msg_normal_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.msg.normal_fg);
    }
    pub fn get_msg_warning_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.msg.warning_fg);
    }
    pub fn get_msg_warning_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.msg.warning_fg);
    }
    pub fn get_msg_warning_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_msg_warning_inversion_bg(), Colors::get_default_inversion_fg());
    }
    pub fn get_msg_err_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.msg.err_fg);
    }
    //
    // eof
    //
    pub fn set_eof(str_vec: &mut Vec<String>) {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        str_vec.push(Colors::fg(cfg.colors.editor.control_char.fg));
        str_vec.push(EOF_STR.to_string());
        str_vec.push(Colors::fg(cfg.colors.editor.fg));
    }
    pub fn hex2rgb(hex: &str) -> Color {
        let rgb2 = transform_Rgb::from_hex_str(hex).unwrap();
        let t = rgb2.as_tuple();
        return Color { rgb: Rgb { r: t.0 as u8, g: t.1 as u8, b: t.2 as u8 } };
    }

    fn fg(c: Color) -> String {
        SetForegroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }
    pub fn bg(c: Color) -> String {
        SetBackgroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }
}
