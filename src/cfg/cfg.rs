use crate::{colors::*, global::*, model::*};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::{ffi::OsStr, path::Path};
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::{self, parsing::SyntaxReference};
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    pub colors: Colors,
    #[serde(skip_deserializing, skip_serializing)]
    pub syntax: Syntax,
}
impl Cfg {
    pub fn default() -> Self {
        Cfg { colors: Colors::default(), syntax: Syntax::default() }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    pub editor: CfgEditor,
    pub status_bar: CfgStatusBar,
}
impl Colors {
    pub fn default() -> Self {
        Colors {
            editor: CfgEditor::default(),
            status_bar: CfgStatusBar::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CfgEditor {
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

impl CfgEditor {
    pub fn default() -> Self {
        CfgEditor {
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
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}
impl ControlChar {
    pub fn default() -> Self {
        ControlChar {
            background: "#000000".to_string(),
            foreground: "#6e6e6e".to_string(),
            bg: Color::default(),
            fg: Color::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "StatusBar")]
pub struct CfgStatusBar {
    background: String,
    foreground: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub bg: Color,
    #[serde(skip_deserializing, skip_serializing)]
    pub fg: Color,
}

impl CfgStatusBar {
    pub fn default() -> Self {
        CfgStatusBar {
            background: "#000000".to_string(),
            foreground: "#87411f".to_string(),
            bg: Color::default(),
            fg: Color::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Syntax {
    pub syntax_set: SyntaxSet,
    pub syntax_reference: SyntaxReference,
    pub theme: Theme,
    pub theme_set: ThemeSet,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Syntax {
    fn default() -> Self {
        Syntax {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            syntax_reference: SyntaxSet::load_defaults_newlines().find_syntax_by_extension(&Path::new("").extension().unwrap_or(OsStr::new("txt")).to_string_lossy().to_string()).unwrap().clone(),
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
        cfg.colors.editor.control_char.bg = Colors::hex2rgb(&cfg.colors.editor.control_char.background);
        cfg.colors.editor.control_char.fg = Colors::hex2rgb(&cfg.colors.editor.control_char.foreground);

        cfg.colors.status_bar.bg = Colors::hex2rgb(&cfg.colors.status_bar.background);
        cfg.colors.status_bar.fg = Colors::hex2rgb(&cfg.colors.status_bar.foreground);

        cfg.syntax.syntax_set = SyntaxSet::load_defaults_newlines();
        cfg.syntax.theme = cfg.syntax.theme_set.themes["base16-eighties.dark"].clone();

        if let Some(sr) = cfg.syntax.syntax_set.find_syntax_by_extension(&ext) {
            cfg.syntax.syntax_reference = sr.clone();
            if let Some(c) = cfg.syntax.theme.settings.background {
                cfg.colors.editor.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                cfg.colors.editor.line_number.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                cfg.colors.editor.control_char.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                cfg.colors.status_bar.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
            }
        } else {
            cfg.syntax.syntax_reference = cfg.syntax.syntax_set.find_syntax_by_extension("txt").unwrap().clone();
        }

        let mut file = File::create("yml.yml").unwrap();
        let s = serde_yaml::to_string(&cfg).unwrap();
        /*
                let file = File::open("yml.yml").unwrap();
                let reader = BufReader::new(file);
                let deserialized: Cfg = serde_yaml::from_reader(reader).unwrap();
                let mut file = File::create("yml_2.yml").unwrap();
                let s = serde_yaml::to_string(&deserialized).unwrap();
        */
        write!(file, "{}", s).unwrap();
        file.flush().unwrap();

        let _ = CFG.set(cfg);
    }
}
