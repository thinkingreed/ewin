use crate::{_cfg::*, colors::*, def::*, global::*, log::*};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{env, fs, fs::File, io::Write, sync::Mutex};
use syntect::{
    self,
    highlighting::{Theme, ThemeSet},
    parsing::SyntaxSet,
};
use theme_loader::ThemeLoader;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub general: CfgGeneral,
    pub colors: CfgColors,
    #[serde(skip_deserializing, skip_serializing)]
    pub syntax: Syntax,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgGeneral {
    pub log: Option<CfgLog>,
    pub editor: CfgEditor,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgLog {
    pub level: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgEditor {
    pub search: CfgSearch,
    pub tab: CfgTab,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgSearch {
    pub case_sens: bool,
    pub regex: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgTab {
    pub width: usize,
    pub tab_input_type: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub tab_type: TabType,
}

#[derive(Debug)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgColors {
    pub theme: CfgColorTheme,
    pub header_bar: CfgColorHeaderBar,
    pub editor: CfgColorEditor,
    pub status_bar: CfgColorStatusBar,
    pub msg: CfgColorMsg,
    pub file: CfgColorFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgColorTheme {
    pub theme_path: Option<String>,
    pub theme_background_enable: Option<bool>,
    #[serde(skip_deserializing, skip_serializing)]
    pub theme_bg_enable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub search: Search,
    pub control_char: ControlChar,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineNumber {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Selection {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ControlChar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "HeaderBar")]
pub struct CfgColorHeaderBar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgColorStatusBar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
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
    pub fn init() -> String {
        let mut cfg: Cfg = toml::from_str(include_str!("../../setting.toml")).unwrap();
        let mut err_str = "".to_string();
        let mut read_str = String::new();

        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.config_dir();
            let config_file = &config_dir.join(env!("CARGO_PKG_NAME")).join(SETTING_FILE);

            if config_file.exists() {
                match fs::read_to_string(config_file) {
                    Ok(str) => {
                        read_str = str;
                    }
                    Err(e) => {
                        err_str = format!("{} {} {}", LANG.file_loading_failed, config_file.to_string_lossy().to_string(), e);
                    }
                }
                if err_str.is_empty() {
                    match toml::from_str(&read_str) {
                        Ok(c) => cfg = c,
                        Err(e) => {
                            err_str = format!("{} {} {}", LANG.file_parsing_failed, config_file.to_string_lossy().to_string(), e);
                        }
                    };
                }
            }
        }

        cfg.general.editor.tab.tab_type = TabType::from_str(&cfg.general.editor.tab.tab_input_type);

        cfg.colors.header_bar.fg = Colors::hex2rgb(&cfg.colors.header_bar.foreground);

        cfg.colors.editor.fg = Colors::hex2rgb(&cfg.colors.editor.foreground);
        cfg.colors.editor.bg = Colors::hex2rgb(&cfg.colors.editor.background);
        cfg.colors.editor.line_number.bg = Colors::hex2rgb(&cfg.colors.editor.line_number.background);
        cfg.colors.editor.line_number.fg = Colors::hex2rgb(&cfg.colors.editor.line_number.foreground);
        cfg.colors.editor.selection.bg = Colors::hex2rgb(&cfg.colors.editor.selection.background);
        cfg.colors.editor.selection.fg = Colors::hex2rgb(&cfg.colors.editor.selection.foreground);
        cfg.colors.editor.search.bg = Colors::hex2rgb(&cfg.colors.editor.search.background);
        cfg.colors.editor.search.fg = Colors::hex2rgb(&cfg.colors.editor.search.foreground);
        cfg.colors.editor.control_char.fg = Colors::hex2rgb(&cfg.colors.editor.control_char.foreground);

        cfg.colors.msg.normal_fg = Colors::hex2rgb(&cfg.colors.msg.normal_foreground);
        cfg.colors.msg.highlight_fg = Colors::hex2rgb(&cfg.colors.msg.highlight_foreground);
        cfg.colors.msg.warning_fg = Colors::hex2rgb(&cfg.colors.msg.warning_foreground);
        cfg.colors.msg.err_fg = Colors::hex2rgb(&cfg.colors.msg.err_foreground);
        cfg.colors.status_bar.fg = Colors::hex2rgb(&cfg.colors.status_bar.foreground);

        cfg.colors.file.normal_fg = Colors::hex2rgb(&cfg.colors.file.normal_foreground);
        cfg.colors.file.directory_fg = Colors::hex2rgb(&cfg.colors.file.directory_foreground);
        cfg.colors.file.executable_fg = Colors::hex2rgb(&cfg.colors.file.executable_foreground);

        match ThemeLoader::new(&cfg.colors.theme.theme_path, &cfg.syntax.theme_set.themes).load() {
            Ok((theme, err_string)) => {
                if !err_string.is_empty() {
                    err_str = err_string;
                }
                cfg.syntax.theme = theme;
                if let Some(c) = cfg.syntax.theme.settings.background {
                    if let Some(theme_bg_enable) = cfg.colors.theme.theme_background_enable {
                        cfg.colors.editor.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                        cfg.colors.editor.line_number.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                        cfg.colors.theme.theme_bg_enable = theme_bg_enable;
                    } else {
                        cfg.colors.theme.theme_bg_enable = false;
                    }
                }
            }
            // Even if the set theme fails to read, the internal theme is read, so the theme is surely read.
            Err(_) => {}
        }

        if cfg!(debug_assertions) {
            let mut file = File::create(SETTING_FILE).unwrap();
            let s = toml::to_string(&cfg).unwrap();
            write!(file, "{}", s).unwrap();
            file.flush().unwrap();
        }

        Log::set_logger(&cfg.general.log);
        if !read_str.is_empty() {
            Log::info("read setting.toml", &read_str);
        }
        let _ = CFG.set(Mutex::new(cfg));

        return err_str;
    }
}
