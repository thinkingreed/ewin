use crate::model::general::default::*;
use colors_transform::{Color as transform_Color, Rgb as transform_Rgb};
use crossterm::style::{Color as CrosstermColor, SetBackgroundColor, SetForegroundColor};
use serde::{Deserialize, Serialize};
use std::usize;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub rgb: Rgb,
}
pub struct Colors {}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Colors {
    //
    // default
    //
    pub fn get_default_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.fg)
    }
    pub fn get_default_bg() -> String {
        Colors::bg(Cfg::get().colors.editor.bg)
    }
    pub fn get_default_fg_bg() -> String {
        format!("{}{}", Colors::get_default_bg(), Colors::get_default_fg())
    }
    fn get_default_inversion_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.bg)
    }
    pub fn get_default_inversion_fg_bg() -> String {
        format!("{}{}", Colors::fg(Cfg::get().colors.editor.bg), Colors::bg(Cfg::get().colors.editor.fg))
    }
    //
    // Row no
    //
    pub fn get_rownum_curt_fg_bg() -> String {
        format!("{}{}", Colors::fg(Cfg::get().colors.editor.line_number.active_fg), Colors::bg(Cfg::get().colors.editor.line_number.active_bg))
    }
    pub fn get_rownum_not_curt_fg_bg() -> String {
        let cfg = Cfg::get();
        let mut s = String::new();
        if cfg.colors.editor.line_number.passive_fg != cfg.colors.editor.fg {
            s.push_str(&Colors::fg(Cfg::get().colors.editor.line_number.passive_fg));
        }
        if cfg.colors.editor.line_number.passive_bg != cfg.colors.editor.bg {
            s.push_str(&Colors::bg(Cfg::get().colors.editor.line_number.passive_bg));
        }
        return s;
    }
    //
    // Select
    //
    pub fn get_select_fg_bg() -> String {
        return format!("{}{}", Colors::fg(Cfg::get().colors.editor.selection.fg), Colors::bg(Cfg::get().colors.editor.selection.bg));
    }
    //
    // Window
    //
    pub fn get_window_split_line_bg() -> String {
        Colors::bg(Cfg::get().colors.editor.window.split_line.bg)
    }
    //
    // Scale
    //
    pub fn get_scale_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.scale.fg)
    }
    pub fn get_scale_bg() -> String {
        Colors::bg(Cfg::get().colors.editor.scale.bg)
    }
    pub fn get_scale_fg_bg() -> String {
        format!("{}{}", Colors::get_scale_fg(), Colors::get_scale_bg())
    }
    //
    // Scrollbar
    //
    pub fn get_scrollbar_v_bg() -> String {
        Colors::bg(Cfg::get().colors.system.scrollbar.bg_vertical)
    }
    pub fn get_scrollbar_h_bg() -> String {
        Colors::bg(Cfg::get().colors.system.scrollbar.bg_horizontal)
    }
    //
    // Sytem.btn
    //
    pub fn get_sytem_btn_bg() -> String {
        Colors::bg(Cfg::get().colors.system.btn.bg)
    }
    pub fn get_sytem_btn_fg() -> String {
        Colors::fg(Cfg::get().colors.system.btn.fg)
    }
    pub fn get_sytem_btn_fg_bg() -> String {
        format!("{}{}", Colors::get_sytem_btn_bg(), Colors::get_sytem_btn_fg())
    }
    //
    // Sytem.state
    //
    pub fn get_sytem_state_bg() -> String {
        Colors::bg(Cfg::get().colors.system.state.bg)
    }
    pub fn get_sytem_state_fg() -> String {
        Colors::fg(Cfg::get().colors.system.state.fg)
    }
    //
    // MenuBar
    //
    pub fn get_menubar_passive_fg() -> String {
        Colors::fg(Cfg::get().colors.menubar.fg_passive)
    }
    pub fn get_menubar_passive_bg() -> String {
        Colors::bg(Cfg::get().colors.menubar.bg_passive)
    }
    pub fn get_menubar_passive_fg_bg() -> String {
        format!("{}{}", Colors::get_menubar_passive_bg(), Colors::get_menubar_passive_fg())
    }
    pub fn get_menubar_bg_active() -> String {
        Colors::bg(Cfg::get().colors.menubar.bg_active)
    }
    pub fn get_menubar_fg_active() -> String {
        Colors::fg(Cfg::get().colors.menubar.fg_active)
    }
    pub fn get_menubar_active_fg_bg() -> String {
        format!("{}{}", Colors::get_menubar_bg_active(), Colors::get_menubar_fg_active())
    }
    pub fn get_menubar_default_bg() -> String {
        Colors::bg(Cfg::get().colors.menubar.bg_default)
    }
    //
    // FileBar
    //
    pub fn get_filebar_passive_fg() -> String {
        Colors::fg(Cfg::get().colors.filebar.fg_passive)
    }
    pub fn get_filebar_passive_bg() -> String {
        Colors::bg(Cfg::get().colors.filebar.bg_passive)
    }
    pub fn get_filebar_passive_fg_bg() -> String {
        format!("{}{}", Colors::get_filebar_passive_bg(), Colors::get_filebar_passive_fg())
    }
    pub fn get_filebar_bg_active() -> String {
        Colors::bg(Cfg::get().colors.filebar.bg_active)
    }
    pub fn get_filebar_fg_active() -> String {
        Colors::fg(Cfg::get().colors.filebar.fg_active)
    }
    pub fn get_filebar_active_fg_bg() -> String {
        format!("{}{}", Colors::get_filebar_bg_active(), Colors::get_filebar_fg_active())
    }
    pub fn get_filebar_default_bg() -> String {
        Colors::bg(Cfg::get().colors.filebar.bg_default)
    }
    //
    // StatusBar
    //
    pub fn get_statusbar_bg() -> String {
        Colors::bg(Cfg::get().colors.statusbar.bg)
    }
    pub fn get_statusbar_fg() -> String {
        Colors::fg(Cfg::get().colors.statusbar.fg)
    }
    pub fn get_statusbar_fg_bg() -> String {
        format!("{}{}", Colors::get_statusbar_fg(), Colors::get_statusbar_bg())
    }
    pub fn get_statusbar_inversion_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.bg)
    }
    pub fn get_statusbar_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.statusbar.fg)
    }
    pub fn get_statusbar_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_statusbar_inversion_fg(), Colors::get_statusbar_inversion_bg())
    }
    //
    // MsgBar
    //
    pub fn get_msg_highlight_fg() -> String {
        Colors::fg(Cfg::get().colors.msg.highlight_fg)
    }
    pub fn get_msg_highlight_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.msg.highlight_fg)
    }
    pub fn get_msg_highlight_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_msg_highlight_inversion_bg(), Colors::get_default_inversion_fg())
    }
    pub fn get_msg_normal_fg() -> String {
        Colors::fg(Cfg::get().colors.msg.normal_fg)
    }
    pub fn get_msg_warning_fg() -> String {
        Colors::fg(Cfg::get().colors.msg.warning_fg)
    }
    pub fn get_msg_warning_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.msg.warning_fg)
    }
    pub fn get_msg_warning_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_msg_warning_inversion_bg(), Colors::get_default_inversion_fg())
    }
    pub fn get_msg_err_fg() -> String {
        Colors::fg(Cfg::get().colors.msg.err_fg)
    }
    pub fn get_msg_bg() -> String {
        Colors::bg(Cfg::get().colors.msg.bg)
    }
    //
    // CtxMenu
    //
    pub fn get_ctx_menu_bg_sel() -> String {
        Colors::bg(Cfg::get().colors.ctx_menu.bg_sel)
    }
    pub fn get_ctx_menu_bg_non_sel() -> String {
        Colors::bg(Cfg::get().colors.ctx_menu.bg_non_sel)
    }
    pub fn get_ctx_menu_fg_sel() -> String {
        Colors::fg(Cfg::get().colors.ctx_menu.fg_sel)
    }
    pub fn get_ctx_menu_fg_non_sel() -> String {
        Colors::fg(Cfg::get().colors.ctx_menu.fg_non_sel)
    }
    pub fn get_ctx_menu_fg_bg_sel() -> String {
        format!("{}{}", Colors::get_ctx_menu_bg_sel(), Colors::get_ctx_menu_fg_sel())
    }
    pub fn get_ctx_menu_fg_bg_non_sel() -> String {
        format!("{}{}", Colors::get_ctx_menu_bg_non_sel(), Colors::get_ctx_menu_fg_non_sel())
    }
    //
    // Dialog
    //
    fn get_dialog_fg_default() -> String {
        Colors::fg(Cfg::get().colors.dialog.fg_default)
    }
    pub fn get_dialog_bg_default() -> String {
        Colors::bg(Cfg::get().colors.dialog.bg_default)
    }
    pub fn get_dialog_fg_bg_default() -> String {
        format!("{}{}", Colors::get_dialog_fg_default(), Colors::get_dialog_bg_default())
    }
    fn get_dialog_fg_header() -> String {
        Colors::fg(Cfg::get().colors.dialog.fg_header)
    }
    fn get_dialog_bg_header() -> String {
        Colors::bg(Cfg::get().colors.dialog.bg_header)
    }
    pub fn get_dialog_fg_bg_header() -> String {
        format!("{}{}", Colors::get_dialog_fg_header(), Colors::get_dialog_bg_header())
    }
    pub fn get_dialog_bg_sel() -> String {
        Colors::bg(Cfg::get().colors.dialog.bg_sel)
    }

    //
    // File
    //
    pub fn get_file_normal_fg() -> String {
        Colors::fg(Cfg::get().colors.file.normal_fg)
    }
    pub fn get_file_normal_bg() -> String {
        Colors::get_default_bg()
    }
    pub fn get_file_normal_fg_bg() -> String {
        format!("{}{}", Colors::get_file_normal_fg(), Colors::get_file_normal_bg())
    }
    pub fn get_file_normal_inversion_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.bg)
    }
    pub fn get_file_normal_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.file.normal_fg)
    }
    pub fn get_file_normal_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_file_normal_inversion_fg(), Colors::get_file_normal_inversion_bg())
    }

    pub fn get_file_dir_fg() -> String {
        Colors::fg(Cfg::get().colors.file.directory_fg)
    }
    pub fn get_file_dir_bg() -> String {
        Colors::get_default_bg()
    }
    pub fn get_file_dir_fg_bg() -> String {
        format!("{}{}", Colors::get_file_dir_fg(), Colors::get_file_dir_bg())
    }
    pub fn get_file_dir_inversion_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.bg)
    }
    pub fn get_file_dir_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.file.directory_fg)
    }
    pub fn get_file_dir_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_file_dir_inversion_fg(), Colors::get_file_dir_inversion_bg())
    }
    pub fn get_file_executable_fg() -> String {
        Colors::fg(Cfg::get().colors.file.executable_fg)
    }
    pub fn get_file_executable_bg() -> String {
        Colors::get_default_bg()
    }
    pub fn get_file_executable_fg_bg() -> String {
        format!("{}{}", Colors::get_file_executable_fg(), Colors::get_file_executable_bg())
    }
    pub fn get_file_executable_inversion_fg() -> String {
        Colors::fg(Cfg::get().colors.editor.bg)
    }
    pub fn get_file_executable_inversion_bg() -> String {
        Colors::bg(Cfg::get().colors.file.executable_fg)
    }
    pub fn get_file_executable_inversion_fg_bg() -> String {
        format!("{}{}", Colors::get_file_executable_inversion_fg(), Colors::get_file_executable_inversion_bg())
    }

    //
    // SideBar
    //
    fn get_sidebar_fg() -> String {
        Colors::fg(Cfg::get().colors.sidebar.fg)
    }
    pub fn get_sidebar_bg() -> String {
        Colors::bg(Cfg::get().colors.sidebar.bg)
    }
    pub fn get_sidebar_bg_header() -> String {
        Colors::bg(Cfg::get().colors.sidebar.bg_header)
    }
    pub fn get_sidebar_bg_open_file() -> String {
        Colors::bg(Cfg::get().colors.sidebar.bg_open_file)
    }
    pub fn get_sidebar_fg_bg() -> String {
        format!("{}{}", Colors::get_sidebar_fg(), Colors::get_sidebar_bg())
    }

    pub fn hex2rgb(hex: &str) -> Color {
        let rgb2 = transform_Rgb::from_hex_str(hex).unwrap();
        let t = rgb2.as_tuple();
        Color { rgb: Rgb { r: t.0 as u8, g: t.1 as u8, b: t.2 as u8 } }
    }

    pub fn fg(c: Color) -> String {
        SetForegroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }
    pub fn bg(c: Color) -> String {
        SetBackgroundColor(CrosstermColor::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }).to_string()
    }
}

impl From<syntect::highlighting::Color> for Color {
    fn from(scolor: syntect::highlighting::Color) -> Self {
        Self { rgb: Rgb { r: scolor.r, g: scolor.g, b: scolor.b } }
    }
}
impl From<Color> for CrosstermColor {
    fn from(c: Color) -> crossterm::style::Color {
        crossterm::style::Color::Rgb { r: c.rgb.r, g: c.rgb.g, b: c.rgb.b }
    }
}
