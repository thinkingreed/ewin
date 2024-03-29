use crate::char_style::*;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub from: CharStyle,
    pub to: CharStyle,
    pub c: char,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Region from:{:?}, to:{:?}, c:{:?},", self.from, self.to, self.c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharStyleType {
    Nomal,
    Select,
    Search,
    CtrlChar,
    ColumnCharAlignmentSpace,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Offset {
    pub y: usize,
    pub y_org: usize,
    pub x: usize,
    pub x_org: usize,
    pub disp_x: usize,
}
