#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
extern crate ewin_key;

pub mod global {
    use crate::editor_gr::*;
    use once_cell::sync::OnceCell;
    use parking_lot::Mutex;
    pub static EDITOR_GR: OnceCell<Mutex<EditorGr>> = OnceCell::new();
}
pub mod buf {
    pub mod edit;
    pub mod io;
}
pub mod key_macro;
pub mod state;
pub mod draws {
    pub mod cache;
    pub mod draw;
    pub mod draw_range;
}
pub mod views {
    pub mod ctx_menu;
    pub mod view;
    pub mod view_evt;
}
pub mod event;
pub mod grep_result {
    pub mod event;
    pub mod grep;
}
pub mod proc {
    pub mod base;
    pub mod edit;
}
pub mod scrollbar {
    pub mod com;
    pub mod horizontal;
    pub mod scrl_h_trait;
    pub mod vertical;
}
pub mod fmt {
    pub mod fmt_ctrl;
    pub mod xml_html;
}
pub mod change_info;
pub mod convert;
pub mod editor;
pub mod input_comple {
    pub mod core;
    pub mod input_comple;
}
pub mod editor_gr;
pub mod model;
pub mod scale;
pub mod scroll;
pub mod search;

pub mod window {
    pub mod window;
    pub mod window_mgr;
}

pub mod key {
    pub mod cur;
    pub mod edit;
    pub mod mouse;
    pub mod save;
    pub mod select;
    pub mod un_redo;
}
