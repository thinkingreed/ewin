#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::term::*;
    use once_cell::sync::OnceCell;
    use std::sync::Mutex;
    pub static TERM_SIZE: OnceCell<Mutex<TermSize>> = OnceCell::new();
}

pub mod def;
pub mod term;

pub mod models {
    pub mod dialog;
    pub mod draw;
    pub mod env;
    pub mod event;
    pub mod key;
    pub mod term;
    pub mod types;
    pub mod view;

    pub mod file;
    pub mod model;
}
