use crate::colors::*;
use serde::{Deserialize, Serialize};

use syntect::{
    self,
    highlighting::{Highlighter, Theme, ThemeSet},
    parsing::SyntaxSet,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUser {
    #[serde(default)]
    pub general: CfgUserGeneral,
    #[serde(skip_deserializing)]
    pub colors: CfgUserColors,
    #[serde(default)]
    pub system: CfgUserSystem,
}

#[derive(Debug, Default)]
pub struct CfgUserSyntax<'a> {
    pub syntax: Syntax,
    pub highlighter_opt: Option<Highlighter<'a>>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserGeneral {
    pub lang: Option<String>,
    pub log: CfgUserLog,
    pub editor: CfgUserEditor,
    pub font: CfgUserFont,
    pub prompt: CfgUserPrompt,
    #[serde(default)]
    pub context_menu: CfgUserCtxMenu,
    #[serde(default)]
    pub mouse: CfgGeneralMouse,
    pub colors: CfgUserGeneralColors,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserLog {
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditor {
    #[serde(default)]
    pub search: CfgUserSearch,
    #[serde(default)]
    pub tab: CfgUserTab,
    #[serde(default)]
    pub format: CfgUserFormat,
    #[serde(default)]
    pub scrollbar: CfgUserScrl,
    #[serde(default)]
    pub cursor: CfgUserCur,
    #[serde(default)]
    pub column_char_width_gap_space: CfgUserEditorColumnCharWidthGap,
    #[serde(default)]
    pub save: CfgUserEditorSave,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserFont {
    pub ambiguous_width: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserPrompt {
    pub open_file: CfgUserPromptOpenFile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserCtxMenu {
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserSearch {
    pub case_sens: Option<bool>,
    pub regex: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserTab {
    pub size: Option<usize>,
    pub input_type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserFormat {
    pub indent_type: Option<String>,
    pub indent_size: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserScrl {
    pub vertical: CfgUserScrlVertical,
    pub horizontal: CfgUserScrlHorizontal,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserScrlVertical {
    pub width: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserScrlHorizontal {
    pub height: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserCur {
    pub move_position_by_scrolling_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorColumnCharWidthGap {
    pub character: Option<char>,
    pub end_of_line_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorSave {
    pub use_string_first_line_for_file_name_of_new_file: Option<bool>,
    pub extension_when_saving_new_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgUserPromptOpenFile {
    pub directory_init_value: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgGeneralMouse {
    pub mouse_enable: Option<bool>,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserGeneralColors {
    pub theme: CfgUserColorTheme,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColors {
    // #[serde(skip_serializing)]
    #[serde(default)]
    pub system: CfgUserColorSystem,
    pub headerbar: CfgUserColorHeaderBar,
    pub editor: CfgUserColorEditor,
    pub statusbar: CfgUserColorStatusBar,
    pub ctx_menu: CfgUserColorCtxMenu,
    pub msg: CfgUserColorMsg,
    pub file: CfgUserColorFile,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorSystem {
    pub btn: CfgUserColorSystemBtn,
    pub state: CfgUserColorSystemState,
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
    pub scrollbar: CfgUserColorScrollbar,
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
#[serde(rename = "HeaderBar")]
pub struct CfgUserColorHeaderBar {
    pub tab_active_background: Option<String>,
    pub tab_active_foreground: Option<String>,
    pub tab_passive_background: Option<String>,
    pub tab_passive_foreground: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgUserColorStatusBar {
    pub foreground: Option<String>,
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
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename = "File")]
pub struct CfgUserColorFile {
    pub normal_foreground: Option<String>,
    pub directory_foreground: Option<String>,
    pub executable_foreground: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Syntax {
    pub syntax_set: SyntaxSet,
    pub theme: Theme,
    pub theme_set: ThemeSet,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Syntax {
    fn default() -> Self {
        Syntax { syntax_set: SyntaxSet::load_defaults_newlines(), theme_set: ThemeSet::load_defaults(), theme: Theme::default(), fg: Color::default(), bg: Color::default() }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserSystem {
    pub os: CfgUserOS,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserOS {
    pub windows: CfgUserWindows,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserWindows {
    pub change_output_encoding_utf8: Option<bool>,
}
