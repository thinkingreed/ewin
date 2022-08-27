#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
extern crate ewin_key;

pub mod global {
    use crate::menubar::MenuBar;
    use once_cell::sync::OnceCell;
    use tokio::sync::Mutex;
    pub static MENU_BAR: OnceCell<Mutex<MenuBar>> = OnceCell::new();
}

pub mod evt_act;
pub mod menubar;
pub mod parts {
    pub mod input_comple;
    pub mod menubar;
    pub mod pulldown;
}

pub mod keys {
    pub mod key;
}
