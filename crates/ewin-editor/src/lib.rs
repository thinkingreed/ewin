#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_com;

pub mod buf {
    pub mod edit;
    pub mod io;
}
pub mod view {
    pub mod buf_cache;
    pub mod render;
    pub mod render_range;
}
pub mod prompt {
    pub mod grep_result;
}
pub mod scrollbar {
    pub mod horizontal;
    pub mod vertical;
}
pub mod fmt {
    pub mod fmt_ctrl;
    pub mod xml_html;
}
pub mod convert;
pub mod editor;
pub mod model;
pub mod proc;
pub mod proc_edit;
pub mod scroll;

pub mod key {
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod search;
    pub mod select;
    pub mod un_redo;
}
