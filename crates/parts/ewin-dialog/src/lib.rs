#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod global {
    use crate::dialog::*;
    use once_cell::sync::*;
    use tokio::sync::Mutex;
    pub static DIALOG: OnceCell<Mutex<Dialog>> = OnceCell::new();
}

extern crate ewin_key;

pub mod dialog_trait {
    pub mod dialog_trait;
}

pub mod view {
    pub mod view;
}

pub mod cont {
    pub mod parts {
        pub mod kvs {
            pub mod about_app;
            pub mod file_prop;
            pub mod kvs;
        }
    }
    pub mod cont;
}
pub mod btn_grourp;
pub mod core;
pub mod dialog;
pub mod draw;
pub mod evt_act;
pub mod factory;
