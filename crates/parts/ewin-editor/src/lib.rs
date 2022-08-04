#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
extern crate ewin_key;
pub mod buf {
    pub mod edit;
    pub mod io;
}
pub mod view {
    pub mod buf_cache;
    pub mod draw;
    pub mod draw_range;
    pub mod scale;
    pub mod view;
}
pub mod prom {
    pub mod grep_result;
}
pub mod proc {
    pub mod proc_base;
    pub mod proc_edit;
}
pub mod scrollbar {
    pub mod horizontal;
    pub mod vertical;
}
pub mod fmt {
    pub mod fmt_ctrl;
    pub mod xml_html;
}
pub mod change_info;
pub mod convert;
pub mod editor;
pub mod input_comple;
pub mod model;
pub mod scroll;
pub mod search;
pub mod window;
pub mod window_mgr;
pub mod key {
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod select;
    pub mod un_redo;
}
