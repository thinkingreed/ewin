#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::term::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static TABS: OnceCell<Mutex<State>> = OnceCell::new();
}

pub mod editor;
pub mod term;
pub mod tabs {
    pub mod all;
    pub mod tab;
    pub mod tabs;
}
pub mod core;
