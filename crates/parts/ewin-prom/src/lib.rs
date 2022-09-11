#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
#[macro_use]
extern crate downcast;
extern crate ewin_key;
pub mod global {
    use crate::model::Prom;
    use once_cell::sync::*;
    use parking_lot::Mutex;

    pub static PROM: OnceCell<Mutex<Prom>> = OnceCell::new();
}
pub mod core;
pub mod event;
pub mod model;
pub mod prom;
pub mod prom_base;
pub mod util;
pub mod prom_trait {
    pub mod cont_trait;
    pub mod main_trait;
}
pub mod views {
    pub mod view;
}

pub mod each {
    pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod greping;
    pub mod move_row;
    pub mod open_file;
    pub mod replace;
    pub mod save_confirm;
    pub mod save_forced;
    pub mod save_new_file;
    pub mod search;
    pub mod watch_file;
}
pub mod cont {
    pub mod parts {
        pub mod choice;
        pub mod file_list;
        pub mod info;
        pub mod input_area;
        pub mod key_desc;
        pub mod path_comp;
        pub mod pulldown;
        pub mod search_opt;
    }
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod proc_edit;
    pub mod select;
    pub mod un_redo;
}
