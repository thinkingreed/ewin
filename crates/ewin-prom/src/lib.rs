#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_com;

pub mod cont {
    pub mod choice;
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod proc_edit;
    pub mod promptcont;
    pub mod select;
    pub mod un_redo;
}

pub mod prom {
    pub mod choice;
    pub mod prom_ctrl;
    pub mod set_draw_posi;
}

pub mod enc_nl;
pub mod grep;
pub mod grep_result;
pub mod menu;
pub mod model;
pub mod move_row;
pub mod open_file;
pub mod replace;
pub mod save_confirm;
pub mod save_forced;
pub mod save_new_file;
pub mod search;
pub mod watch_result;
