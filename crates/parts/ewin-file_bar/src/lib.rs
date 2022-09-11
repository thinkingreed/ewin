#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::filebar::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;
    pub static FILE_BAR: OnceCell<Mutex<FileBar>> = OnceCell::new();
}

pub mod views {
    pub mod ctx_menu;
    pub mod view;
}
pub mod draw;
pub mod event;

pub mod core;
pub mod filebar;
pub mod filebar_file;
