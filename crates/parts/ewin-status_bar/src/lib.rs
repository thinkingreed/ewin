#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::statusbar::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static STATUS_BAR: OnceCell<Mutex<StatusBar>> = OnceCell::new();
}
pub mod views {
    pub mod view;
}
pub mod core;
pub mod draw;
pub mod evt;
pub mod statusbar;
