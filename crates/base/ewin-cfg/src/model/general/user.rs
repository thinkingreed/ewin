use crate::{
    colors::*,
    model::{color::user::*, system::user::*},
};
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
    #[serde(default)]
    pub color_scheme: CfgUserColorScheme,
    pub log: CfgUserLog,
    pub editor: CfgUserEditor,
    pub font: CfgUserFont,
    #[serde(default)]
    pub prompt: CfgUserPrompt,
    #[serde(default)]
    pub context_menu: CfgUserCtxMenu,
    #[serde(default)]
    pub menubar: CfgUserMenubar,
    #[serde(default)]
    pub mouse: CfgUserGeneralMouse,
    #[serde(default)]
    pub word: CfgUserEditorWord,
    #[serde(default)]
    pub view: CfgUserGeneralView,
    #[serde(default)]
    pub sidebar: CfgUserSideBar,
    #[serde(default)]
    pub activitybar: CfgUserActivityBar,
    #[serde(default)]
    pub tooltip: CfgUserToolTip,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserColorScheme {
    pub default_color_theme: Option<String>,
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
    #[serde(default)]
    pub scrollbar: CfgUserScrl,
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

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct CfgUserEditorInputComple {
    pub word_delimiter: Option<String>,
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct CfgUserEditorRowNo {
    pub is_enable: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct CfgUserEditorScale {
    pub is_enable: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CfgUserScrl {
    #[serde(default)]
    pub vertical: CfgUserScrlVertical,
    #[serde(default)]
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

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
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
pub struct CfgUserSideBar {
    pub width: Option<usize>,
    #[serde(default)]
    pub explorer: CfgUserSideBarExplorer,
    #[serde(default)]
    pub scrollbar: CfgUserScrl,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserActivityBar {
    pub width: Option<usize>,
    pub content: Option<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserToolTip {
    pub hover_delay: Option<usize>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserSideBarExplorer {
    pub tree: CfgUserSideBarExplorerTree,
    pub quick_access: CfgUserSideBarExplorerQuickAccess,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserSideBarExplorerTree {
    pub indent: Option<usize>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CfgUserSideBarExplorerQuickAccess {
    pub width: Option<usize>,
    pub content: Option<String>,
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
