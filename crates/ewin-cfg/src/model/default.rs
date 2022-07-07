use crate::{colors::*, global::*};
use serde::{Deserialize, Serialize};
use std::fmt;
use syntect::{
    self,
    highlighting::{Highlighter, Theme, ThemeSet},
    parsing::SyntaxSet,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cfg {
    pub general: CfgGeneral,
    pub system: CfgSystem,
    #[serde(skip_deserializing, skip_serializing)]
    pub colors: CfgColors,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgEdit {}
#[derive(Debug, Default)]
pub struct CfgSyntax<'a> {
    pub syntax: Syntax,
    pub highlighter_opt: Option<Highlighter<'a>>,
}

impl CfgSyntax<'_> {
    pub fn get() -> &'static CfgSyntax<'static> {
        return CFG_SYNTAX.get().unwrap();
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgGeneral {
    pub lang: String,
    pub log: CfgLog,
    pub editor: CfgEditor,
    pub font: CfgFont,
    pub prompt: CfgPrompt,
    pub context_menu: CfgCtxMenu,
    pub menubar: CfgMenubar,
    pub mouse: CfgGeneralMouse,
    pub colors: CfgGeneralColors,
    pub view: CfgGeneralView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgLog {
    pub level: String,
}

impl Default for CfgLog {
    fn default() -> Self {
        CfgLog { level: "info".to_string() }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgEditor {
    pub search: CfgSearch,
    pub tab: CfgTab,
    pub format: CfgFormat,
    pub scrollbar: CfgScrl,
    pub cursor: CfgCur,
    pub column_char_width_gap_space: CfgUserEditorColumnCharWidthGap,
    pub save: CfgEditorSave,
    pub word: CfgEditorWord,
    pub input_comple: CfgEditorInputComple,
    pub row_no: CfgUserEditorRowNo,
    pub scale: CfgUserEditorScale,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgFont {
    pub ambiguous_width: Option<usize>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgPrompt {
    pub open_file: CfgPromptOpenFile,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgCtxMenu {
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgMenubar {
    pub content: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgSearch {
    pub case_sensitive: bool,
    pub regex: bool,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgTab {
    pub size: usize,
    pub input_type: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab_type: TabType,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgFormat {
    pub indent_type: String,
    pub indent_size: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab_type: TabType,
    #[serde(skip_deserializing, skip_serializing)]
    pub indent: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgScrl {
    pub vertical: CfgScrlVertical,
    pub horizontal: CfgScrlHorizontal,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgScrlVertical {
    pub width: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgScrlHorizontal {
    pub height: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgCur {
    pub move_position_by_scrolling_enable: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorColumnCharWidthGap {
    pub character: char,
    pub end_of_line_enable: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgEditorSave {
    pub use_string_first_line_for_file_name_of_new_file: bool,
    pub candidate_extension_when_saving_new_file: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgEditorWord {
    pub word_delimiter: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgEditorInputComple {
    pub word_delimiter: String,
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorScale {
    pub is_enable: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserEditorRowNo {
    pub is_enable: bool,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq)]
pub struct CfgPromptOpenFile {
    pub directory_init_value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TabType {
    Tab,
    HalfWidthBlank,
}

impl Default for TabType {
    fn default() -> Self {
        TabType::Tab
    }
}
impl TabType {
    pub(crate) fn from_str(s: &str) -> TabType {
        match s {
            //
            "tab" => TabType::Tab,
            "half_width_blank" => TabType::HalfWidthBlank,
            _ => TabType::Tab,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgGeneralColors {
    pub theme: CfgColorTheme,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgGeneralMouse {
    pub mouse_enable: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgGeneralView {
    pub tab_characters_as_symbols: String,
    pub full_width_space_characters_as_symbols: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColors {
    pub system: CfgColorSystem,
    pub menubar: CfgColorMenuBar,
    pub filebar: CfgColorFileBar,
    pub editor: CfgColorEditor,
    pub statusbar: CfgColorStatusBar,
    pub ctx_menu: CfgColorCtxMenu,
    pub msg: CfgColorMsg,
    pub file: CfgColorFile,
}
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorSystem {
    pub btn: CfgColorSystemBtn,
    pub state: CfgColorSystemState,
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
    pub default_color_theme: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeSystemColorType {
    Black,
    White,
}
impl ThemeSystemColorType {
    pub fn from_str_color_type(s: &str) -> ThemeSystemColorType {
        match s {
            "black" => return ThemeSystemColorType::Black,
            "white" => return ThemeSystemColorType::White,
            _ => return ThemeSystemColorType::default(),
        }
    }
}
impl Default for ThemeSystemColorType {
    fn default() -> Self {
        Self::White
    }
}

impl fmt::Display for ThemeSystemColorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThemeSystemColorType::White => write!(f, "white"),
            ThemeSystemColorType::Black => write!(f, "black"),
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
    pub scrollbar: CfgColorEditorScrollbar,
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
pub struct CfgColorEditorScrollbar {
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
#[serde(rename = "Msg")]
pub struct CfgColorMsg {
    pub normal_foreground: String,
    pub highlight_foreground: String,
    pub warning_foreground: String,
    pub err_foreground: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub normal_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub highlight_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub warning_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub err_fg: Color,
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
pub struct CfgSystem {
    pub os: CfgOS,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgOS {
    pub windows: CfgWindows,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgWindows {
    pub change_output_encoding_utf8: bool,
}
