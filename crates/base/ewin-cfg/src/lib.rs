#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::lang::lang_cfg::*;
    use crate::model::general::default::*;
    use once_cell::sync::Lazy;
    use once_cell::sync::OnceCell;
    use std::{collections::HashMap, sync::Mutex};

    pub static CFG: OnceCell<Cfg> = OnceCell::new();
    pub static CFG_EDIT: OnceCell<Mutex<Cfg>> = OnceCell::new();
    pub static CFG_SYNTAX: OnceCell<CfgSyntax> = OnceCell::new();
    pub static LOG: OnceCell<crate::log::Log> = OnceCell::new();
    pub static LANG_MAP: Lazy<HashMap<String, String>> = Lazy::new(Lang::get_lang_map);
    pub static LANG: OnceCell<Lang> = OnceCell::new();
}
pub mod lang {
    pub mod lang_cfg;
}

pub mod model {

    pub mod color {
        pub mod default;
        pub mod user;
    }

    pub mod system {
        pub mod default;
        pub mod user;
    }
    pub mod general {
        pub mod default;
        pub mod user;
    }
    pub mod modal;
}
pub mod cfg;
pub mod cfg_edit;
pub mod cfg_file_path;
pub mod cfg_setting;
pub mod colors;
pub mod log;
pub mod setting_file_loader;
pub mod theme_loader;
