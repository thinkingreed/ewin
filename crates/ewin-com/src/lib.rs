pub mod global {
    use crate::{
        _cfg::{
            cfg::*,
            key::{keycmd::*, keys::*, keywhen::*},
            lang::lang_cfg::*,
        },
        model::*,
        util::*,
    };
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::{collections::HashMap, env, sync::Mutex};

    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static LANG_MAP: Lazy<HashMap<String, String>> = Lazy::new(|| LangCfg::get_lang_map());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env_platform());
    pub static CFG: OnceCell<Mutex<Cfg>> = OnceCell::new();
    pub static LOG: OnceCell<crate::log::Log> = OnceCell::new();
    pub static KEY_CMD_MAP: OnceCell<HashMap<(Keys, KeyWhen), KeyCmd>> = OnceCell::new();
    pub static CMD_KEY_MAP: OnceCell<HashMap<KeyCmd, Keys>> = OnceCell::new();
    pub static APP_VERSION: OnceCell<String> = OnceCell::new();

    pub static GREP_INFO_VEC: OnceCell<tokio::sync::Mutex<Vec<GrepState>>> = OnceCell::new();
    // Cancel is defined independently. Because it needs to be obtained when GREP_INFO_VEC is locked
    pub static GREP_CANCEL_VEC: OnceCell<tokio::sync::Mutex<Vec<bool>>> = OnceCell::new();

    pub static CURT_DIR: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().to_string_lossy().to_string());

    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(|| is_wsl_powershell_enable());
    // Clipboard on memory
    pub static CLIPBOARD: OnceCell<String> = OnceCell::new();
}
pub mod char_style;
pub mod clipboard;
pub mod colors;
pub mod def;
pub mod file;
pub mod history;
pub mod log;
pub mod model;
pub mod sel_range;
pub mod util;
pub mod _cfg {
    pub mod cfg;
    pub mod key {
        pub mod keybind;
        pub mod keycmd;
        pub mod keys;
        pub mod keywhen;
    }
    pub mod lang {
        pub mod lang_cfg;
    }
    pub mod theme_loader;
}
