#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
extern crate ewin_key;

pub mod global {
    use crate::menubar::MenuBar;
    use once_cell::sync::OnceCell;
    use parking_lot::Mutex;
    pub static MENU_BAR: OnceCell<Mutex<MenuBar>> = OnceCell::new();
}

pub mod core;
pub mod draw;
pub mod event;
pub mod menubar;
pub mod menulist;

pub mod keys {
    pub mod key;
}
