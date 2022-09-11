use crate::{
    cur::*,
    key::{cmd::*, keys::*},
    sel_range::*,
};
use chrono::NaiveDateTime;
use ewin_cfg::{lang::lang_cfg::Lang, log::Log};
use ewin_const::{def::*, models::model::*};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::usize;
use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
};
use syntect::highlighting::HighlightState;
use syntect::parsing::{ParseState, ScopeStackOp};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// undo,redo範囲
/// EventProcess
pub struct EvtProc {
    pub sel_proc: Option<Proc>,
    pub proc: Option<Proc>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
// Process
pub struct Proc {
    pub cmd: Cmd,
    pub cur_s: Cur,
    pub cur_e: Cur,
    pub str: String,
    pub box_sel_vec: Vec<(SelRange, String)>,
    pub box_sel_redo_vec: Vec<(SelRange, String)>,
    pub sel: SelRange,
}

impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvtProc cur_s:{}, cur_e:{}, str:{}, sel:{}, ", self.cur_s, self.cur_e, self.str, self.sel,)
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
/// All edit history including undo and redo
/// History
pub struct History {
    pub mouse_click_vec: VecDeque<(NaiveDateTime, Keys)>,
    pub undo_vec: Vec<EvtProc>,
    pub redo_vec: Vec<EvtProc>,
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "History mouse_click_vec:{:?}, undo_vec:{:?}, redo_vec:{:?}, ", self.mouse_click_vec, self.undo_vec, self.redo_vec,)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryInfo {
    pub ope_type: Opetype,
    pub evt_proc: Proc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Operation Type
pub enum Opetype {
    Normal,
    Undo,
    Redo,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrepResult {
    pub filenm: String,
    pub row_num: usize,
}
impl GrepResult {
    pub fn new(filenm: String, row_num: usize) -> Self {
        GrepResult { filenm, row_num }
    }
}
impl fmt::Display for GrepResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GrepResult filenm:{}, row_num:{},", self.filenm, self.row_num)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Search {
    pub str: String,
    pub idx: usize,
    pub ranges: Vec<SearchRange>,
    pub fullpath: String,
    pub filenm: String,
    pub dir: String,
    pub row_num: usize,
}

impl Search {
    pub fn clear(&mut self) {
        Log::debug_key("Search.clear");

        self.str = String::new();
        self.idx = USIZE_UNDEFINED;
        self.ranges = vec![];
        // file full path
        self.fullpath = String::new();
        self.dir = String::new();
    }

    pub fn get_y_range(&self) -> (usize, usize) {
        if !self.ranges.is_empty() {
            let (sy, ey) = (self.ranges.first().unwrap().y, self.ranges.last().unwrap().y);
            return (sy, ey);
        }
        (0, 0)
    }
}
impl Default for Search {
    fn default() -> Self {
        Search { str: String::new(), idx: USIZE_UNDEFINED, ranges: vec![], filenm: String::new(), fullpath: String::new(), dir: String::new(), row_num: USIZE_UNDEFINED }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMacro {
    pub cmd_type: CmdType,
    pub search: Search,
}

impl Default for KeyMacro {
    fn default() -> Self {
        KeyMacro { cmd_type: CmdType::Null, search: Search::default() }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct KeyMacroState {
    pub is_record: bool,
}

#[derive(Debug, Default, Hash, Ord, PartialOrd, Clone, Copy, PartialEq, Eq)]
/// 検索範囲
pub struct SearchRange {
    pub y: usize,
    pub sx: usize,
    pub ex: usize,
}

impl fmt::Display for SearchRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchRange y:{}, sx:{}, ex:{},", self.y, self.sx, self.ex,)
    }
}
#[derive(Debug, Clone)]
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
}

#[derive(Debug, Default, PartialEq, Hash, Eq, Clone)]
pub struct GrepInfo {
    // pub search_str: String,
    // pub search_filenm: String,
    // pub search_dir: String,
    pub search: Search,
    pub is_empty: bool,
    pub is_cancel: bool,
}

impl GrepInfo {
    pub fn clear(&mut self) {
        self.search.str = String::new();
    }
}

#[derive(Debug, Default, Copy, PartialEq, Eq, Clone)]
pub enum PromState {
    #[default]
    None,
    Search,
    SaveConfirm,
    SaveNewFile,
    SaveForced,
    Replase,
    MoveRow,
    OpenFile,
    Grep,
    Greping,
    GrepResult,
    EncNl,
    WatchFile,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawRangX {
    Range(usize, bool),
}

impl Default for DrawRangX {
    fn default() -> Self {
        DrawRangX::Range(USIZE_UNDEFINED, false)
    }
}

impl DrawRangX {
    pub fn get_x(&self) -> usize {
        match self {
            DrawRangX::Range(x, _) => {
                return *x;
            }
        }
    }

    pub fn is_margin(&self) -> bool {
        match self {
            DrawRangX::Range(_, is_margin) => {
                return *is_margin;
            }
        }
    }
}

pub type WatchHistory = BTreeSet<(String, String)>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WatchInfo {
    // pub is_watch: bool,
    pub fullpath: String,
    pub fullpath_org: String,
    // pub is_reflect_changes: bool,
    pub mode: WatchMode,
    pub history_set: WatchHistory,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GrepCancelType {
    None,
    Greping,
    Canceling,
    Canceled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mouse {
    #[default]
    Enable,

    Disable,
}

impl fmt::Display for Mouse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mouse::Enable => write!(f, ""),
            Mouse::Disable => write!(f, "{}", Lang::get().mouse_disable),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// ConvertType
pub enum ConvType {
    Lowercase,
    Uppercase,
    HalfWidth,
    FullWidth,
    Space,
    Tab,
}
impl ConvType {
    pub fn from_str_conv_type(s: &str) -> ConvType {
        return if s == Lang::get().to_lowercase {
            ConvType::Lowercase
        } else if s == Lang::get().to_uppercase {
            ConvType::Uppercase
        } else if s == Lang::get().to_half_width {
            ConvType::HalfWidth
        } else if s == Lang::get().to_full_width {
            ConvType::FullWidth
        } else if s == Lang::get().to_space {
            ConvType::Space
        } else {
            ConvType::Tab
        };
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
