extern crate ropey;
use crate::{
    _cfg::keys::{KeyCmd, Keys},
    bar::{headerbar::HeaderFile, msgbar::MsgBar},
    def::*,
    editor::{buf::edit::TextBuffer, view::char_style::*},
    global::LANG,
    log::Log,
    sel_range::{BoxInsert, SelMode, SelRange},
};
use chrono::NaiveDateTime;
use crossterm::event::{Event, KeyCode::Null};
use encoding_rs::Encoding;
use faccess::PathExt;
#[cfg(target_os = "windows")]
use regex::Regex;
#[cfg(target_os = "linux")]
use std::path::MAIN_SEPARATOR;
use std::{
    cmp::{max, min},
    collections::VecDeque,
    io::ErrorKind,
    usize,
};
use std::{fmt, path::Path};
use syntect::parsing::{ParseState, ScopeStackOp};
use syntect::{highlighting::HighlightState, parsing::SyntaxReference};

/// Event action
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, PartialEq)]
pub enum EvtActType {
    // Check next Process
    Hold,
    // Cancel process
    None,
    Exit,
    // Editor Event Process
    Next,
    // Do not Editor key Process
    DrawOnly,
}

impl EvtActType {
    pub fn check_next_process_type(evt_act_type: &EvtActType) -> bool {
        return match evt_act_type {
            EvtActType::Next | EvtActType::DrawOnly | EvtActType::None | EvtActType::Exit => true,
            EvtActType::Hold => false,
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum Env {
    WSL,
    Linux,
    Windows,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// undo,redo範囲
/// EventProcess
pub struct EvtProc {
    pub sel_proc: Option<Proc>,
    pub evt_proc: Option<Proc>,
}
impl Default for EvtProc {
    fn default() -> Self {
        EvtProc { sel_proc: None, evt_proc: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Process
pub struct Proc {
    pub keycmd: KeyCmd,
    // not include lnw
    pub cur_s: Cur,
    pub cur_e: Cur,
    pub str: String,
    pub box_sel_vec: Vec<(SelRange, String)>,
    pub box_sel_redo_vec: Vec<(SelRange, String)>,
    pub sel: SelRange,
    pub draw_type: DrawType,
}
impl Default for Proc {
    fn default() -> Self {
        Proc { cur_s: Cur::default(), cur_e: Cur::default(), str: String::new(), keycmd: KeyCmd::Null, sel: SelRange::default(), draw_type: DrawType::default(), box_sel_vec: vec![], box_sel_redo_vec: vec![] }
    }
}
impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvtProc cur_s:{}, cur_e:{}, str:{}, keycmd:{:?}, sel:{}, d_range:{}", self.cur_s, self.cur_e, self.str, self.keycmd, self.sel, self.draw_type)
    }
}
impl Proc {
    pub fn new(keycmd: KeyCmd, cur_s: Cur, cur_e: Cur, draw_type: DrawType) -> Self {
        return Proc { keycmd, cur_s, cur_e, draw_type, ..Proc::default() };
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// All edit history including undo and redo
/// History
pub struct History {
    pub mouse_click_vec: VecDeque<(NaiveDateTime, KeyCmd)>,
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
        Search { str: String::new(), idx: USIZE_UNDEFINED, ranges: vec![], filenm: String::new(), folder: String::new(), row_num: USIZE_UNDEFINED }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub struct Macros {
    pub key_macro_vec: Vec<KeyMacro>,
}

impl Default for Macros {
    fn default() -> Self {
        Macros { key_macro_vec: vec![] }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMacro {
    pub keys: Keys,
    pub search: Search,
}

impl Default for KeyMacro {
    fn default() -> Self {
        KeyMacro { keys: Keys::Null, search: Search::default() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMacroState {
    pub is_record: bool,
    pub is_exec: bool,
    pub is_exec_end: bool,
}

impl Default for KeyMacroState {
    fn default() -> Self {
        KeyMacroState { is_record: false, is_exec: false, is_exec_end: false }
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
    pub macros: Macros,
    pub is_enable_syntax_highlight: bool,
    pub h_file: HeaderFile,
    // Have sy・ey, or Copy vec
    pub box_insert: BoxInsert,
}

impl Editor {
    pub const RNW_MARGIN: usize = 1;

    pub fn new() -> Self {
        Editor {
            mouse_mode: MouseMode::Normal,
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
            //  sel_range: SelRange::default(),
            sel: SelRange::default(),
            sel_org: SelRange::default(),
            keys: Keys::Null,
            keycmd: KeyCmd::Null,
            // for UT set
            disp_row_num: 5,
            disp_row_posi: 1,
            disp_col_num: 5,
            search: Search::default(),
            //  draw: Draw::default(),
            draw_type: DrawType::default(),
            history: History::default(),
            grep_result_vec: vec![],
            macros: Macros::default(),
            is_enable_syntax_highlight: false,
            h_file: HeaderFile::default(),
            box_insert: BoxInsert::default(),
        }
    }
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
    pub fn is_readable(path_str: &str) -> bool {
        if path_str.is_empty() {
            return true;
        } else {
            let path = Path::new(path_str);
            return path.readable();
        }
    }
    pub fn is_readable_writable(path_str: &str) -> (bool, bool) {
        if path_str.is_empty() {
            return (true, true);
        } else {
            let path = Path::new(path_str);
            return (path.readable(), path.writable());
        }
    }
    pub fn is_executable(path: &String) -> bool {
        if path.is_empty() {
            return false;
        } else {
            let path = Path::new(path);
            return path.executable();
        }
    }
    #[cfg(target_os = "linux")]
    pub fn is_root_dir(path: &String) -> bool {
        return path == &MAIN_SEPARATOR.to_string();
    }

    #[cfg(target_os = "windows")]
    pub fn is_root_dir(path: &String) -> bool {
        // C:\ or D:\ ...
        let re = Regex::new(r"[a-zA-Z]:\\").unwrap();
        return re.is_match(path) && path.chars().count() == 3;
    }
    pub fn read_file(filepath: &str, mbar: &mut MsgBar) -> Option<String> {
        let is_readable = File::is_readable(&filepath);

        match TextBuffer::read(filepath) {
            Ok((string, _, _)) => return Some(string),
            Err(err) => {
                let filenm = Path::new(&filepath).file_name().unwrap().to_string_lossy().to_string();
                Log::error_s(&err.to_string());
                match err.kind() {
                    ErrorKind::PermissionDenied => {
                        if !is_readable {
                            mbar.set_err(&format!("{} {}", &filenm, &LANG.no_read_permission.clone()))
                        }
                    }
                    ErrorKind::NotFound => mbar.set_err(&format!("{} {}", &filenm, &LANG.file_not_found.clone())),
                    _ => mbar.set_err(&format!("{} {}", &filenm, &LANG.file_opening_problem.clone())),
                }
                return None;
            }
        };
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
        JobGrep { grep_str: String::new(), is_result: false, is_stdout_end: false, is_stderr_end: false, is_cancel: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobType {
    Event,
    GrepResult,
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

#[derive(Debug, Clone)]
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// DrawType
pub enum DrawType {
    Target(usize, usize), // Target row only redraw
    After(usize),         // Redraw after the specified line
    None,                 // First time
    All,
    ScrollDown(usize, usize),
    ScrollUp(usize, usize),
    MoveCur,
    Not,
}

impl Default for DrawType {
    fn default() -> Self {
        DrawType::None
    }
}

impl DrawType {
    pub fn get_type(sel_mode: SelMode, sy: usize, ey: usize) -> DrawType {
        match sel_mode {
            SelMode::Normal => {
                return DrawType::Target(min(sy, ey), max(sy, ey));
            }
            SelMode::BoxSelect => {
                return DrawType::All;
            }
        }
    }
}

impl fmt::Display for DrawType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DrawType::Target(_, _) => write!(f, "Target"),
            DrawType::After(_) => write!(f, "After"),
            DrawType::None => write!(f, "None"),
            DrawType::All => write!(f, "All"),
            DrawType::ScrollDown(_, _) => write!(f, "ScrollDown"),
            DrawType::ScrollUp(_, _) => write!(f, "ScrollUp"),
            DrawType::MoveCur => write!(f, "MoveCur"),
            DrawType::Not => write!(f, "Not"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GrepState {
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

impl Default for GrepState {
    fn default() -> Self {
        GrepState {
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

impl GrepState {
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
pub enum MouseMode {
    Normal,
    Mouse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseProc {
    DownLeft,
    DragLeft,
    DownLeftBox,
    DragLeftBox,
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
pub struct NL {}
impl NL {
    pub fn get_nl(nl_str: &str) -> String {
        if nl_str == NEW_LINE_LF_STR {
            return NEW_LINE_LF.to_string();
        } else {
            return NEW_LINE_CRLF.to_string();
        }
    }
}
// Cursor direction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CurDirection {
    Right,
    Left,
    Up,
    Down,
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
    pub fn from_str(s: &str) -> ConvType {
        if s == &LANG.to_lowercase {
            return ConvType::Lowercase;
        } else if s == &LANG.to_uppercase {
            return ConvType::Uppercase;
        } else if s == &LANG.to_half_width {
            return ConvType::HalfWidth;
        } else if s == &LANG.to_full_width {
            return ConvType::FullWidth;
        } else if s == &LANG.to_space {
            return ConvType::Space;
        } else {
            return ConvType::Tab;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// FormatType
pub enum FmtType {
    JSON,
    XML,
}

impl fmt::Display for FmtType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FmtType::JSON => write!(f, "JSON"),
            FmtType::XML => write!(f, "XML"),
        }
    }
}
impl FmtType {
    pub fn from_str(s: &str) -> FmtType {
        if s == LANG.json {
            return FmtType::JSON;
        } else {
            return FmtType::XML;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
