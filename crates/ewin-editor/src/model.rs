use crate::ewin_com::model::*;
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

#[derive(Debug, Default, Clone)]
pub struct WindowMgr {
    // Vertical, horizontal
    pub win_list: Vec<Vec<Window>>,
    pub win_v_idx: usize,
    pub win_h_idx: usize,
    pub split_type: WindowSplitType,
    pub split_line_v: usize,
    //  pub split_line_v_width: usize,
    pub split_line_h: usize,
}

impl WindowMgr {
    pub const SPLIT_LINE_V_WIDTH: usize = 1;
    pub fn curt(&mut self) -> &mut Window {
        return self.win_list.get_mut(self.win_v_idx).unwrap().get_mut(self.win_h_idx).unwrap();
    }
    pub fn curt_ref(&self) -> &Window {
        return self.win_list.get(self.win_v_idx).unwrap().get(self.win_h_idx).unwrap();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Window {
    pub v_idx: usize,
    pub h_idx: usize,
    pub area_h: (usize, usize),
    // area_h + rnw + scrollbar
    pub area_all_h: (usize, usize),
    pub area_h_org: (usize, usize),
    pub area_v: (usize, usize),
    // area_v + scrollbar
    pub area_all_v: (usize, usize),
    pub area_v_org: (usize, usize),
    /// current cursor position
    pub cur: Cur,
    pub cur_org: Cur,
    // Basic x position when moving the cursor up and down
    pub updown_x: usize,
    pub row_posi: usize,
    pub row_len_org: usize,
    pub offset: Offset,
    pub draw_range: E_DrawRange,
    pub scrl_h: ScrollbarH,
    pub scrl_v: ScrollbarV,
    pub sel: SelRange,
    pub sel_org: SelRange,
}

impl Window {
    pub fn new() -> Self {
        Window {
            v_idx: 0,
            h_idx: 0,
            area_v: (0, 0),
            area_all_v: (0, 0),
            area_h: (0, 0),
            area_all_h: (0, 0),
            area_v_org: (0, 0),
            area_h_org: (0, 0),
            cur: Cur::default(),
            cur_org: Cur::default(),
            updown_x: 0,
            row_posi: 0,
            row_len_org: 0,
            offset: Offset::default(),
            draw_range: E_DrawRange::default(),
            scrl_v: ScrollbarV::default(),
            scrl_h: ScrollbarH::default(),
            sel: SelRange::default(),
            sel_org: SelRange::default(),
        }
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
    // pub win: Window,
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
    pub bar_height: usize,
    pub row_max_width_org: usize,
    pub move_char_x: usize,
    pub scrl_range: usize,
}

impl Default for ScrollbarH {
    fn default() -> Self {
        ScrollbarH { is_show: false, is_show_org: false, is_enable: false, row_width_chars_vec: vec![], row_posi: USIZE_UNDEFINED, row_max_width_idx: 0, clm_posi: 0, clm_posi_org: 0, bar_len: 0, bar_height: 0, row_max_width: 0, row_max_width_org: 0, row_max_chars: 0, move_char_x: 0, scrl_range: 0 }
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
