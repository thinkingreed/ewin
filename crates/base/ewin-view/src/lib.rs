#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod view;

pub mod char_style;
pub mod model;

pub mod scrollbar {
    pub mod horizontal;
    pub mod scrl_h_trait;
    pub mod vertical;
}
pub mod traits {
    pub mod view;
    pub mod view_evt;
}
pub mod menulists {
    pub mod core;
    pub mod menulist;
}
pub mod term;

pub mod parts {
    pub mod pulldown;
}
