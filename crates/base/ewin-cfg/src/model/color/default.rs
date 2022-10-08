use crate::colors::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColors {
    pub system: CfgColorSystem,
    pub theme: CfgColorTheme,
    pub menubar: CfgColorMenuBar,
    pub filebar: CfgColorFileBar,
    pub editor: CfgColorEditor,
    pub statusbar: CfgColorStatusBar,
    pub ctx_menu: CfgColorCtxMenu,
    pub msg: CfgColorMsg,
    pub file: CfgColorFile,
    pub dialog: CfgColorDialog,
    pub sidebar: CfgColorSideBar,
    pub activitybar: CfgColorActivityBar,
    pub tooltip: CfgColorToolTip,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorSystem {
    pub btn: CfgColorSystemBtn,
    pub state: CfgColorSystemState,
    pub scrollbar: CfgColorSystemScrollbar,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorSystemBtn {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorSystemState {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorTheme {
    pub highlight_theme_path: Option<String>,
    pub highlight_theme_background_enable: Option<bool>,
    #[serde(skip_deserializing, skip_serializing)]
    pub theme_bg_enable: bool,
    pub disable_highlight_ext: Vec<String>,
    pub disable_syntax_highlight_file_size: usize,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum ColorSchemeThemeType {
    Black,
    #[default]
    White,
}
impl ColorSchemeThemeType {
    pub fn from_str_color_type(s: &str) -> ColorSchemeThemeType {
        match s {
            "black" => return ColorSchemeThemeType::Black,
            "white" => return ColorSchemeThemeType::White,
            _ => return ColorSchemeThemeType::default(),
        }
    }
}

impl fmt::Display for ColorSchemeThemeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorSchemeThemeType::White => write!(f, "white"),
            ColorSchemeThemeType::Black => write!(f, "black"),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgTmpColors {
    pub colors: CfgColors,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditor {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    pub line_number: CfgColorEditorLineNumber,
    pub selection: CfgColorEditorSelection,
    pub search: CfgColorEditorSearch,
    pub control_char: CfgColorEditorControlChar,
    pub column_char_width_gap_space: CfgColorEditorColumnCharWidthGapSpace,
    pub scale: CfgColorEditorScale,
    pub window: CfgColorEditorWindow,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorLineNumber {
    pub active_background: String,
    pub active_foreground: String,
    pub passive_background: String,
    pub passive_foreground: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub active_bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub active_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub passive_bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub passive_fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorSelection {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorSearch {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorControlChar {
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorColumnCharWidthGapSpace {
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    pub background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorSystemScrollbar {
    pub horizontal_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_horizontal: Color,
    pub vertical_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_vertical: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorScale {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorWindow {
    pub split_line: CfgColorEditorWindowSplitLine,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorWindowSplitLine {
    pub background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "MenuBar")]
pub struct CfgColorMenuBar {
    pub active_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_active: Color,
    pub active_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_active: Color,
    pub passive_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_passive: Color,
    pub passive_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_passive: Color,
    pub default_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_default: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "FileBar")]
pub struct CfgColorFileBar {
    pub active_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_active: Color,
    pub active_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_active: Color,
    pub passive_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_passive: Color,
    pub passive_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_passive: Color,
    pub default_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_default: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgColorStatusBar {
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    pub background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "CtxMenu")]
pub struct CfgColorCtxMenu {
    pub non_select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_non_sel: Color,
    pub select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_sel: Color,
    pub non_select_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_non_sel: Color,
    pub select_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_sel: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "Dialog")]
pub struct CfgColorDialog {
    pub default_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_default: Color,
    pub default_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_default: Color,
    pub header_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_header: Color,
    pub header_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_header: Color,
    pub select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_sel: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "SideBar")]
pub struct CfgColorSideBar {
    pub foreground: String,
    pub background: String,
    pub header_background: String,
    pub open_file_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_header: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_open_file: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "ActivityBar")]
pub struct CfgColorActivityBar {
    pub default_background: String,
    pub select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_default: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_select: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "ToolTip")]
pub struct CfgColorToolTip {
    pub background: String,
    pub foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "Msg")]
pub struct CfgColorMsg {
    pub normal_foreground: String,
    pub highlight_foreground: String,
    pub warning_foreground: String,
    pub err_foreground: String,
    pub background: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub normal_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub highlight_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub warning_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub err_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "File")]
pub struct CfgColorFile {
    pub normal_foreground: String,
    pub directory_foreground: String,
    pub executable_foreground: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub normal_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub directory_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub executable_fg: Color,
}
