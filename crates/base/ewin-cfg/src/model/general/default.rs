use crate::{
    colors::*,
    global::*,
    model::{color::default::*, system::default::CfgSystem},
};
use serde::{Deserialize, Serialize};
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
    pub color_scheme: CfgColorScheme,
    pub log: CfgLog,
    pub editor: CfgEditor,
    pub font: CfgFont,
    pub prompt: CfgPrompt,
    pub context_menu: CfgCtxMenu,
    pub menubar: CfgMenubar,
    pub mouse: CfgGeneralMouse,
    pub view: CfgGeneralView,
    pub sidebar: CfgSideBar,
    pub activitybar: CfgActivityBar,
    pub tooltip: CfgToolTip,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgColorScheme {
    pub default_color_theme: String,
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
    pub cursor: CfgCur,
    pub column_char_width_gap_space: CfgUserEditorColumnCharWidthGap,
    pub save: CfgEditorSave,
    pub word: CfgEditorWord,
    pub input_comple: CfgEditorInputComple,
    pub row_no: CfgEditorRowNo,
    pub scale: CfgEditorScale,
    pub scrollbar: CfgScrl,
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

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CfgEditorScale {
    pub is_enable: bool,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CfgEditorRowNo {
    pub is_enable: bool,
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

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
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
pub struct CfgGeneralMouse {
    pub mouse_enable: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgGeneralView {
    pub tab_characters_as_symbols: String,
    pub full_width_space_characters_as_symbols: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgSideBar {
    pub width: usize,
    pub explorer: CfgSideBarExplorer,
    pub scrollbar: CfgScrl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgActivityBar {
    pub width: usize,
    pub content: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgToolTip {
    pub hover_delay: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgSideBarExplorer {
    pub tree: CfgSideBarExplorerTree,
    pub quick_access: CfgSideBarExplorerQuickAccess,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgSideBarExplorerTree {
    pub indent: usize,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgSideBarExplorerQuickAccess {
    pub width: usize,
    pub content: String,
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
