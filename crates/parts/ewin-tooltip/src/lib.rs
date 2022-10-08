#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::tooltip::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;
    pub static TOOLTIP: OnceCell<Mutex<ToolTip>> = OnceCell::new();
}
pub mod core;
pub mod draw;
pub mod event;
pub mod tooltip;
