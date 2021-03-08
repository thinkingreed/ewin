pub mod global {
    use crate::{
        _cfg::{cfg::*, lang_cfg::*},
        model::*,
        util::*,
    };
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::sync::Mutex;

    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env());
    pub static CFG: OnceCell<Mutex<Cfg>> = OnceCell::new();
    pub static FILE: OnceCell<Mutex<File>> = OnceCell::new();
    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(|| is_powershell_enable());
}
pub mod colors;
pub mod def;
pub mod evt_act_headerbar;
pub mod evt_act_prom;
pub mod bar {
    pub mod headerbar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod editor {
    pub mod view {
        pub mod buf_cache;
        pub mod char_style;
        pub mod draw;
        pub mod text_buf;
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
    pub mod grep;
    pub mod grep_result;
    pub mod move_row;
    pub mod prompt;
    pub mod replace;
    pub mod save_new_file;
    pub mod search;
}
pub mod evt_act;
pub mod help;
pub mod log;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod cfg;
    pub mod lang;
    pub mod lang_cfg;
    pub mod theme_loader;
}
