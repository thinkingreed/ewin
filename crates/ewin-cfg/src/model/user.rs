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
    pub menubar: CfgUserMenubar,
    #[serde(default)]
    pub mouse: CfgUserGeneralMouse,
    #[serde(default)]
    pub word: CfgUserEditorWord,
    pub colors: CfgUserGeneralColors,
    #[serde(default)]
    pub view: CfgUserGeneralView,
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
    #[serde(default)]
    pub word: CfgUserEditorWord,
    #[serde(default)]
    pub input_comple: CfgUserEditorInputComple,
    #[serde(default)]
    pub row_no: CfgUserEditorRowNo,
    #[serde(default)]
    pub scale: CfgUserEditorScale,
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
pub struct CfgUserMenubar {
    pub content: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserSearch {
    pub case_sensitive: Option<bool>,
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
    pub candidate_extension_when_saving_new_file: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorWord {
    pub word_delimiter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgUserEditorInputComple {
    pub word_delimiter: Option<String>,
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgUserEditorRowNo {
    pub is_enable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgUserEditorScale {
    pub is_enable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgUserPromptOpenFile {
    pub directory_init_value: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserGeneralMouse {
    pub mouse_enable: Option<bool>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserGeneralView {
    pub tab_characters_as_symbols: Option<String>,
    pub full_width_space_characters_as_symbols: Option<String>,
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
    pub scale: CfgUserColorEditorScale,
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
