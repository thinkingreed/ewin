pub mod global {
    use crate::{
        _cfg::{cfg::*, lang_cfg::*},
        model::*,
        util::*,
    };
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::{collections::BTreeSet, sync::Mutex};

    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env_platform());
    pub static CFG: OnceCell<Mutex<Cfg>> = OnceCell::new();
    pub static LOG: OnceCell<crate::log::Log> = OnceCell::new();

    pub static GREP_INFO_VEC: OnceCell<tokio::sync::Mutex<Vec<GrepInfo>>> = OnceCell::new();
    // Cancel is defined independently. Because it needs to be obtained when GREP_INFO_VEC is locked
    pub static GREP_CANCEL_VEC: OnceCell<tokio::sync::Mutex<Vec<bool>>> = OnceCell::new();

    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(|| is_powershell_enable());
    pub static REPLACE_SEARCH_RANGE: OnceCell<Mutex<BTreeSet<(usize, usize)>>> = OnceCell::new();
    pub static REPLACE_REPLACE_RANGE: OnceCell<Mutex<BTreeSet<(usize, usize)>>> = OnceCell::new();
}
pub mod colors;
pub mod def;
pub mod evt_act {
    pub mod evt_act;
    pub mod headerbar;
    pub mod prom;
    pub mod statusbar;
}
pub mod bar {
    pub mod headerbar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod editor {
    pub mod buf {
        pub mod edit;
        pub mod io;
    }
    pub mod view {
        pub mod buf_cache;
        pub mod char_style;
        pub mod draw;
    }
    pub mod clipboard;
    pub mod editor;
    pub mod evt_proc;
    pub mod history;
    pub mod key;
    pub mod key_ctrl;
    pub mod key_shift;
    pub mod mouse;
}

pub mod model;
pub mod prompt {
    pub mod promptcont {
        pub mod key;
        pub mod key_ctrl;
        pub mod key_shift;
        pub mod promptcont;
    }
    pub mod close;
    pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod menu;
    pub mod move_row;
    pub mod open_file;
    pub mod prompt;
    pub mod replace;
    pub mod save_new_file;
    pub mod search;
}
pub mod help;
pub mod log;
pub mod tab;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod cfg;
    pub mod lang;
    pub mod lang_cfg;
    pub mod theme_loader;
}
