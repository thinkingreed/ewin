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
    pub fn get_default_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_default_fg_bg() -> String {
        return format!("{}{}", Colors::get_default_bg(), Colors::get_default_fg());
    }
    fn get_default_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_default_inversion_fg_bg() -> String {
        let cfg = CFG.get().unwrap().try_lock().unwrap();
        return format!("{}{}", Colors::fg(cfg.colors.editor.bg), Colors::bg(cfg.colors.editor.fg));
    }
    //
    // HeaderBar
    //
    fn get_hbar_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    fn get_hbar_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.header_bar.fg);
    }
    pub fn get_hbar_fg_bg() -> String {
        return format!("{}{}", Colors::get_hbar_fg(), Colors::get_hbar_bg());
    }
    pub fn get_hbar_inversion_bg_passive() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.control_char.fg);
    }
    pub fn get_hbar_inversion_fg_bg_passive() -> String {
        return format!("{}{}", Colors::get_hbar_inversion_bg_passive(), Colors::get_default_inversion_fg());
    }
    pub fn get_hbar_inversion_bg_active() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.header_bar.fg);
    }
    pub fn get_hbar_inversion_fg_bg_active() -> String {
        return format!("{}{}", Colors::get_hbar_inversion_bg_active(), Colors::get_default_inversion_fg());
    }
    //
    // StatusBar
    //
    pub fn get_sbar_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_sbar_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.status_bar.fg);
    }
    pub fn get_sbar_fg_bg() -> String {
        return format!("{}{}", Colors::get_sbar_fg(), Colors::get_sbar_bg());
    }
    pub fn get_sbar_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_sbar_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.status_bar.fg);
    }
    pub fn get_sbar_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_sbar_inversion_fg(), Colors::get_sbar_inversion_bg());
    }
    //
    // MsgBar
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
    // File
    //
    pub fn get_file_normal_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.file.normal_fg);
    }
    pub fn get_file_normal_bg() -> String {
        return Colors::get_default_bg();
    }
    pub fn get_file_normal_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_normal_fg(), Colors::get_file_normal_bg());
    }
    pub fn get_file_normal_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_file_normal_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.file.normal_fg);
    }
    pub fn get_file_normal_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_normal_inversion_fg(), Colors::get_file_normal_inversion_bg());
    }

    pub fn get_file_dir_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.file.directory_fg);
    }
    pub fn get_file_dir_bg() -> String {
        return Colors::get_default_bg();
    }
    pub fn get_file_dir_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_dir_fg(), Colors::get_file_dir_bg());
    }
    pub fn get_file_dir_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_file_dir_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.file.directory_fg);
    }
    pub fn get_file_dir_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_dir_inversion_fg(), Colors::get_file_dir_inversion_bg());
    }

    pub fn get_file_executable_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.file.executable_fg);
    }
    pub fn get_file_executable_bg() -> String {
        return Colors::get_default_bg();
    }
    pub fn get_file_executable_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_executable_fg(), Colors::get_file_executable_bg());
    }
    pub fn get_file_executable_inversion_fg() -> String {
        return Colors::fg(CFG.get().unwrap().try_lock().unwrap().colors.editor.bg);
    }
    pub fn get_file_executable_inversion_bg() -> String {
        return Colors::bg(CFG.get().unwrap().try_lock().unwrap().colors.file.executable_fg);
    }
    pub fn get_file_executable_inversion_fg_bg() -> String {
        return format!("{}{}", Colors::get_file_executable_inversion_fg(), Colors::get_file_executable_inversion_bg());
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
