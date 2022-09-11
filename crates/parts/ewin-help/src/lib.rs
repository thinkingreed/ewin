#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::help::*;
    use once_cell::sync::OnceCell;
    use parking_lot::Mutex;
    pub static HELP: OnceCell<Mutex<Help>> = OnceCell::new();
}

pub mod core;
pub mod views {
    pub mod view;
}
pub mod help;
