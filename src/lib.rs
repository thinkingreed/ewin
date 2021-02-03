pub mod global {
    use crate::{
        cfg::{cfg::*, lang_cfg::*},
        model::*,
        util::*,
    };
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;

    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env());
    // pub static CFG: Lazy<Cfg> = Lazy::new(|| Cfg::read_cfg());
    pub static CFG: OnceCell<Cfg> = OnceCell::new();
}
pub mod colors;
pub mod def;
pub mod evt_act_prom;
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
pub mod cfg {
    pub mod cfg;
    pub mod lang;
    pub mod lang_cfg;
}
