#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::sidebar::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static SIDE_BAR: OnceCell<Mutex<SideBar>> = OnceCell::new();
}

pub mod core;
pub mod draw;
pub mod event;
pub mod sidebar;

pub mod tree_file_view {
    pub mod cont_trait;
    pub mod event;
    pub mod mouse;
    pub mod tree;
    pub mod tree_file;
}
pub mod side_bar_trait {
    pub mod side_bar_trait;
}

pub mod views {

    pub mod view;
}
