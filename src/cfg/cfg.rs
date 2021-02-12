use crate::{cfg::*, colors::*, def::*, global::*, model::*, util::*};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::{fs, fs::File, io::Write};
use syntect::{
    self,
    highlighting::{Theme, ThemeSet},
    parsing::SyntaxReference,
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

impl Cfg {
    pub fn default() -> Self {
        Cfg {
            general: CfgGeneral::default(),
            colors: CfgColors::default(),
            syntax: Syntax::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgGeneral {}
impl CfgGeneral {
    pub fn default() -> Self {
        CfgGeneral {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CfgColors {
    pub theme: CfgColorTheme,
    pub editor: CfgColorEditor,
    pub status_bar: CfgColorStatusBar,
    pub msg: CfgColorMsg,
}
impl CfgColors {
    pub fn default() -> Self {
        CfgColors {
            theme: CfgColorTheme::default(),
            editor: CfgColorEditor::default(),
            msg: CfgColorMsg::default(),
            status_bar: CfgColorStatusBar::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgColorTheme {
    pub theme_path: Option<String>,
    pub theme_background_enable: Option<bool>,
    #[serde(skip_deserializing, skip_serializing)]
    pub theme_bg_enable: bool,
}

impl CfgColorTheme {
    pub fn default() -> Self {
        CfgColorTheme {
            theme_path: None,
            theme_background_enable: None,
            theme_bg_enable: false,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgColorEditor {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
    pub line_number: LineNumber,
    pub selection: Selection,
    pub search: Search,
    pub control_char: ControlChar,
}

impl CfgColorEditor {
    pub fn default() -> Self {
        CfgColorEditor {
            background: "#000000".to_string(),
            foreground: "#ffffff".to_string(),
            bg: Color::default(),
            fg: Color::default(),
            line_number: LineNumber::default(),
            selection: Selection::default(),
            search: Search::default(),
            control_char: ControlChar::default(),
        }
    }
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

impl LineNumber {
    pub fn default() -> Self {
        LineNumber {
            background: "#000000".to_string(),
            foreground: "#6e6e6e".to_string(),
            bg: Color::default(),
            fg: Color::default(),
        }
    }
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
impl Selection {
    pub fn default() -> Self {
        Selection {
            background: "#dd4814".to_string(),
            foreground: "#000000".to_string(),
            bg: Color::default(),
            fg: Color::default(),
        }
    }
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
impl Search {
    pub fn default() -> Self {
        Search {
            background: "#dd4814".to_string(),
            foreground: "#000000".to_string(),
            bg: Color::default(),
            fg: Color::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ControlChar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
impl ControlChar {
    pub fn default() -> Self {
        ControlChar {
            foreground: "#6e6e6e".to_string(),
            fg: Color::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgColorStatusBar {
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

impl CfgColorStatusBar {
    pub fn default() -> Self {
        CfgColorStatusBar {
            foreground: "#87411f".to_string(),
            fg: Color::default(),
        }
    }
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

impl CfgColorMsg {
    pub fn default() -> Self {
        CfgColorMsg {
            normal_foreground: "#ffffff".to_string(),
            highlight_foreground: "#006400".to_string(),
            warning_foreground: "#ffa500".to_string(),
            err_foreground: "#ff0000".to_string(),
            normal_fg: Color::default(),
            highlight_fg: Color::default(),
            warning_fg: Color::default(),
            err_fg: Color::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Syntax {
    pub syntax_set: SyntaxSet,
    pub syntax_reference: Option<SyntaxReference>,
    pub theme: Theme,
    pub theme_set: ThemeSet,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Syntax {
    fn default() -> Self {
        Syntax {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            syntax_reference: None,
            theme_set: ThemeSet::load_defaults(),
            theme: Theme::default(),
            fg: Color::default(),
            bg: Color::default(),
        }
    }
}
impl Cfg {
    pub fn init(ext: &String) -> String {
        let mut cfg = Cfg::default();

        let mut err_str = "".to_string();
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.config_dir();
            let config_file = &config_dir.join(env!("CARGO_PKG_NAME")).join(SETTING_FILE);

            if config_file.exists() {
                let mut read_str = String::new();
                match fs::read_to_string(config_file) {
                    Ok(str) => read_str = str,
                    Err(e) => {
                        err_str = format!("{} {}", LANG.file_loading_failed, config_file.to_string_lossy().to_string());
                        Log::ep("SETTING_FILE read_to_string", &e);
                    }
                }
                if err_str.is_empty() {
                    match toml::from_str(&read_str) {
                        Ok(c) => cfg = c,
                        Err(e) => {
                            err_str = format!("{} {}", LANG.file_parsing_failed, config_file.to_string_lossy().to_string());
                            Log::ep("SETTING_FILE parsing", &e);
                        }
                    };
                }
            }
        }

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

        if is_enable_syntax_highlight(ext) {
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
        }
        if let Some(sr) = cfg.syntax.syntax_set.find_syntax_by_extension(&ext) {
            cfg.syntax.syntax_reference = Some(sr.clone());
        } else {
            cfg.syntax.syntax_reference = None;
        }

        if cfg!(debug_assertions) {
            let mut file = File::create(SETTING_FILE).unwrap();
            let s = toml::to_string(&cfg).unwrap();
            write!(file, "{}", s).unwrap();
            file.flush().unwrap();
        }
        let _ = CFG.set(cfg);
        return err_str;
    }
}
