#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod editor {
    pub mod clipboard;
    pub mod draw;
    pub mod editor;
    pub mod key;
    pub mod mouse;
}
pub mod model;
pub mod prompt;
pub mod statusbar;
pub mod terminal;
pub mod util;
pub mod _cfg {
    pub mod lang {
        pub mod cfg;
        pub mod lang;
    }
    pub mod args;
}
