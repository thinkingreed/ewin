extern crate ropey;
use crate::{def::*, editor::view::char_style::*};
use chrono::NaiveDateTime;
use crossterm::event::{Event, Event::Key, KeyCode::Null};
use ropey::Rope;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::{fmt, path};
use syntect::highlighting::HighlightState;
use syntect::parsing::{ParseState, ScopeStackOp};

/// Event後のEditor以外の操作
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, PartialEq)]
pub enum EvtActType {
    // Promt Process only
    Hold,
    Exit,
    // Editor key Process
    Next,
    // Do not Editor key Process
    DrawOnly,
}

#[derive(Debug, PartialEq)]
pub enum Env {
    WSL,
    Linux,
    Windows,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// undo,redo範囲
/// EvtProcess
pub struct EvtProc {
    pub evt_type: EvtType,
    // not include lnw
    pub cur_s: Cur,
    pub cur_e: Cur,
    pub str: String,
    pub sel: SelRange,
    pub d_range: DRange,
}
impl Default for EvtProc {
    fn default() -> Self {
        EvtProc {
            cur_s: Cur::default(),
            cur_e: Cur::default(),
            str: String::new(),
            evt_type: EvtType::None,
            sel: SelRange::default(),
            d_range: DRange::default(),
        }
    }
}
impl fmt::Display for EvtProc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvtProc cur_s:{}, cur_e:{}, str:{}, do_type:{}, sel:{}, d_range:{}", self.cur_s, self.cur_e, self.str, self.evt_type, self.sel, self.d_range)
    }
}
impl EvtProc {
    pub fn new(do_type: EvtType, cur_s: Cur, cur_e: Cur, d_range: DRange) -> Self {
        return EvtProc {
            evt_type: do_type,
            cur_s: cur_s,
            cur_e: cur_e,
            d_range: d_range,
            ..EvtProc::default()
        };
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// All edit history including undo and redo
/// History
pub struct History {
    pub mouse_click_vec: VecDeque<(NaiveDateTime, Event)>,
    pub undo_vec: Vec<EvtProc>,
    pub redo_vec: Vec<EvtProc>,
}

impl Default for History {
    fn default() -> Self {
        History {
            mouse_click_vec: VecDeque::new(),
            undo_vec: vec![],
            redo_vec: vec![],
        }
    }
}
impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "History mouse_click_vec:{:?}, undo_vec:{:?}, redo_vec:{:?}, ", self.mouse_click_vec, self.undo_vec, self.redo_vec,)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryInfo {
    pub ope_type: Opetype,
    pub evt_proc: EvtProc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Operation Type
pub enum Opetype {
    Normal,
    Undo,
    Redo,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// 検索範囲
pub struct GrepResult {
    pub filenm: String,
    pub row_num: usize,
}
impl GrepResult {
    pub fn new(filenm: String, row_num: usize) -> Self {
        return GrepResult { filenm: filenm, row_num: row_num };
    }
}
impl fmt::Display for GrepResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GrepResult filenm:{}, row_num:{},", self.filenm, self.row_num)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Search {
    pub str: String,
    pub idx: usize,
    pub ranges: Vec<SearchRange>,
    pub file: String,
    pub filenm: String,
    pub folder: String,
    pub row_num: String,
}

impl Search {
    pub fn clear(&mut self) {
        self.str = String::new();
        self.idx = USIZE_UNDEFINED;
        self.ranges = vec![];
        // file full path
        self.file = String::new();
        self.filenm = String::new();
        self.folder = String::new();
    }

    pub fn get_y_range(&self) -> (usize, usize) {
        if !self.ranges.is_empty() {
            let (sy, ey) = (self.ranges.first().unwrap().y, self.ranges.last().unwrap().y);
            return (sy, ey);
        }
        return (0, 0);
    }
}
impl Default for Search {
    fn default() -> Self {
        Search {
            str: String::new(),
            idx: USIZE_UNDEFINED,
            ranges: vec![],
            file: String::new(),
            filenm: String::new(),
            folder: String::new(),
            row_num: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyRecord {
    pub evt: Event,
    pub search: Search,
}

impl Default for KeyRecord {
    fn default() -> Self {
        KeyRecord { evt: Event::Resize(0, 0), search: Search::default() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// 検索範囲
pub struct SearchRange {
    pub y: usize,
    pub sx: usize,
    pub ex: usize,
}

impl Default for SearchRange {
    fn default() -> Self {
        SearchRange { y: 0, sx: 0, ex: 0 }
    }
}

impl fmt::Display for SearchRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchRange y:{}, sx:{}, ex:{},", self.y, self.sx, self.ex,)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// SelectRange
/// マウスの選択操作関連 0-indexed
pub struct SelRange {
    // y 0-indexed
    pub sy: usize,
    pub ey: usize,
    // x 0-indexed (line_num_width含まない)
    pub sx: usize,
    pub ex: usize,
    // disp_x 1-indexed(line_num_width含む) disp_xに合せる為
    pub s_disp_x: usize,
    pub e_disp_x: usize,
}
impl Default for SelRange {
    fn default() -> Self {
        SelRange { sy: 0, ey: 0, sx: 0, ex: 0, s_disp_x: 0, e_disp_x: 0 }
    }
}

impl SelRange {
    pub fn clear(&mut self) {
        //  Log::ep_s("SelRange.clear");
        self.sy = 0;
        self.ey = 0;
        self.sx = 0;
        self.ex = 0;
        self.s_disp_x = 0;
        self.e_disp_x = 0;
    }

    // For prompt buf
    pub fn clear_prompt(&mut self) {
        //  Log::ep_s("SelRange.clear");
        self.sx = USIZE_UNDEFINED;
        self.ex = USIZE_UNDEFINED;
        self.s_disp_x = USIZE_UNDEFINED;
        self.e_disp_x = USIZE_UNDEFINED;
    }

    pub fn is_selected(&self) -> bool {
        return !(self.sy == 0 && self.ey == 0 && self.s_disp_x == 0 && self.e_disp_x == 0);
    }

    /// 開始位置 < 終了位置に変換
    pub fn get_range(&self) -> Self {
        let mut sy = self.sy;
        let mut ey = self.ey;
        let mut sx = self.sx;
        let mut ex = self.ex;
        let mut s_disp_x = self.s_disp_x;
        let mut e_disp_x = self.e_disp_x;
        if sy > ey || (sy == ey && s_disp_x > e_disp_x) {
            sy = self.ey;
            ey = self.sy;
            sx = self.ex;
            ex = self.sx;
            s_disp_x = self.e_disp_x;
            e_disp_x = self.s_disp_x;
        }
        // 範囲選択が続く可能性がある為に新規構造体を返却
        SelRange {
            sy: sy,
            ey: ey,
            sx: sx,
            ex: ex,
            s_disp_x: s_disp_x,
            e_disp_x: e_disp_x,
        }
    }

    pub fn set_s(&mut self, y: usize, x: usize, disp_x: usize) {
        self.sy = y;
        self.sx = x;
        self.s_disp_x = disp_x;
    }

    pub fn set_e(&mut self, y: usize, x: usize, disp_x: usize) {
        self.ey = y;
        self.ex = x;
        self.e_disp_x = disp_x;
    }

    pub fn check_overlap(&mut self) {
        // selectio start position and cursor overlap
        if self.sy == self.ey && self.s_disp_x == self.e_disp_x {
            self.clear();
        }
    }
    pub fn set_sel_posi(&mut self, is_start: bool, y: usize, x: usize, disp_x: usize) {
        if is_start {
            if !self.is_selected() {
                self.set_s(y, x, disp_x);
            }
        } else {
            self.set_e(y, x, disp_x);
        }
    }
    pub fn is_another_select(&mut self, sel_org: SelRange) -> bool {
        if self.sy == sel_org.sy && self.s_disp_x == sel_org.s_disp_x {
            return false;
        }
        return true;
    }
    pub fn get_diff_y_mouse_drag(&mut self, sel_org: SelRange, cur_y: usize) -> usize {
        let sel = self.get_range();
        let sel_org = sel_org.get_range();

        if sel.sy < sel_org.sy {
            return sel.sy;
        } else if sel.sy > sel_org.sy {
            return sel.sy - 1;
        } else if sel.ey > sel_org.ey {
            return sel.ey - 1;
        } else if sel.ey < sel_org.ey {
            return sel.ey;
        } else if sel.sy == cur_y {
            return sel.sy;
        //sel.ey == cur_y
        } else {
            return sel.ey - 1;
        }
    }
}

impl fmt::Display for SelRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelRange sy:{}, ey:{}, sx:{}, ex:{}, s_disp_x:{}, e_disp_x:{},", self.sy, self.ey, self.sx, self.ex, self.s_disp_x, self.e_disp_x)
    }
}

/// Cursor 　0-indexed
/// Editor, Prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cur {
    // Editor.bufferの[y]
    pub y: usize,
    // Editor.bufferの[y][x] + line_num_width
    pub x: usize,
    // 2文字分文字対応 line_num_width + 1以上
    pub disp_x: usize,
}

impl Default for Cur {
    fn default() -> Self {
        Cur { y: 0, x: 0, disp_x: 1 }
    }
}

impl fmt::Display for Cur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cur y:{}, x:{}, disp_x:{}, ", self.y, self.x, self.disp_x)
    }
}

// エディタの内部状態
#[derive(Debug)]
pub struct Editor {
    pub buf: TextBuffer,
    pub buf_cache: Vec<Vec<char>>,
    /// current cursor position
    pub cur: Cur,
    pub offset_y: usize,
    pub offset_x: usize,
    pub offset_disp_x: usize,
    pub cur_y_org: usize,
    // Basic x position when moving the cursor up and down  line_num_width + 1 over initial:0
    pub updown_x: usize,
    // row_number_width
    pub rnw: usize,
    pub sel: SelRange,
    pub sel_org: SelRange,
    pub evt: Event,
    // Clipboard on memory
    pub clipboard: String,
    /// number displayed on the terminal
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    pub search: Search,
    pub draw: Draw,
    // draw_ranges
    pub d_range: DRange,
    pub history: History,
    pub grep_result_vec: Vec<GrepResult>,
    pub key_record_vec: Vec<KeyRecord>,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            buf: TextBuffer::default(),
            buf_cache: vec![],
            cur: Cur::default(),
            offset_y: 0,
            offset_x: 0,
            offset_disp_x: 0,
            cur_y_org: 0,
            updown_x: 0,
            rnw: 0,
            sel: SelRange::default(),
            sel_org: SelRange::default(),
            evt: Key(Null.into()),
            clipboard: String::new(),
            // for UT set
            disp_row_num: 5,
            disp_row_posi: 1,
            disp_col_num: 5,
            search: Search::default(),
            draw: Draw::default(),
            d_range: DRange::default(),
            history: History::default(),
            grep_result_vec: vec![],
            key_record_vec: vec![],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub filenm: String,
    pub ext: String,
    pub path: Option<path::PathBuf>,
    pub is_changed: bool,
    pub is_enable_syntax_highlight: bool,
}

impl Default for File {
    fn default() -> Self {
        File {
            filenm: String::new(),
            ext: String::new(),
            path: None,
            is_changed: false,
            is_enable_syntax_highlight: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    CtrlChar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Nomal,
    Delim,
    HalfSpace,
    FullSpace,
}

#[derive(Debug)]
pub struct Draw {
    pub sy: usize,
    pub ey: usize,
    // pub x_vec: Vec<(usize, usize)>,
    // Caching the drawing string because ropey takes a long time to access char
    pub char_vec: Vec<Vec<char>>,
    pub cells: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
}

#[derive(Debug, Clone)]
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
}

impl Default for Draw {
    fn default() -> Self {
        Draw {
            sy: 0,
            ey: 0,
            char_vec: vec![],
            cells: vec![],
            syntax_state_vec: vec![],
        }
    }
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, char_vec:{:?}, ", self.sy, self.ey, self.char_vec)
    }
}
impl Draw {
    pub fn new(sy: usize, ey: usize) -> Self {
        return Draw { sy: sy, ey: ey, ..Draw::default() };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// DrawRange
pub struct DRange {
    pub sy: usize,
    pub ey: usize,
    pub draw_type: DrawType,
}

impl Default for DRange {
    fn default() -> Self {
        DRange { sy: 0, ey: 0, draw_type: DrawType::None }
    }
}

impl DRange {
    pub fn new(sy: usize, ey: usize, d_type: DrawType) -> Self {
        return DRange { sy: sy, ey: ey, draw_type: d_type };
    }

    pub fn get_range(&mut self) -> Self {
        return DRange { sy: self.sy, ey: self.ey, draw_type: self.draw_type };
    }
    pub fn set_target(&mut self, sy: usize, ey: usize) {
        self.draw_type = DrawType::Target;
        self.sy = min(sy, ey);
        self.ey = max(sy, ey);
    }
    pub fn set_after(&mut self, sy: usize) {
        self.draw_type = DrawType::After;
        self.sy = sy;
    }

    pub fn clear(&mut self) {
        self.sy = 0;
        self.ey = 0;
        self.draw_type = DrawType::Not;
    }
}
impl fmt::Display for DRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DRnage sy:{}, ey:{}, d_type:{}, ", self.sy, self.ey, self.draw_type)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// DrawType
pub enum DrawType {
    Target, // Target row only redraw
    After,  // Redraw after the specified line
    None,   // First time
    All,
    ScrollDown,
    ScrollUp,
    MoveCur,
    Not,
}

impl fmt::Display for DrawType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DrawType::Target => write!(f, "Target"),
            DrawType::After => write!(f, "After"),
            DrawType::None => write!(f, "None"),
            DrawType::All => write!(f, "All"),
            DrawType::ScrollDown => write!(f, "ScrollDown"),
            DrawType::ScrollUp => write!(f, "ScrollUp"),
            DrawType::MoveCur => write!(f, "MoveCur"),
            DrawType::Not => write!(f, "Not"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// UnDo ReDo Type
pub enum EvtType {
    Del,
    Enter,
    BS,
    Cut,
    Paste,
    InsertChar,
    ShiftDown,
    ShiftUp,
    ShiftRight,
    ShiftLeft,
    ShiftHome,
    ShiftEnd,
    None,
}
impl fmt::Display for EvtType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvtType::Del => write!(f, "Del"),
            EvtType::Enter => write!(f, "Enter"),
            EvtType::BS => write!(f, "BS"),
            EvtType::Cut => write!(f, "Cut"),
            EvtType::Paste => write!(f, "Paste"),
            EvtType::InsertChar => write!(f, "InsertChar"),
            EvtType::ShiftDown => write!(f, "ShiftDown"),
            EvtType::ShiftUp => write!(f, "ShiftUp"),
            EvtType::ShiftRight => write!(f, "ShiftRight"),
            EvtType::ShiftLeft => write!(f, "ShiftLeft"),
            EvtType::ShiftHome => write!(f, "ShiftHome"),
            EvtType::ShiftEnd => write!(f, "ShiftEnd"),
            EvtType::None => write!(f, "None"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
