#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::os::*;
    use ewin_const::models::env::*;
    use once_cell::sync::Lazy;
    use std::env;

    pub static ENV: Lazy<Env> = Lazy::new(get_env_platform);
    pub static CURT_DIR: Lazy<String> = Lazy::new(|| env::current_dir().unwrap().to_string_lossy().to_string());
    pub static IS_POWERSHELL_ENABLE: Lazy<bool> = Lazy::new(is_wsl_powershell_enable);
}

pub mod char_edit;
pub mod os;
pub mod path;
pub mod str_edit;
pub mod util;
pub mod files {
    pub mod bom;
    pub mod dir;
    pub mod encode;
    pub mod file;
    pub mod nl;
}
