use crate::{ewin_com::model::*, window_mgr::*};
use ewin_cfg::model::default::*;
use ewin_com::_cfg::key::cmd::*;
use ewin_const::def::*;
use ewin_widget::widget::input_comple::*;
use ropey::Rope;
use std::{
    collections::{BTreeSet, HashMap},
    fmt, usize,
};
use syntect::{highlighting::Style, parsing::SyntaxReference};

#[derive(Debug, Clone)]
pub struct Editor {
    pub buf: TextBuffer,
    pub win_mgr: WindowMgr,
    pub state: EditorState,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    pub cmd: Cmd,
    /// number displayed on the terminal
    pub row_posi: usize,
    pub row_num: usize,
    pub buf_len_rows_org: usize,
    pub search: Search,
    pub search_org: Search,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    pub key_vec: Vec<KeyMacro>,
    pub is_enable_syntax_highlight: bool,
    pub h_file: HeaderFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
    pub change_info: ChangeInfo,
    pub input_comple: InputComple,
    pub scale: Scale,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buf: TextBuffer::default(),
            win_mgr: WindowMgr::default(),
            state: EditorState::default(),
            rnw: 0,
            rnw_org: 0,
            cmd: Cmd::default(),
            row_num: TERM_MINIMUM_HEIGHT - FILEBAR_ROW_NUM - STATUSBAR_ROW_NUM,
            row_posi: Editor::get_row_posi(),
            search: Search::default(),
            search_org: Search::default(),
            history: History::default(),
            grep_result_vec: vec![],
            key_vec: vec![],
            is_enable_syntax_highlight: false,
            h_file: HeaderFile::default(),
            box_insert: BoxInsert::default(),
            buf_len_rows_org: 0,
            change_info: ChangeInfo::default(),
            input_comple: InputComple::default(),
            scale: Scale::default(),
        }
    }
    pub fn get_row_posi() -> usize {
        return MENUBAR_ROW_NUM + FILEBAR_ROW_NUM + if CfgEdit::get().general.editor.scale.is_enable { 1 } else { 0 };
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Offset {
    pub y: usize,
    pub y_org: usize,
    pub x: usize,
    pub x_org: usize,
    pub disp_x: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorState {
    pub is_changed: bool,
    pub is_changed_org: bool,
    pub is_read_only: bool,
    pub is_dragging: bool,
    pub key_macro: KeyMacroState,
    pub mouse: Mouse,
    pub input_comple_mode: InputCompleMode,
    pub input_comple_mode_org: InputCompleMode,
}

impl Default for EditorState {
    fn default() -> Self {
        EditorState { is_changed: false, is_changed_org: false, is_read_only: false, is_dragging: false, key_macro: KeyMacroState::default(), mouse: Mouse::Enable, input_comple_mode: InputCompleMode::None, input_comple_mode_org: InputCompleMode::None }
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
    pub cells_all: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
    pub style_vecs: Vec<Vec<(Style, String)>>,
    pub syntax_reference: Option<SyntaxReference>,
    pub change_row_vec: Vec<usize>,
}

impl fmt::Display for EditorDraw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, ", self.sy, self.ey)
    }
}
pub struct FormatXml {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarH {
    pub is_show: bool,
    pub is_show_org: bool,
    pub is_enable: bool,
    pub row_posi: usize,
    pub clm_posi: usize,
    pub clm_posi_org: usize,
    pub bar_len: usize,
    pub bar_height: usize,
    pub move_char_x: usize,
    pub scrl_range: usize,
}

impl Default for ScrollbarH {
    fn default() -> Self {
        ScrollbarH { is_show: false, is_show_org: false, is_enable: false, row_posi: USIZE_UNDEFINED, clm_posi: 0, clm_posi_org: 0, bar_len: 0, bar_height: 0, move_char_x: 0, scrl_range: 0 }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeInfo {
    pub change_type: EditerChangeType,
    pub new_row: BTreeSet<usize>,
    pub restayle_row_set: BTreeSet<usize>,
    pub del_row_set: BTreeSet<usize>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditerChangeType {
    #[default]
    None,
    Edit,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Scale {
    pub is_enable: bool,
}
