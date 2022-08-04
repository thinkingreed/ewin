#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {

    use crate::tabs::*;
    use once_cell::sync::*;
    use tokio::sync::Mutex;

    pub static TABS: OnceCell<Mutex<Tabs>> = OnceCell::new();
}

pub mod editor_state;
pub mod header_file;
pub mod tabs;
