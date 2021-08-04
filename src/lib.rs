pub mod global {
    use crate::{
        _cfg::{
            cfg::*,
            keys::KeyWhen,
            keys::{KeyCmd, Keys},
            lang::lang_cfg::LangCfg,
        },
        model::*,
        tab::Tab,
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

    pub static GREP_INFO_VEC: OnceCell<tokio::sync::Mutex<Vec<GrepState>>> = OnceCell::new();
    // Cancel is defined independently. Because it needs to be obtained when GREP_INFO_VEC is locked
    pub static GREP_CANCEL_VEC: OnceCell<tokio::sync::Mutex<Vec<bool>>> = OnceCell::new();

    pub static CURT_DIR: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().to_string_lossy().to_string());

    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(|| is_powershell_enable());
    // Clipboard on memory
    pub static CLIPBOARD: OnceCell<String> = OnceCell::new();
    pub static TAB: OnceCell<tokio::sync::Mutex<Tab>> = OnceCell::new();
}
pub mod colors;
pub mod def;
pub mod sel_range;
pub mod evt_act {
    pub mod _evt_act;
    pub mod ctx_menu;
    pub mod headerbar;
    pub mod prom;
    pub mod statusbar;
}
pub mod bar {
    pub mod headerbar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod ctx_menu {
    pub mod ctx_menu;
}
pub mod editor {
    pub mod buf {
        pub mod edit;
        pub mod io;
    }
    pub mod macros {
        pub mod js_func;
        pub mod js_macro;
        pub mod key_macro;
    }

    pub mod view {
        pub mod buf_cache;
        pub mod char_style;
        pub mod draw;
    }
    pub mod convert;
    pub mod draw_range;
    pub mod edit_proc;
    pub mod editor;
    pub mod format;
    pub mod history;
    pub mod key;
    pub mod key_ctrl;
    pub mod key_shift;

    pub mod mouse;
}

pub mod model;
pub mod prompt {
    pub mod cont {
        pub mod edit_proc;
        pub mod key;
        pub mod key_ctrl;
        pub mod key_shift;
        pub mod promptcont;
    }
    pub mod prompt {
        pub mod prompt;
    }
    pub mod choice;
    pub mod close;
    pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod menu;
    pub mod move_row;
    pub mod open_file;
    pub mod replace;
    pub mod save_new_file;
    pub mod search;
}
pub mod clipboard;
pub mod help;
pub mod log;
pub mod tab;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod cfg;
    pub mod keybind;
    pub mod keys;
    pub mod lang {
        pub mod lang_cfg;
    }
    pub mod theme_loader;
}
