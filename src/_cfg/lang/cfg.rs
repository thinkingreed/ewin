use crate::_cfg::lang::lang::LANG_CONFIG;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct LangCfg {
    pub row: String,
    pub col: String,
    pub yes: String,
    pub no: String,
    pub cancel: String,
    pub fixed: String,
    pub new_file: String,
    pub save_confirmation_to_close: String,
    pub terminal_size_small: String,
    pub set_new_filenm: String,
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
            cancel: String::new(),
            fixed: String::new(),
            new_file: String::new(),
            save_confirmation_to_close: String::new(),
            terminal_size_small: String::new(),
            set_new_filenm: String::new(),
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
