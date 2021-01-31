use crate::cfg::lang::lang::LANG_CONFIG;
use serde::Deserialize;
use std::env;
#[derive(Debug, Serialize, Deserialize)]
pub struct Cfg {
    colors: Colors,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Colors {
    editor: Editor,
}
#[derive(Debug, Serialize, Deserialize)]
struct Editor {
    background: String,
    foreground: String,
    LineNumber: LineNumber,
}

#[derive(Debug, Serialize, Deserialize)]
struct LineNumber {
    background: String,
    foreground: String,
}

impl Cfg {
    pub fn default() -> Self {
        Cfg { colors: Colors::default() }
    }
}

impl Colors {
    pub fn default() -> Self {
        Colors { editor: Editor::default() }
    }
}

impl Editor {
    pub fn default() -> Self {
        Editor {
            background: "#000000".to_string(),
            foreground: "#ffffff".to_string(),
            line_number: LineNumber::default(),
        }
    }
}
impl LineNumber {
    pub fn default() -> Self {
        LineNumber {
            background: "#000000".to_string(),
            foreground: "##6e6e6e".to_string(),
        }
    }
}

impl Cfg {
    pub fn read_cfg() -> Cfg {
        // TODO Read from configuration file
        let cfg = Cfg::default();
        return cfg;
    }
}
