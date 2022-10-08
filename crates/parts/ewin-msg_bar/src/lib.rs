#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::msgbar::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static MSG_BAR: OnceCell<Mutex<MsgBar>> = OnceCell::new();
}

pub mod core;
pub mod draw;

pub mod msgbar;
pub mod views {
    pub mod view;
}
