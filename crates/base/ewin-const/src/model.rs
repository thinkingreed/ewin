use std::{fmt, ops::Range};

use crate::def::*;

#[derive(Debug, PartialEq)]
pub enum Env {
    WSL,
    Linux,
    Windows,
}

#[derive(Debug, PartialEq)]
// ActionType
pub enum ActType {
    Cancel, // Cancel process
    None,
    Exit,
    Next, // Next Process
    Draw(DParts),
}

#[derive(Debug, PartialEq, Clone)]
// DrawParts
pub enum DParts {
    Editor(E_DrawRange), // and StatuusBar
    InputComple,
    Absolute(Range<usize>),
    Prompt,
    MsgBar(String),
    StatusBar,
    MenuBar,
    MenuWidget,
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
    #[default]
    Init,
    TargetRange(usize, usize), // Target row only redraw
    After(usize),              // Redraw after the specified line
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
            E_DrawRange::Init => write!(f, "Init"),
            E_DrawRange::Targetpoint => write!(f, "AllDiff"),
            E_DrawRange::ScrollDown(_, _) => write!(f, "ScrollDown"),
            E_DrawRange::ScrollUp(_, _) => write!(f, "ScrollUp"),
            E_DrawRange::MoveCur => write!(f, "MoveCur"),
            E_DrawRange::Not => write!(f, "Not"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScrollUpDownType {
    Normal,
    Grep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Nomal,
    Delim,
    HalfSpace,
    FullSpace,
    NewLineCode,
}

pub struct NL {}
impl NL {
    pub fn get_nl(nl_str: &str) -> String {
        if nl_str == NEW_LINE_CRLF_STR {
            NEW_LINE_CRLF.to_string()
        } else {
            NEW_LINE_LF.to_string()
        }
    }
}
// Cursor direction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}
