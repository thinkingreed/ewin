#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use once_cell::sync::*;
    use parking_lot::Mutex;

    use crate::ctx_menu::*;
    pub static CTX_MENU: OnceCell<Mutex<CtxMenu>> = OnceCell::new();
}

pub mod views {
    pub mod view;
}
pub mod traits {
    pub mod traits;
}
pub mod keys {
    pub mod key;
}
pub mod ctx_menu;
pub mod init;

pub mod event;
