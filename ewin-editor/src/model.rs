use crate::{ewin_core::_cfg::keys::*, ewin_core::model::*};
use ropey::Rope;
use std::{fmt, usize};
use syntect::parsing::SyntaxReference;

#[derive(Debug, Clone)]
pub struct Editor {
    pub mouse_mode: MouseMode,
    pub buf: TextBuffer,
    pub buf_cache: Vec<Vec<char>>,
    /// current cursor position
    pub cur: Cur,
    pub offset_y: usize,
    pub offset_y_org: usize,
    pub offset_x: usize,
    pub offset_x_org: usize,
    pub offset_disp_x: usize,
    pub cur_y_org: usize,
    pub is_changed: bool,
    // Basic x position when moving the cursor up and down  line_num_width + 1 over initial:0
    pub updown_x: usize,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    //  pub sel_range: SelRange,
    pub sel: SelRange,
    pub sel_org: SelRange,
    pub keys: Keys,
    pub keycmd: KeyCmd,
    // Clipboard on memory
    // pub clipboard: String,
    /// number displayed on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    pub search: Search,
    // pub draw: Draw,
    pub draw_type: DrawType,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    // TODO workspace
    pub key_vec: Vec<KeyMacro>,
    pub is_enable_syntax_highlight: bool,
    pub h_file: HeaderFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}

#[derive(Debug, Clone)]
pub struct EditorDraw {
    pub sy: usize,
    pub ey: usize,
    // Caching the drawing string because ropey takes a long time to access char
    pub cells: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
    pub syntax_reference: Option<SyntaxReference>,
}

impl EditorDraw {
    pub fn clear(&mut self) {
        self.syntax_state_vec.clear();
    }
}

impl Default for EditorDraw {
    fn default() -> Self {
        EditorDraw { sy: 0, ey: 0, cells: vec![], syntax_state_vec: vec![], syntax_reference: None }
    }
}

impl fmt::Display for EditorDraw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, ", self.sy, self.ey)
    }
}

pub struct FormatXml {}
