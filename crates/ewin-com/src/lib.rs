#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::{
        _cfg::key::{
            cmd::{Cmd, CmdType},
            keys::*,
            keywhen::*,
        },
        model::*,
        util::*,
    };
    use ewin_const::model::Env;
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::{collections::HashMap, env, sync::mpsc::Sender};
    use tokio::sync::Mutex;

    pub static ENV: Lazy<Env> = Lazy::new(get_env_platform);
    pub static CMD_MAP: OnceCell<HashMap<(Keys, KeyWhen), Cmd>> = OnceCell::new();
    pub static CMD_TYPE_MAP: OnceCell<HashMap<CmdType, Keys>> = OnceCell::new();
    pub static APP_VERSION: OnceCell<String> = OnceCell::new();

    pub static TX_JOB: OnceCell<Mutex<Sender<Job>>> = OnceCell::new();

    // Cancel is defined independently. Because it needs to be obtained when GREP_INFO_VEC is locked
    pub static GREP_CANCEL_VEC: OnceCell<Mutex<Vec<GrepCancelType>>> = OnceCell::new();
    pub static WATCH_INFO: OnceCell<Mutex<WatchInfo>> = OnceCell::new();
    pub static CURT_DIR: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().to_string_lossy().to_string());

    pub static TERM_SIZE: OnceCell<Mutex<TermSize>> = OnceCell::new();

    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(is_wsl_powershell_enable);
    // Clipboard on memory
    pub static CLIPBOARD: OnceCell<String> = OnceCell::new();
}

pub mod char_style;
pub mod clipboard;
pub mod files {
    pub mod bom;
    pub mod encode;
    pub mod file;
}
pub mod history;
pub mod model;
pub mod scrollbar_v;
pub mod sel_range;
pub mod tab_state;
pub mod util;
pub mod watcher;
pub mod _cfg {
    pub mod key {
        pub mod cmd;
        pub mod keybind;
        pub mod keys;
        pub mod keywhen;
    }
}
