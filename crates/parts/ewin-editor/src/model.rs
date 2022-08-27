use crate::window_mgr::*;
use ewin_cfg::{lang::lang_cfg::Lang, model::default::*};
use ewin_const::def::*;
use ewin_const::models::draw::*;
use ewin_key::key::cmd::*;
use ewin_key::model::*;
use ewin_key::sel_range::*;
use ewin_menulist::parts::input_comple::*;
use ewin_state::term::*;
use ewin_view::{model::Cell, view::*};
use ropey::Rope;
use std::{
    collections::{BTreeSet, HashMap},
    fmt, usize,
};
use syntect::highlighting::Style;

#[derive(Debug, Clone)]
pub struct Editor {
    pub buf: TextBuffer,
    pub win_mgr: WindowMgr,
    pub draw_range: E_DrawRange,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
    pub cmd: Cmd,
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
    pub draw_cache: Vec<Vec<EditorDraw>>,
}

impl Editor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Editor {
            buf: TextBuffer::default(),
            win_mgr: WindowMgr::default(),
            draw_range: E_DrawRange::default(),
            view: View { y: MENUBAR_ROW_NUM + FILEBAR_ROW_NUM + if CfgEdit::get().general.editor.scale.is_enable { 1 } else { 0 }, height: TERM_MINIMUM_HEIGHT - FILEBAR_ROW_NUM - STATUSBAR_ROW_NUM, ..View::default() },
            rnw: 0,
            rnw_org: 0,
            cmd: Cmd::default(),
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
            draw_cache: vec![],
        }
    }
    pub fn get_row_posi(&self) -> usize {
        return MENUBAR_ROW_NUM + FILEBAR_ROW_NUM + if State::get().curt_state().editor.scale.is_enable { 1 } else { 0 };
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
    // pub syntax_reference: Option<SyntaxReference>,
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
