use crate::ewin_com::{_cfg::key::keycmd::*, def::*, model::*};
use ropey::Rope;
use std::{
    collections::{BTreeSet, HashMap},
    fmt, usize,
};
use syntect::{highlighting::Style, parsing::SyntaxReference};

#[derive(Debug, Clone)]
pub struct Editor {
    pub buf: TextBuffer,
    /// current cursor position
    pub cur: Cur,
    // Used for display position setting regardless of cursor position
    pub offset_y: usize,
    pub offset_y_org: usize,
    pub offset_x: usize,
    pub offset_x_org: usize,
    pub offset_disp_x: usize,
    pub offset_disp_x_org: usize,
    pub cur_org: Cur,
    pub state: EditorState,
    // Basic x position when moving the cursor up and down  line_num_width + 1 over initial:0
    pub updown_x: usize,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    //  pub sel_range: SelRange,
    pub sel: SelRange,
    pub sel_org: SelRange,
    pub e_cmd: E_Cmd,
    // Clipboard on memory
    // pub clipboard: String,
    /// number displayed on the terminal
    pub row_disp_len: usize,
    pub row_disp_len_org: usize,
    pub row_len_org: usize,
    pub row_posi: usize,
    pub col_len: usize,
    pub col_len_org: usize,
    pub search: Search,
    pub search_org: Search,
    // pub draw: Draw,
    pub draw_range: E_DrawRange,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    pub key_vec: Vec<KeyMacro>,
    pub is_enable_syntax_highlight: bool,
    pub h_file: HeaderFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
    pub scrl_v: ScrollbarV,
    pub scrl_h: ScrollbarH,
    pub change_info: ChangeInfo,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buf: TextBuffer::default(),
            cur: Cur::default(),
            cur_org: Cur::default(),
            offset_y: 0,
            offset_y_org: 0,
            offset_x: 0,
            offset_x_org: 0,
            offset_disp_x: 0,
            offset_disp_x_org: 0,
            state: EditorState::default(),
            updown_x: 0,
            rnw: 0,
            rnw_org: 0,
            sel: SelRange::default(),
            sel_org: SelRange::default(),
            e_cmd: E_Cmd::Null,
            // for UT set
            row_disp_len: TERM_MINIMUM_HEIGHT - HEADERBAR_ROW_NUM - STATUSBAR_ROW_NUM,
            row_disp_len_org: 0,
            row_posi: 1,
            col_len: TERM_MINIMUM_WIDTH - Editor::RNW_MARGIN,
            col_len_org: 0,
            search: Search::default(),
            search_org: Search::default(),
            draw_range: E_DrawRange::default(),
            history: History::default(),
            grep_result_vec: vec![],

            key_vec: vec![],
            is_enable_syntax_highlight: false,
            h_file: HeaderFile::default(),
            box_insert: BoxInsert::default(),
            scrl_v: ScrollbarV::default(),
            scrl_h: ScrollbarH::default(),
            row_len_org: 0,
            change_info: ChangeInfo::default(),
        }
    }
}

impl Default for Editor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorState {
    pub is_changed: bool,
    pub is_changed_org: bool,
    pub is_read_only: bool,
    pub is_dragging: bool,
    pub key_macro: KeyMacroState,
    pub mouse_mode: MouseMode,
}

impl Default for EditorState {
    fn default() -> Self {
        EditorState { is_changed: false, is_changed_org: false, is_read_only: false, is_dragging: false, key_macro: KeyMacroState::default(), mouse_mode: MouseMode::Normal }
    }
}
impl EditorState {
    pub fn is_change_changed(&self) -> bool {
        self.is_changed != self.is_changed_org
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}

#[derive(Debug, Clone, Default)]
pub struct EditorDraw {
    pub sy: usize,
    pub ey: usize,

    // Caching the drawing string because ropey takes a long time to access char
    pub cells_to: HashMap<usize, Vec<Cell>>,
    pub cells_from: HashMap<usize, Vec<Cell>>,
    pub cells_to_all: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
    pub style_vecs: Vec<Vec<(Style, String)>>,
    pub syntax_reference: Option<SyntaxReference>,
    pub change_row_vec: Vec<usize>,
    // pub highlighter_opt: Option<Highlighter>,
}

impl EditorDraw {
    pub fn clear(&mut self) {
        self.syntax_state_vec.clear();
        self.style_vecs.clear();
        self.cells_to_all.clear();
    }
}

impl fmt::Display for EditorDraw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, ", self.sy, self.ey)
    }
}
pub struct FormatXml {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarV {
    pub is_show: bool,
    pub is_enable: bool,
    // Not include　editor.row_posi
    pub row_posi: usize,
    pub row_posi_org: usize,
    pub bar_len: usize,
    pub move_len: usize,
}

impl Default for ScrollbarV {
    fn default() -> Self {
        ScrollbarV { is_show: false, is_enable: false, row_posi: USIZE_UNDEFINED, row_posi_org: USIZE_UNDEFINED, bar_len: USIZE_UNDEFINED, move_len: USIZE_UNDEFINED }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarH {
    pub is_show: bool,
    pub is_show_org: bool,
    pub is_enable: bool,
    pub row_max_width_idx: usize,
    pub row_max_width: usize,
    pub row_max_chars: usize,
    // pub row_chars_vec: Vec<usize>,
    pub row_width_chars_vec: Vec<(usize, usize)>,
    pub row_posi: usize,
    pub clm_posi: usize,
    pub clm_posi_org: usize,
    pub bar_len: usize,
    pub row_max_width_org: usize,
    pub move_char_x: usize,
    pub scrl_range: usize,
}

impl Default for ScrollbarH {
    fn default() -> Self {
        ScrollbarH { is_show: false, is_show_org: false, is_enable: false, row_width_chars_vec: vec![], row_posi: USIZE_UNDEFINED, row_max_width_idx: 0, clm_posi: 0, clm_posi_org: 0, bar_len: 0, row_max_width: 0, row_max_width_org: 0, row_max_chars: 0, move_char_x: 0, scrl_range: 0 }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeInfo {
    pub change_type: EditerChangeType,
    pub new_row: BTreeSet<usize>,
    pub restayle_row: BTreeSet<usize>,
    pub del_row: BTreeSet<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditerChangeType {
    StyleOnly,
    Edit,
    None,
}
impl Default for EditerChangeType {
    fn default() -> Self {
        EditerChangeType::None
    }
}
