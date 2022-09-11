use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColors {
    // #[serde(skip_serializing)]
    #[serde(default)]
    pub system: CfgUserColorSystem,
    #[serde(default)]
    pub theme: CfgUserColorTheme,
    #[serde(default)]
    pub menubar: CfgUserColorMenuBar,
    #[serde(default)]
    pub filebar: CfgUserColorFileBar,
    #[serde(default)]
    pub editor: CfgUserColorEditor,
    #[serde(default)]
    pub statusbar: CfgUserColorStatusBar,
    #[serde(default)]
    pub ctx_menu: CfgUserColorCtxMenu,
    #[serde(default)]
    pub msg: CfgUserColorMsg,
    #[serde(default)]
    pub file: CfgUserColorFile,
    #[serde(default)]
    pub dialog: CfgUserColorDialog,
    #[serde(default)]
    pub sdiebar: CfgUserColorSideBar,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorSystem {
    pub btn: CfgUserColorSystemBtn,
    pub state: CfgUserColorSystemState,
    pub scrollbar: CfgUserColorScrollbar,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorSystemBtn {
    pub background: Option<String>,
    pub foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorSystemState {
    pub background: Option<String>,
    pub foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorTheme {
    pub highlight_theme_path: Option<String>,
    pub highlight_theme_background_enable: Option<bool>,
    #[serde(skip_deserializing, skip_serializing)]
    pub theme_bg_enable: bool,
    pub disable_highlight_ext: Option<Vec<String>>,
    pub disable_syntax_highlight_file_size: Option<usize>,
    pub default_color_theme: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserTmpColors {
    pub colors: CfgUserColors,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditor {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub line_number: CfgUserColorEditorLineNumber,
    pub selection: CfgUserColorEditorSelection,
    pub search: CfgUserColorEditorSearch,
    pub control_char: CfgUserColorEditorControlChar,
    pub column_char_width_gap_space: CfgUserColorEditorColumnCharWidthGap,
    pub scale: CfgUserColorEditorScale,
    pub window: Option<CfgUserColorEditorWindow>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorLineNumber {
    pub active_background: Option<String>,
    pub active_foreground: Option<String>,
    pub passive_background: Option<String>,
    pub passive_foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorSelection {
    pub background: Option<String>,
    pub foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorSearch {
    pub background: Option<String>,
    pub foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorControlChar {
    pub foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorColumnCharWidthGap {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorScrollbar {
    pub horizontal_background: Option<String>,
    pub vertical_background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorScale {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorWindow {
    pub split_line: Option<CfgUserColorEditorWindowSplitLine>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorEditorWindowSplitLine {
    pub background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "MenuBar")]
pub struct CfgUserColorMenuBar {
    pub active_foreground: Option<String>,
    pub active_background: Option<String>,
    pub passive_foreground: Option<String>,
    pub passive_background: Option<String>,
    pub default_background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "FileBar")]
pub struct CfgUserColorFileBar {
    pub active_background: Option<String>,
    pub active_foreground: Option<String>,
    pub passive_background: Option<String>,
    pub passive_foreground: Option<String>,
    pub default_background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgUserColorStatusBar {
    pub foreground: Option<String>,
    pub background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "CtxMenu")]
pub struct CfgUserColorCtxMenu {
    pub non_select_background: Option<String>,
    pub select_background: Option<String>,
    pub non_select_foreground: Option<String>,
    pub select_foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "Msg")]
pub struct CfgUserColorMsg {
    pub normal_foreground: Option<String>,
    pub highlight_foreground: Option<String>,
    pub warning_foreground: Option<String>,
    pub err_foreground: Option<String>,
    pub background: Option<String>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "File")]
pub struct CfgUserColorFile {
    pub normal_foreground: Option<String>,
    pub directory_foreground: Option<String>,
    pub executable_foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "Dialog")]

pub struct CfgUserColorDialog {
    pub default_foreground: Option<String>,
    pub default_background: Option<String>,
    pub header_foreground: Option<String>,
    pub header_background: Option<String>,
    pub select_background: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "SideBar")]
pub struct CfgUserColorSideBar {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub header_background: Option<String>,
    pub open_file_background: Option<String>,
}
