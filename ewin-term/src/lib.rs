extern crate ewin_core;
extern crate ewin_editor;
extern crate ewin_prom;

pub mod global_term {

    use crate::tab::Tab;
    use once_cell::sync::OnceCell;

    pub static TAB: OnceCell<tokio::sync::Mutex<Tab>> = OnceCell::new();
}
pub mod evt_act {
    pub mod _evt_act;
    pub mod ctx_menu;
    pub mod headerbar;
    pub mod prom;
    pub mod statusbar;
}
pub mod bar {
    pub mod headerbar;
    pub mod msgbar;
    pub mod statusbar;
}
pub mod ctx_menu {
    pub mod evt;
    pub mod init;
}

pub mod macros {
    pub mod js_func;
    pub mod js_macro;
    pub mod key_macro;
}

pub mod prompt {
    /*
    pub mod cont {
        pub mod cur;
        pub mod edit;
        pub mod edit_proc;
        pub mod promptcont;
        pub mod select;
        pub mod un_redo;
    }
    pub mod prompt {
        pub mod choice;
        pub mod prompt;
    }
     */

    pub mod close;
    /*pub mod enc_nl;
    pub mod grep;
    pub mod grep_result;
    pub mod menu;
    pub mod move_row;
      */
    pub mod open_file;
    pub mod replace;
    pub mod save_new_file;

    pub mod search;
}
pub mod help;
pub mod model;
pub mod tab;
pub mod terminal;
