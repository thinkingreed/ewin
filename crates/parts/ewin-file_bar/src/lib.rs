#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::filebar::*;
    use once_cell::sync::*;
    use tokio::sync::Mutex;
    pub static FILE_BAR: OnceCell<Mutex<FileBar>> = OnceCell::new();
}

pub mod views {
    pub mod ctx_menu;
    pub mod view;
}
pub mod draw;
pub mod evt_act;

pub mod core;
pub mod filebar;
pub mod filebar_file;
