#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use once_cell::sync::OnceCell;
    use std::sync::Mutex;

    use crate::term::TermSize;

    pub static TERM_SIZE: OnceCell<Mutex<TermSize>> = OnceCell::new();
}

pub mod def;
pub mod model;
pub mod term;
