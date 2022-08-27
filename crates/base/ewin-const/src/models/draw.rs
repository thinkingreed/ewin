use std::{fmt, ops::Range};

use super::key::*;

#[derive(Debug, PartialEq, Eq, Clone)]
// DrawParts
pub enum DParts {
    Editor(E_DrawRange), // and StatuusBar
    InputComple,
    Absolute(Range<usize>),
    Prompt,
    MsgBar(String),
    StatusBar,
    MenuBar,
    MenuBarMenuList,
    FileBar,
    CtxMenu,
    Dialog,
    All,
    ScrollUpDown(ScrollUpDownType),
    AllMsgBar(String),
    None,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
/// DrawType
#[allow(non_camel_case_types)]
pub enum E_DrawRange {
    TargetRange(usize, usize), // Target row only redraw
    After(usize),              // Redraw after the specified line
    #[default]
    All,
    WinOnlyAll,
    Targetpoint,
    ScrollDown(usize, usize),
    ScrollUp(usize, usize),
    MoveCur,
    Not,
}

impl fmt::Display for E_DrawRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            E_DrawRange::TargetRange(_, _) => write!(f, "Target"),
            E_DrawRange::After(_) => write!(f, "After"),
            E_DrawRange::All => write!(f, "All"),
            E_DrawRange::WinOnlyAll => write!(f, "WinOnly"),
            E_DrawRange::Targetpoint => write!(f, "AllDiff"),
            E_DrawRange::ScrollDown(_, _) => write!(f, "ScrollDown"),
            E_DrawRange::ScrollUp(_, _) => write!(f, "ScrollUp"),
            E_DrawRange::MoveCur => write!(f, "MoveCur"),
            E_DrawRange::Not => write!(f, "Not"),
        }
    }
}
