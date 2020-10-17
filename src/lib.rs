pub mod color;
pub mod editor {
    pub mod clipboard;
    pub mod draw;
    pub mod editor;
    pub mod key;
    pub mod key_ctrl;
    pub mod key_shift;
    pub mod mouse;
}
pub mod model;
pub mod prompt {
    pub mod process {
        pub mod close;
        pub mod replace;
        pub mod save_new_file;
        pub mod search;
    }
    pub mod prompt;
}
pub mod msgbar;
pub mod process;

pub mod statusbar;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod lang {
        pub mod cfg;
        pub mod lang;
    }
    //   pub mod args;
}
