extern crate ewin_core;

pub mod buf {
    pub mod edit;
    pub mod io;
}
pub mod view {
    pub mod buf_cache;
    pub mod draw;
}
pub mod prompt {
    pub mod grep_result;
}
pub mod format {
    pub mod format;
    pub mod xml_html;
}
pub mod convert;
pub mod draw_range;
pub mod edit_proc;
pub mod editor;
pub mod model;

pub mod key {
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod search;
    pub mod select;
    pub mod un_redo;
}
