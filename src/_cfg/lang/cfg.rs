use crate::_cfg::lang::lang::LANG_CONFIG;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct LangCfg {
    pub row: String,
    pub col: String,
    pub yes: String,
    pub no: String,
    pub save_confirmation_to_close: String,
}
#[derive(Debug, Deserialize)]
pub struct LangMulti {
    en: LangCfg,
    ja: LangCfg,
}

impl LangCfg {
    pub fn default() -> Self {
        LangCfg {
            row: String::new(),
            col: String::new(),
            yes: String::new(),
            no: String::new(),
            save_confirmation_to_close: String::new(),
        }
    }
    pub fn read_lang_cfg() -> LangCfg {
        let lang_multi: LangMulti = toml::from_str(&LANG_CONFIG.to_string()).unwrap();

        let lang = env::var("LANG").unwrap_or("en_US".to_string());
        if lang.starts_with("ja_JP") {
            return lang_multi.ja;
        } else {
            return lang_multi.en;
        }
    }
}
