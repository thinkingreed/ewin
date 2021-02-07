use crate::{cfg::*, colors::*, global::*, util::*};
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::{self, parsing::SyntaxReference};
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
    pub theme_path: String,
    pub theme_background_enable: bool,
}

impl CfgColorTheme {
    pub fn default() -> Self {
        CfgColorTheme {
            theme_path: "/home/hi/rust/ewin/target/debug/Dracula.tmTheme".to_string(),
            theme_background_enable: true,
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
            highlight_foreground: "#00ff00".to_string(),
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
    pub fn init(ext: &String) {
        let mut cfg = Cfg::default();

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

        cfg.syntax.syntax_set = SyntaxSet::load_defaults_newlines();
        // cfg.syntax.theme = cfg.syntax.theme_set.themes[&cfg.colors.theme.theme_path].clone();

        cfg.syntax.theme = ThemeLoader::new(&cfg.colors.theme.theme_path, &cfg.syntax.theme_set.themes)
            .load()
            .with_context(|| format!("Failed to read instrs from {:?}", &cfg.syntax.theme_set.themes))
            .unwrap();

        if let Some(sr) = cfg.syntax.syntax_set.find_syntax_by_extension(&ext) {
            cfg.syntax.syntax_reference = Some(sr.clone());
            if is_enable_syntax_highlight(ext) {
                if let Some(c) = cfg.syntax.theme.settings.background {
                    if cfg.colors.theme.theme_background_enable {
                        cfg.colors.editor.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                        cfg.colors.editor.line_number.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                    }
                }
            }
        } else {
            cfg.syntax.syntax_reference = None;
        }

        if cfg!(debug_assertions) {
            // TODO Read configuration file
            let mut file = File::create("setting.toml").unwrap();
            let s = toml::to_string(&cfg).unwrap();
            /* read
                    let file = File::open("yml.yml").unwrap();
                    let reader = BufReader::new(file);
                    let deserialized: Cfg = serde_yaml::from_reader(reader).unwrap();
                    let mut file = File::create("yml_2.yml").unwrap();
                    let s = serde_yaml::to_string(&deserialized).unwrap();
            */
            write!(file, "{}", s).unwrap();
            file.flush().unwrap();
        }
        let _ = CFG.set(cfg);
    }
}
