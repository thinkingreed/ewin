#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_com;
extern crate ewin_editor;
extern crate ewin_prom;

pub mod global_term {
    use crate::tab::Tab;
    use once_cell::sync::OnceCell;
    pub static TAB: OnceCell<tokio::sync::Mutex<Tab>> = OnceCell::new();
}
pub mod evt_act {
    pub mod _com;
    pub mod ctx_menu;
    pub mod editor;
    pub mod headerbar;
    pub mod input_comple;
    pub mod prom;
    pub mod statusbar;
}
pub mod bar {
    pub mod headerbar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod macros {
    pub mod js_func;
    pub mod js_macro;
    pub mod key_macro;
}
pub mod prompt {
    pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod menu;
    pub mod move_row;
    pub mod open_file;
    pub mod replace;
    pub mod save_confirm;
    pub mod save_forced;
    pub mod save_new_file;
    pub mod search;
    pub mod watch_result;
}
pub mod help;
pub mod model;
pub mod tab;
pub mod terminal;
