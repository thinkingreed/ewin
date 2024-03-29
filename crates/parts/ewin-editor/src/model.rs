use crate::input_comple::core::*;
use crate::window::window_mgr::*;
use ewin_cfg::{lang::lang_cfg::*, model::general::default::*};
use ewin_const::def::*;
use ewin_key::key::cmd::*;
use ewin_key::key::keys::*;
use ewin_key::model::*;
use ewin_key::sel_range::*;
use ewin_state::term::*;
use ewin_view::model::Cell;
use ewin_view::view::View;
use ropey::Rope;
use std::{
    collections::{BTreeSet, HashMap},
    fmt, usize,
};
use syntect::highlighting::Style;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Editor {
    pub buf: TextBuffer,
    pub win_mgr: WindowMgr,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    pub cmd: Cmd,
    pub keys: Keys,
    /// overall size
    pub view: View,

    pub buf_len_rows_org: usize,
    pub search: Search,
    pub search_org: Search,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    pub key_vec: Vec<KeyMacro>,
    pub is_enable_syntax_highlight: bool,
    // pub h_file: FilebarFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
    pub change_info: ChangeInfo,
    pub input_comple: InputComple,
    // pub draw_cache: EditorDraw,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            buf: TextBuffer::default(),
            win_mgr: WindowMgr::default(),
            view: View { y: MENUBAR_HEIGHT + FILEBAR_HEIGHT + if CfgEdit::get().general.editor.scale.is_enable { 1 } else { 0 }, height: TERM_MINIMUM_HEIGHT - FILEBAR_HEIGHT - STATUSBAR_HEIGHT, ..View::default() },
            rnw: 0,
            rnw_org: 0,
            cmd: Cmd::default(),
            keys: Keys::Null,
            search: Search::default(),
            search_org: Search::default(),
            history: History::default(),
            grep_result_vec: vec![],
            key_vec: vec![],
            is_enable_syntax_highlight: false,
            //  h_file: FilebarFile::default(),
            box_insert: BoxInsert::default(),
            buf_len_rows_org: 0,
            change_info: ChangeInfo::default(),
            input_comple: InputComple::default(),
            // draw_cache: EditorDraw::default(),
        }
    }
}
impl Editor {
    #[allow(clippy::new_without_default)]
    pub fn get_row_posi(&self) -> usize {
        return MENUBAR_HEIGHT + FILEBAR_HEIGHT + if State::get().curt_ref_state().editor.scale.is_enable { 1 } else { 0 };
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}

#[derive(Debug, Clone, Default)]
pub struct EditorDrawCache {
    pub sy: usize,
    pub ey: usize,
    // Caching the drawing string because ropey takes a long time to access char
    pub cells_to: HashMap<usize, Vec<Cell>>,
    pub cells_from: HashMap<usize, Vec<Cell>>,
    pub cells_all: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
    pub style_vecs: Vec<Vec<(Style, String)>>,
    // pub syntax_reference: Option<SyntaxReference>,
    pub change_row_vec: Vec<usize>,
}

impl fmt::Display for EditorDrawCache {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, ", self.sy, self.ey)
    }
}

impl EditorDrawCache {
    pub fn clear(&mut self) {
        self.cells_all.clear();
        self.style_vecs.clear();
    }
}

pub struct FormatXml {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChangeInfo {
    pub change_type: EditerChangeType,
    pub new_row: BTreeSet<usize>,
    pub restayle_row_set: BTreeSet<usize>,
    pub del_row_set: BTreeSet<usize>,
}
impl ChangeInfo {
    pub fn clear(&mut self) {
        self.change_type = EditerChangeType::None;
        self.new_row = BTreeSet::new();
        self.restayle_row_set = BTreeSet::new();
        self.del_row_set = BTreeSet::new();
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditerChangeType {
    #[default]
    None,
    Edit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxInsert {
    pub mode: BoxInsertMode,
    pub vec: Vec<(SelRange, String)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxInsertMode {
    Normal,
    Insert,
}

impl Default for BoxInsert {
    fn default() -> Self {
        BoxInsert { vec: vec![], mode: BoxInsertMode::Normal }
    }
}

impl fmt::Display for BoxInsertMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BoxInsertMode::Normal => write!(f, ""),
            BoxInsertMode::Insert => write!(f, "{}", Lang::get().box_insert),
        }
    }
}

impl BoxInsert {
    pub fn clear_clipboard(&mut self) {
        self.vec = vec![]
    }
    pub fn get_str(&mut self, nl: &str) -> String {
        let mut str = String::new();
        for (_, s) in self.vec.iter() {
            str.push_str(s);
            str.push_str(nl);
        }
        str
    }
}
