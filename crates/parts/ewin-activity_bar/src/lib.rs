#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::activitybar::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static ACTIVITY_BAR: OnceCell<Mutex<ActivityBar>> = OnceCell::new();
}

pub mod activitybar;
pub mod cont;
pub mod core;
pub mod draw;
pub mod event;
pub mod init;
pub mod traits {
    pub mod traits;
}
pub mod each {
    pub mod explorer;
    pub mod management;
    pub mod search;
}
pub mod views {
    pub mod view;
}
