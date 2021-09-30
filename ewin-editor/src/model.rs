use crate::ewin_core::{
    _cfg::key::{keycmd::*, keys::*},
    model::*,
};
use ropey::Rope;
use std::{fmt, usize};
use syntect::parsing::SyntaxReference;

#[derive(Debug, Clone)]
pub struct Editor {
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
    pub state: EditorState,
    // Basic x position when moving the cursor up and down  line_num_width + 1 over initial:0
    pub updown_x: usize,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    //  pub sel_range: SelRange,
    pub sel: SelRange,
    pub sel_org: SelRange,
    pub keys: Keys,
    // pub keycmd: KeyCmd,
    pub e_cmd: E_Cmd,
    // Clipboard on memory
    // pub clipboard: String,
    /// number displayed on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    pub search: Search,
    // pub draw: Draw,
    pub draw_range: EditorDrawRange,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    pub key_vec: Vec<KeyMacro>,
    pub is_enable_syntax_highlight: bool,
    pub h_file: HeaderFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
}
impl Editor {
    pub fn new() -> Self {
        Editor {
            buf: TextBuffer::default(),
            buf_cache: vec![],
            cur: Cur::default(),
            offset_y: 0,
            offset_y_org: 0,
            offset_x: 0,
            offset_x_org: 0,
            offset_disp_x: 0,
            cur_y_org: 0,
            state: EditorState::default(),
            updown_x: 0,
            rnw: 0,
            rnw_org: 0,
            //  sel_range: SelRange::default(),
            sel: SelRange::default(),
            sel_org: SelRange::default(),
            keys: Keys::Null,
            // keycmd: KeyCmd::Edit(E_Cmd::Null),
            e_cmd: E_Cmd::Null,
            // for UT set
            disp_row_num: 5,
            disp_row_posi: 1,
            disp_col_num: 5,
            search: Search::default(),
            //  draw: Draw::default(),
            draw_range: EditorDrawRange::default(),
            history: History::default(),
            grep_result_vec: vec![],

            key_vec: vec![],
            is_enable_syntax_highlight: false,
            h_file: HeaderFile::default(),
            box_insert: BoxInsert::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorState {
    pub is_changed: bool,
    pub is_changed_org: bool,
    pub is_read_only: bool,
    pub key_macro: KeyMacroState,
    pub mouse_mode: MouseMode,
}

impl Default for EditorState {
    fn default() -> Self {
        EditorState { is_changed: false, is_changed_org: false, is_read_only: false, key_macro: KeyMacroState::default(), mouse_mode: MouseMode::Normal }
    }
}
impl EditorState {
    pub fn is_change_changed(&self) -> bool {
        return self.is_changed != self.is_changed_org;
    }
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
