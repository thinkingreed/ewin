#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::dialog::*;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static DIALOG: OnceCell<Mutex<Dialog>> = OnceCell::new();
}

extern crate ewin_key;

pub mod traits {
    pub mod traits;
}

pub mod views {
    pub mod view;
    pub mod view_evt;
}

pub mod conts {
    pub mod parts {
        pub mod kvs {
            pub mod about_app;
            pub mod core;
            pub mod file_prop;
        }
    }
    pub mod cont;
}
pub mod btn_grourp;
pub mod core;
pub mod dialog;
pub mod draw;
pub mod event;
pub mod factory;
