#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]
pub mod view;

pub mod char_style;
pub mod model;
pub mod scrollbar_v;
pub mod view_traits {
    pub mod view_trait;
}
pub mod menulists {
    pub mod core;
    pub mod menulist;
}
pub mod terminal;

pub mod parts {
    pub mod pulldown;
}
