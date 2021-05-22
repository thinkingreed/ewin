extern crate ropey;
use crate::{def::*, editor::view::char_style::*};
use chrono::NaiveDateTime;
use crossterm::event::{Event, Event::Key, KeyCode::Null};
use encoding_rs::Encoding;
#[cfg(target_os = "linux")]
use permissions::*;
use ropey::Rope;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt;
use syntect::parsing::{ParseState, ScopeStackOp};
use syntect::{highlighting::HighlightState, parsing::SyntaxReference};

/// Event後のEditor以外の操作
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, PartialEq)]
pub enum EvtActType {
    // Promt Process only
    Hold,
    Exit,
    // key Process
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
    pub str_replace: String,
    pub sel: SelRange,
    pub d_range: DRange,
}
impl Default for EvtProc {
    fn default() -> Self {
        EvtProc {
            cur_s: Cur::default(),
            cur_e: Cur::default(),
            str: String::new(),
            str_replace: String::new(),
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
        return EvtProc { evt_type: do_type, cur_s: cur_s, cur_e: cur_e, d_range: d_range, ..EvtProc::default() };
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
        History { mouse_click_vec: VecDeque::new(), undo_vec: vec![], redo_vec: vec![] }
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
    // Full path
    pub filenm: String,
    pub folder: String,
    pub row_num: usize,
}

impl Search {
    pub fn clear(&mut self) {
        self.str = String::new();
        self.idx = USIZE_UNDEFINED;
        self.ranges = vec![];
        // file full path
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
            filenm: String::new(),
            folder: String::new(),
            row_num: USIZE_UNDEFINED,
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
    pub disp_x_s: usize,
    pub disp_x_e: usize,
}
impl Default for SelRange {
    fn default() -> Self {
        SelRange { sy: 0, ey: 0, sx: 0, ex: 0, disp_x_s: 0, disp_x_e: 0 }
    }
}

impl SelRange {
    pub fn clear(&mut self) {
        //  Log::ep_s("SelRange.clear");
        self.sy = 0;
        self.ey = 0;
        self.sx = 0;
        self.ex = 0;
        self.disp_x_s = 0;
        self.disp_x_e = 0;
    }

    // For prompt buf
    pub fn clear_prompt(&mut self) {
        //  Log::ep_s("SelRange.clear");
        self.sx = USIZE_UNDEFINED;
        self.ex = USIZE_UNDEFINED;
        self.disp_x_s = USIZE_UNDEFINED;
        self.disp_x_e = USIZE_UNDEFINED;
    }

    pub fn is_selected(&self) -> bool {
        return !(self.sy == 0 && self.ey == 0 && self.disp_x_s == 0 && self.disp_x_e == 0);
    }

    /// 開始位置 < 終了位置に変換
    pub fn get_range(&self) -> Self {
        let mut sy = self.sy;
        let mut ey = self.ey;
        let mut sx = self.sx;
        let mut ex = self.ex;
        let mut s_disp_x = self.disp_x_s;
        let mut e_disp_x = self.disp_x_e;
        if sy > ey || (sy == ey && s_disp_x > e_disp_x) {
            sy = self.ey;
            ey = self.sy;
            sx = self.ex;
            ex = self.sx;
            s_disp_x = self.disp_x_e;
            e_disp_x = self.disp_x_s;
        }
        // 範囲選択が続く可能性がある為に新規構造体を返却
        SelRange { sy: sy, ey: ey, sx: sx, ex: ex, disp_x_s: s_disp_x, disp_x_e: e_disp_x }
    }

    pub fn set_s(&mut self, y: usize, x: usize, disp_x: usize) {
        self.sy = y;
        self.sx = x;
        self.disp_x_s = disp_x;
    }

    pub fn set_e(&mut self, y: usize, x: usize, disp_x: usize) {
        self.ey = y;
        self.ex = x;
        self.disp_x_e = disp_x;
    }

    pub fn check_overlap(&mut self) {
        // selectio start position and cursor overlap
        if self.sy == self.ey && self.disp_x_s == self.disp_x_e {
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
        if self.sy == sel_org.sy && self.disp_x_s == sel_org.disp_x_s {
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
        write!(f, "SelRange sy:{}, ey:{}, sx:{}, ex:{}, s_disp_x:{}, e_disp_x:{},", self.sy, self.ey, self.sx, self.ex, self.disp_x_s, self.disp_x_e)
    }
}

/// Cursor 　0-indexed
/// Editor, Prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cur {
    // Editor.buffer [y]
    pub y: usize,
    // Editor.buffer [y][x]
    pub x: usize,
    // Display position on the terminal, row num width + 1
    pub disp_x: usize,
}

impl Default for Cur {
    fn default() -> Self {
        Cur { y: 0, x: 0, disp_x: 0 }
    }
}

impl fmt::Display for Cur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cur y:{}, x:{}, disp_x:{}, ", self.y, self.x, self.disp_x)
    }
}

// エディタの内部状態
#[derive(Debug, Clone)]
pub struct Editor {
    pub mode: TermMode,
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
    // Basic x position when moving the cursor up and down  line_num_width + 1 over initial:0
    pub updown_x: usize,
    // row_number_width
    pub rnw: usize,
    pub rnw_org: usize,
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
    pub is_enable_syntax_highlight: bool,
}

impl Editor {
    pub const RNW_MARGIN: usize = 1;

    pub fn new() -> Self {
        Editor {
            mode: TermMode::Normal,
            buf: TextBuffer::default(),
            buf_cache: vec![],
            cur: Cur::default(),
            offset_y: 0,
            offset_y_org: 0,
            offset_x: 0,
            offset_x_org: 0,
            offset_disp_x: 0,
            cur_y_org: 0,
            updown_x: 0,
            rnw: 0,
            rnw_org: 0,
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
            is_enable_syntax_highlight: false,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub text: Rope,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
}

impl Default for File {
    fn default() -> Self {
        File { name: String::new(), is_dir: false }
    }
}
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File filenm:{}, is_dir:{} ", self.name, self.is_dir)
    }
}
impl File {
    #[cfg(target_os = "linux")]
    pub fn is_readable_writable(path: &String) -> (bool, bool) {
        if path.is_empty() {
            return (true, true);
        } else {
            return (is_readable(path).unwrap_or(false), is_writable(path).unwrap_or(false));
        }
    }
    #[cfg(target_os = "linux")]
    pub fn is_executable(path: &String) -> bool {
        if path.is_empty() {
            return false;
        } else {
            return is_executable(path).unwrap_or(false);
        }
    }

    #[cfg(target_os = "windows")]
    pub fn is_readable_writable(path: &String) -> (bool, bool) {
        // TDOD Permission Research
        return (true, true);
    }

    #[cfg(target_os = "windows")]
    pub fn is_executable(path: &String) -> bool {
        if path.is_empty() {
            return false;
        } else {
            return false;
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
#[derive(Debug, Clone)]
pub struct Job {
    pub job_type: JobType,
    pub job_evt: Option<JobEvent>,
    pub job_grep: Option<JobGrep>,
}

impl Default for Job {
    fn default() -> Self {
        Job { job_type: JobType::Event, job_evt: None, job_grep: None }
    }
}

#[derive(Debug, Clone)]
pub struct JobEvent {
    pub evt: Event,
}

impl Default for JobEvent {
    fn default() -> Self {
        JobEvent { evt: Event::Key(Null.into()) }
    }
}

#[derive(Debug, Clone)]
pub struct JobGrep {
    pub grep_str: String,
    pub is_result: bool,
    pub is_stdout_end: bool,
    pub is_stderr_end: bool,
    pub is_cancel: bool,
}

impl Default for JobGrep {
    fn default() -> Self {
        JobGrep {
            grep_str: String::new(),
            is_result: false,
            is_stdout_end: false,
            is_stderr_end: false,
            is_cancel: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobType {
    Event,
    GrepResult,
}

#[derive(Debug, Clone)]
pub struct Draw {
    pub sy: usize,
    pub ey: usize,
    // pub x_vec: Vec<(usize, usize)>,
    // Caching the drawing string because ropey takes a long time to access char
    pub cells: Vec<Vec<Cell>>,
    pub syntax_state_vec: Vec<SyntaxState>,
    pub syntax_reference: Option<SyntaxReference>,
}

#[derive(Debug, Clone)]
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
}

impl Default for Draw {
    fn default() -> Self {
        Draw { sy: 0, ey: 0, cells: vec![], syntax_state_vec: vec![], syntax_reference: None }
    }
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Draw y_s:{}, y_e:{}, ", self.sy, self.ey)
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
    Replace,
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
            EvtType::Replace => write!(f, "Replace"),
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

#[derive(Debug, Clone, PartialEq)]
pub struct GrepInfo {
    pub is_grep: bool,
    pub is_result: bool,
    pub is_stdout_end: bool,
    pub is_stderr_end: bool,
    pub is_cancel: bool,
    // pub is_grep_result_init: bool,
    //  pub is_grep_result_cancel: bool,
    pub search_str: String,
    pub search_folder: String,
    pub search_filenm: String,
}

impl Default for GrepInfo {
    fn default() -> Self {
        GrepInfo {
            is_grep: false,
            is_result: false,
            is_cancel: false,
            is_stdout_end: false,
            is_stderr_end: false,
            //  is_grep_result_init: false,
            //  is_grep_result_cancel: false,
            search_str: String::new(),
            search_folder: String::new(),
            search_filenm: String::new(),
        }
    }
}

impl GrepInfo {
    pub fn clear(&mut self) {
        self.is_grep = false;
        self.is_stdout_end = false;
        self.is_stderr_end = false;
        // self.is_result = false;
        self.search_str = String::new();
        self.search_folder = String::new();
        self.search_filenm = String::new();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermMode {
    Normal,
    Mouse,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encode {
    UTF8,
    UTF16LE,
    UTF16BE,
    SJIS,
    JIS,
    EucJp,
    GBK,
    Unknown,
}

impl Encode {
    pub fn into_encoding(self) -> &'static Encoding {
        match self {
            Encode::UTF16LE => return &encoding_rs::UTF_16LE_INIT,
            Encode::UTF16BE => return &encoding_rs::UTF_16BE_INIT,
            Encode::SJIS => return &encoding_rs::SHIFT_JIS_INIT,
            Encode::JIS => return &encoding_rs::ISO_2022_JP_INIT,
            Encode::EucJp => return &encoding_rs::EUC_JP_INIT,
            Encode::GBK => return &encoding_rs::GBK_INIT,
            _ => return &encoding_rs::UTF_8_INIT,
        }
    }
    pub fn from_name(name: &String) -> Encode {
        if name == &Encode::UTF16LE.to_string() {
            return Encode::UTF16LE;
        } else if name == &Encode::UTF16BE.to_string() {
            return Encode::UTF16BE;
        } else if name == &Encode::SJIS.to_string() {
            return Encode::SJIS;
        } else if name == &Encode::EucJp.to_string() {
            return Encode::EucJp;
        } else if name == &Encode::JIS.to_string() {
            return Encode::JIS;
        } else if name == &Encode::GBK.to_string() {
            return Encode::GBK;
        } else {
            return Encode::UTF8;
        }
    }

    pub fn from_encoding(from: &encoding_rs::Encoding) -> Encode {
        if from == &encoding_rs::UTF_16LE_INIT {
            return Encode::UTF16LE;
        } else if from == &encoding_rs::UTF_16BE_INIT {
            return Encode::UTF16BE;
        } else if from == &encoding_rs::SHIFT_JIS_INIT {
            return Encode::SJIS;
        } else if from == &encoding_rs::EUC_JP_INIT {
            return Encode::EucJp;
        } else if from == &encoding_rs::ISO_2022_JP_INIT {
            return Encode::JIS;
        } else if from == &encoding_rs::GBK_INIT {
            return Encode::GBK;
        } else {
            return Encode::UTF8;
        }
    }
}

impl fmt::Display for Encode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Encode::UTF8 => write!(f, "UTF-8"),
            Encode::UTF16LE => write!(f, "UTF-16LE"),
            Encode::UTF16BE => write!(f, "UTF-16BE"),
            Encode::SJIS => write!(f, "Shift_JIS"),
            Encode::JIS => write!(f, "JIS"),
            Encode::EucJp => write!(f, "EUC-JP"),
            Encode::GBK => write!(f, "GBK"),
            Encode::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// DrawRange
pub struct Choice {
    pub name: String,
    pub y: usize,
    pub area: (usize, usize),
}

impl Default for Choice {
    fn default() -> Self {
        Choice { name: String::new(), y: 0, area: (USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}

impl Choice {
    pub fn new(name: &String) -> Self {
        return Choice { name: name.clone(), ..Choice::default() };
    }
}

#[derive(Debug, Clone)]
pub struct Choices {
    pub vec: Vec<Vec<Choice>>,
    pub idx: usize,
}

impl Default for Choices {
    fn default() -> Self {
        Choices { vec: vec![], idx: USIZE_UNDEFINED }
    }
}

impl Choices {
    pub fn set_next_back_choice(&mut self, is_asc: bool) {
        // count item
        let mut total_idx = 0;
        for v in self.vec.iter_mut() {
            total_idx += v.len();
        }
        self.idx = if is_asc {
            if total_idx == self.idx + 1 {
                0
            } else {
                self.idx + 1
            }
        } else {
            if self.idx == 0 {
                total_idx - 1
            } else {
                self.idx - 1
            }
        };
    }
    pub fn get_choice(&self) -> Choice {
        let dummy_item = Choice::new(&"".to_string());
        let mut total_idx = 0;
        for v in self.vec.iter() {
            for item in v {
                if self.idx == total_idx {
                    return item.clone();
                }
                total_idx += 1;
            }
        }
        return dummy_item;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
