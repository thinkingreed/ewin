pub mod global {
    use crate::{_cfg::lang::cfg::LangCfg, model::*, util::*};
    use once_cell::sync::Lazy;
    pub static LANG: Lazy<LangCfg> = Lazy::new(|| LangCfg::read_lang_cfg());
    pub static ENV: Lazy<Env> = Lazy::new(|| get_env());
    pub static ENV_STR: Lazy<String> = Lazy::new(|| get_env_str());
}
pub mod colors;
pub mod def;
pub mod evt_act_prom;
pub mod history;
pub mod editor {
    pub mod rope {
        pub mod buf_cache;
        pub mod rope_util;
        pub mod text_buf;
    }
    pub mod clipboard;
    pub mod color;
    pub mod draw;
    pub mod editor;
    pub mod evtproc;
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
        pub mod cfg;
        pub mod lang;
    }
}
