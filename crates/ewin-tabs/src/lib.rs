#![allow(clippy::needless_return, clippy::iter_nth_zero, clippy::type_complexity)]

extern crate ewin_editor;
extern crate ewin_key;
extern crate ewin_prom;

pub mod draw;
pub mod event;
pub mod file;
pub mod size;
pub mod tabs;

pub mod tab {
    pub mod grep;
    pub mod tab;
}
