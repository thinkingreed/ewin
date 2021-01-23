pub mod global {
    use crate::{_cfg::lang::lang_cfg::*, model::*, util::*};
    use once_cell::sync::Lazy;
    use std::{collections::HashMap, sync::Mutex};

    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env());
    pub static CFG: Lazy<Mutex<HashMap<&str, &str>>> = Lazy::new(|| {
        let mut m = HashMap::new();
        m.insert("THEME", "base16-ocean.dark");
        Mutex::new(m)
    });
}
pub mod colors;
pub mod def;
pub mod evt_act_prom;
pub mod history;
pub mod editor {
    pub mod draw {
        pub mod buf_cache;
        pub mod char_style;
        pub mod draw;
        pub mod text_buf;
    }
    pub mod clipboard;
    pub mod editor;
    pub mod evt_proc;
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
    pub mod prompt;
    pub mod replace;
    pub mod save_new_file;
    pub mod search;
}
pub mod evt_act;
pub mod log;
pub mod msgbar;
pub mod statusbar;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod lang {
        pub mod lang;
        pub mod lang_cfg;
    }
}
