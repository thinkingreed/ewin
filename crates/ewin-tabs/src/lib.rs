#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_editor;
extern crate ewin_key;
extern crate ewin_prom;

pub mod global_term {}

pub mod evt_acts {
    pub mod prom;
    pub mod statusbar;
    pub mod tabs;
}

pub mod draws {
    pub mod draw;
}
pub mod file;
pub mod size;
pub mod tab;
pub mod tabs;

pub mod prom {
    pub mod enc_nl;
    pub mod grep_result;
    pub mod move_row;
    pub mod open_file;
    pub mod replace;
    pub mod save_forced;
    pub mod save_new_file;
    pub mod watch_file;
}

pub mod msgbar;
pub mod statusbar;
