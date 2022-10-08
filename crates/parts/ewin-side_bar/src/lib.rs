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
pub mod explorer {

    pub mod event;
    pub mod explorer;
    pub mod file;
    pub mod quick_access;
    pub mod traits;
}
pub mod traits {
    pub mod traits;
}
pub mod scrollbar {
    pub mod horizontal;
    pub mod scrl_h_trait;
}
pub mod views {
    pub mod view;
    pub mod view_evt;
}
