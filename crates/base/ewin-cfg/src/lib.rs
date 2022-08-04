#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::lang::lang_cfg::Lang;
    use crate::model::default::*;
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::{collections::HashMap, sync::Mutex};

    pub static CFG: OnceCell<Cfg> = OnceCell::new();
    pub static LOG: OnceCell<crate::log::Log> = OnceCell::new();
    pub static LANG_MAP: Lazy<HashMap<String, String>> = Lazy::new(Lang::get_lang_map);
    pub static LANG: OnceCell<Lang> = OnceCell::new();
    pub static CFG_SYNTAX: OnceCell<CfgSyntax> = OnceCell::new();
    pub static CFG_EDIT: OnceCell<Mutex<Cfg>> = OnceCell::new();
}
pub mod lang {
    pub mod lang_cfg;
}

pub mod model {
    pub mod default;
    pub mod modal;
    pub mod user;
}
pub mod cfg;
pub mod cfg_edit;
pub mod cfg_file_path;
pub mod cfg_setting;
pub mod colors;
pub mod log;
pub mod setting_file_loader;
pub mod theme_loader;
