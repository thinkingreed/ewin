#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::{
        key::{cmd::*, keys::*},
        model::*,
    };
    use ewin_const::models::term::*;
    use once_cell::sync::OnceCell;
    use std::collections::HashMap;
    use tokio::sync::Mutex;

    pub static CMD_MAP: OnceCell<HashMap<(Keys, Place), Cmd>> = OnceCell::new();
    pub static CMD_TYPE_MAP: OnceCell<HashMap<CmdType, Keys>> = OnceCell::new();
    pub static APP_VERSION: OnceCell<String> = OnceCell::new();

    // Cancel is defined independently. Because it needs to be obtained when GREP_INFO_VEC is locked
    pub static GREP_CANCEL_VEC: OnceCell<Mutex<Vec<GrepCancelType>>> = OnceCell::new();
    pub static WATCH_INFO: OnceCell<Mutex<WatchInfo>> = OnceCell::new();

    // Clipboard on memory
    pub static CLIPBOARD: OnceCell<String> = OnceCell::new();
}

pub mod clipboard;
pub mod key_traits {
    pub mod key_trait;
}

pub mod cur;
pub mod history;
pub mod model;
pub mod sel_range;
pub mod watcher;

pub mod key {
    pub mod cmd;
    pub mod keybind;
    pub mod keys;
}
