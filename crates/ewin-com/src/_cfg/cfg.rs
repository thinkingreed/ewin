use crate::{_cfg::lang::lang_cfg::*, colors::*, def::*, global::*, log::*, model::*};
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::Write, sync::Mutex};
use syntect::{
    self,
    highlighting::{Theme, ThemeSet},
    parsing::SyntaxSet,
};

use super::theme_loader::ThemeLoader;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cfg {
    pub general: CfgGeneral,
    pub colors: CfgColors,
}
impl Cfg {
    pub fn get() -> &'static Cfg {
        return CFG.get().unwrap();
    }

    pub fn get_edit_search() -> CfgSearch {
        return CfgSearch { regex: Cfg::get_edit_search_regex(), case_sens: Cfg::get_edit_search_case_sens() };
    }

    pub fn get_edit_search_regex() -> bool {
        return CFG_EDIT.get().unwrap().try_lock().unwrap().general.editor.search.regex;
    }
    pub fn get_edit_search_case_sens() -> bool {
        return CFG_EDIT.get().unwrap().try_lock().unwrap().general.editor.search.case_sens;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgSyntax {
    pub syntax: Syntax,
}

impl Default for CfgSyntax {
    fn default() -> Self {
        CfgSyntax { syntax: Syntax::default() }
    }
}
impl CfgSyntax {
    pub fn get() -> &'static CfgSyntax {
        return CFG_SYNTAX.get().unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgGeneral {
    pub lang: Option<String>,
    pub log: Option<CfgLog>,
    pub editor: CfgEditor,
    pub prompt: CfgPrompt,
    pub ctx_menu: CfgCtxMenu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgLog {
    pub level: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgEditor {
    pub search: CfgSearch,
    pub tab: CfgTab,
    pub format: CfgFormat,
    pub scrollbar: CfgScrl,
    pub cursor: CfgCur,
    pub column_char_alignment_space: CfgColumnCharAlignmentSpace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgPrompt {
    pub open_file: CfgPromptOpenFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgCtxMenu {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgSearch {
    pub case_sens: bool,
    pub regex: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgTab {
    pub size: usize,
    pub tab_input_type: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab_type: TabType,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgFormat {
    pub indent_type: String,
    pub indent_size: usize,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab_type: TabType,
    #[serde(skip_deserializing, skip_serializing)]
    pub indent: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgScrl {
    pub vertical: CfgScrlVertical,
    pub horizontal: CfgScrlHorizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgScrlVertical {
    pub width: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgScrlHorizontal {
    pub height: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgCur {
    pub move_position_by_scrolling_enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColumnCharAlignmentSpace {
    pub character: char,
    pub end_of_line_enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CfgPromptOpenFile {
    pub directory_init_value: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub dir_init: OpenFileInitValue,
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
    fn from_str(s: &str) -> TabType {
        match s {
            //
            "tab" => TabType::Tab,
            "half_width_blank" => TabType::HalfWidthBlank,
            _ => TabType::Tab,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColors {
    pub theme: CfgColorTheme,
    pub headerbar: CfgColorHeaderBar,
    pub editor: CfgColorEditor,
    pub statusbar: CfgColorStatusBar,
    pub ctx_menu: CfgColorCtxMenu,
    pub msg: CfgColorMsg,
    pub file: CfgColorFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColorTheme {
    pub theme_path: Option<String>,
    pub theme_background_enable: Option<bool>,
    #[serde(skip_deserializing, skip_serializing)]
    pub theme_bg_enable: bool,
    pub disable_syntax_highlight_ext: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColorEditor {
    background: String,
    foreground: String,
    // #[serde(rename = "background", serialize_with = "str_2_color")]
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    pub line_number: LineNumber,
    pub selection: Selection,
    pub search: CfgColorEditorSearch,
    pub control_char: ControlChar,
    pub column_char_alignment_space: ColorColumnCharAlignmentSpace,
    pub scrollbar: CfgColorScrollbar,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineNumber {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Selection {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColorEditorSearch {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlChar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorColumnCharAlignmentSpace {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfgColorScrollbar {
    horizontal_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_horizontal: Color,
    vertical_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_vertical: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "HeaderBar")]
pub struct CfgColorHeaderBar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    tab_active_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_tab_active: Color,
    tab_active_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_tab_active: Color,
    tab_passive_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_tab_passive: Color,
    tab_passive_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_tab_passive: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgColorStatusBar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "CtxMenu")]
pub struct CfgColorCtxMenu {
    non_select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_non_sel: Color,
    select_background: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg_sel: Color,
    non_select_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_non_sel: Color,
    select_foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg_sel: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Msg")]
pub struct CfgColorMsg {
    normal_foreground: String,
    highlight_foreground: String,
    warning_foreground: String,
    err_foreground: String,

    #[serde(skip_deserializing, skip_serializing)]
    pub normal_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub highlight_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub warning_fg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub err_fg: Color,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "File")]
pub struct CfgColorFile {
    normal_foreground: String,
    directory_foreground: String,
    executable_foreground: String,

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
impl Cfg {
    pub fn init(args: &Args, cfg_str: &str) -> String {
        let mut cfg: Cfg = toml::from_str(cfg_str).unwrap();
        let mut cfg_syntax = CfgSyntax::default();
        let mut err_str = "".to_string();
        let mut read_str = String::new();

        if let Some(config_file) = FilePath::get_app_config_file_path() {
            if config_file.exists() {
                match fs::read_to_string(&config_file) {
                    Ok(str) => read_str = str,
                    Err(e) => err_str = format!("{} {} {}", Lang::get().file_loading_failed, config_file.to_string_lossy().to_string(), e),
                }
                if err_str.is_empty() {
                    match toml::from_str(&read_str) {
                        Ok(c) => cfg = c,
                        Err(e) => err_str = format!("{}{} {} {}", Lang::get().file, Lang::get().parsing_failed, config_file.to_string_lossy().to_string(), e),
                    };
                }
            } else if args.out_config_flg {
                if let Ok(mut file) = File::create(config_file) {
                    let _ = write!(&mut file, "{}", cfg_str);
                    let _ = &mut file.flush().unwrap();
                }
            }
        }

        cfg.general.editor.tab.tab_type = TabType::from_str(&cfg.general.editor.tab.tab_input_type);

        cfg.general.editor.tab.tab = match cfg.general.editor.tab.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(cfg.general.editor.tab.size),
        };

        cfg.general.editor.format.tab_type = TabType::from_str(&cfg.general.editor.format.indent_type);
        cfg.general.editor.format.indent = match cfg.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(cfg.general.editor.format.indent_size),
        };
        cfg.general.editor.format.indent = match cfg.general.editor.format.tab_type {
            TabType::Tab => TAB_CHAR.to_string(),
            TabType::HalfWidthBlank => " ".repeat(cfg.general.editor.format.indent_size),
        };
        cfg.general.prompt.open_file.dir_init = match &cfg.general.prompt.open_file.directory_init_value {
            s if s == &"current_directory".to_string() => OpenFileInitValue::CurtDir,
            _ => OpenFileInitValue::None,
        };

        cfg.colors.headerbar.fg = Colors::hex2rgb(&cfg.colors.headerbar.foreground);
        cfg.colors.headerbar.bg = Colors::hex2rgb(&cfg.colors.headerbar.background);
        cfg.colors.headerbar.fg_tab_active = Colors::hex2rgb(&cfg.colors.headerbar.tab_active_foreground);
        cfg.colors.headerbar.bg_tab_active = Colors::hex2rgb(&cfg.colors.headerbar.tab_active_background);
        cfg.colors.headerbar.fg_tab_passive = Colors::hex2rgb(&cfg.colors.headerbar.tab_passive_foreground);
        cfg.colors.headerbar.bg_tab_passive = Colors::hex2rgb(&cfg.colors.headerbar.tab_passive_background);

        cfg.colors.editor.fg = Colors::hex2rgb(&cfg.colors.editor.foreground);
        cfg.colors.editor.bg = Colors::hex2rgb(&cfg.colors.editor.background);
        cfg.colors.editor.line_number.bg = Colors::hex2rgb(&cfg.colors.editor.line_number.background);
        cfg.colors.editor.line_number.fg = Colors::hex2rgb(&cfg.colors.editor.line_number.foreground);
        cfg.colors.editor.selection.bg = Colors::hex2rgb(&cfg.colors.editor.selection.background);
        cfg.colors.editor.selection.fg = Colors::hex2rgb(&cfg.colors.editor.selection.foreground);
        cfg.colors.editor.search.bg = Colors::hex2rgb(&cfg.colors.editor.search.background);
        cfg.colors.editor.search.fg = Colors::hex2rgb(&cfg.colors.editor.search.foreground);
        cfg.colors.editor.control_char.fg = Colors::hex2rgb(&cfg.colors.editor.control_char.foreground);

        cfg.colors.editor.column_char_alignment_space.fg = Colors::hex2rgb(&cfg.colors.editor.column_char_alignment_space.foreground);
        cfg.colors.editor.column_char_alignment_space.bg = Colors::hex2rgb(&cfg.colors.editor.column_char_alignment_space.background);

        cfg.colors.editor.scrollbar.bg_vertical = Colors::hex2rgb(&cfg.colors.editor.scrollbar.vertical_background);
        cfg.colors.editor.scrollbar.bg_horizontal = Colors::hex2rgb(&cfg.colors.editor.scrollbar.horizontal_background);

        cfg.colors.msg.normal_fg = Colors::hex2rgb(&cfg.colors.msg.normal_foreground);
        cfg.colors.msg.highlight_fg = Colors::hex2rgb(&cfg.colors.msg.highlight_foreground);
        cfg.colors.msg.warning_fg = Colors::hex2rgb(&cfg.colors.msg.warning_foreground);
        cfg.colors.msg.err_fg = Colors::hex2rgb(&cfg.colors.msg.err_foreground);
        cfg.colors.statusbar.fg = Colors::hex2rgb(&cfg.colors.statusbar.foreground);

        cfg.colors.ctx_menu.fg_sel = Colors::hex2rgb(&cfg.colors.ctx_menu.select_foreground);
        cfg.colors.ctx_menu.fg_non_sel = Colors::hex2rgb(&cfg.colors.ctx_menu.non_select_foreground);
        cfg.colors.ctx_menu.bg_sel = Colors::hex2rgb(&cfg.colors.ctx_menu.select_background);
        cfg.colors.ctx_menu.bg_non_sel = Colors::hex2rgb(&cfg.colors.ctx_menu.non_select_background);

        cfg.colors.file.normal_fg = Colors::hex2rgb(&cfg.colors.file.normal_foreground);
        cfg.colors.file.directory_fg = Colors::hex2rgb(&cfg.colors.file.directory_foreground);
        cfg.colors.file.executable_fg = Colors::hex2rgb(&cfg.colors.file.executable_foreground);

        if let Ok((theme, err_string)) = ThemeLoader::new(&cfg.colors.theme.theme_path, &cfg_syntax.syntax.theme_set.themes).load() {
            if !err_string.is_empty() {
                err_str = err_string;
            }
            cfg_syntax.syntax.theme = theme;
            if let Some(c) = cfg_syntax.syntax.theme.settings.background {
                if let Some(theme_bg_enable) = cfg.colors.theme.theme_background_enable {
                    cfg.colors.editor.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                    cfg.colors.editor.line_number.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                    cfg.colors.theme.theme_bg_enable = theme_bg_enable;
                } else {
                    cfg.colors.theme.theme_bg_enable = false;
                }
            }
        }

        Log::set_logger(&cfg.general.log);
        Log::info("cfg.general.log", &cfg.general.log);

        if !read_str.is_empty() {
            Log::info("read setting.toml", &read_str);
        }
        let _ = CFG.set(cfg.clone());
        let _ = CFG_EDIT.set(Mutex::new(cfg));
        let _ = CFG_SYNTAX.set(cfg_syntax);

        err_str
    }
}
