use crate::{
    _cfg::key::{
        cmd::{Cmd, CmdType},
        keys::*,
    },
    char_style::*,
    global::*,
};
use chrono::NaiveDateTime;
use crossterm::event::{Event, KeyCode::Null};
use encoding_rs::Encoding;
use ewin_cfg::{lang::lang_cfg::Lang, log::Log};
use ewin_const::def::*;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::usize;
use std::{
    collections::{BTreeSet, VecDeque},
    fmt,
    ops::Range,
    path::Path,
    time::SystemTime,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Search {
    pub str: String,
    pub idx: usize,
    pub ranges: Vec<SearchRange>,
    pub fullpath: String,
    pub dir: String,
    pub row_num: usize,
}

impl Search {
    pub fn set_info(&mut self, search_str: String, search_filenm: String, search_dir: String) {
        self.str = search_str;
        self.fullpath = search_filenm;
        self.dir = search_dir;
    }
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
        Search { str: String::new(), idx: USIZE_UNDEFINED, ranges: vec![], fullpath: String::new(), dir: String::new(), row_num: USIZE_UNDEFINED }
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct KeyMacroState {
    pub is_record: bool,
    pub is_exec: bool,
    pub is_exec_end: bool,
}

impl KeyMacroState {
    pub fn is_running(&self) -> bool {
        self.is_exec && !self.is_exec_end
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
/// Cursor 　0-indexed
/// Editor, Prompt
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cur {
    // Editor.buffer [y]
    pub y: usize,
    // Editor.buffer [y][x]
    pub x: usize,
    // Display position on the terminal, row num width + 1
    pub disp_x: usize,
}

impl fmt::Display for Cur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cur y:{}, x:{}, disp_x:{}, ", self.y, self.x, self.disp_x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Search,
    CtrlChar,
    ColumnCharAlignmentSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharType {
    Nomal,
    Delim,
    HalfSpace,
    FullSpace,
    NewLineCode,
}
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Job {
    pub job_type: JobType,
    pub job_evt: Option<JobEvent>,
    pub job_grep: Option<JobGrep>,
    pub job_watch: Option<JobWatch>,
}

impl Default for Job {
    fn default() -> Self {
        Job { job_type: JobType::Event, job_evt: None, job_grep: None, job_watch: None }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JobEvent {
    pub evt: Event,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct JobWatch {
    pub fullpath_str: String,
    pub unixtime_str: String,
}

impl Default for JobEvent {
    fn default() -> Self {
        JobEvent { evt: Event::Key(Null.into()) }
    }
}

#[derive(Debug, Hash, Default, Eq, PartialEq, Clone)]
pub struct JobGrep {
    pub grep_str: String,
    pub is_end: bool,
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum JobType {
    Event,
    GrepResult,
    Watch,
}

#[derive(Debug, Clone)]
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
/// DrawType
#[allow(non_camel_case_types)]
pub enum E_DrawRange {
    #[default]
    Init,
    TargetRange(usize, usize), // Target row only redraw
    After(usize),              // Redraw after the specified line
    All,
    Targetpoint,
    ScrollDown(usize, usize),
    ScrollUp(usize, usize),
    MoveCur,
    Not,
}

impl fmt::Display for E_DrawRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            E_DrawRange::TargetRange(_, _) => write!(f, "Target"),
            E_DrawRange::After(_) => write!(f, "After"),
            E_DrawRange::All => write!(f, "All"),
            E_DrawRange::Init => write!(f, "Init"),
            E_DrawRange::Targetpoint => write!(f, "AllDiff"),
            E_DrawRange::ScrollDown(_, _) => write!(f, "ScrollDown"),
            E_DrawRange::ScrollUp(_, _) => write!(f, "ScrollUp"),
            E_DrawRange::MoveCur => write!(f, "MoveCur"),
            E_DrawRange::Not => write!(f, "Not"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct GrepInfo {
    pub search_str: String,
    pub is_empty: bool,
    pub is_cancel: bool,
}

impl GrepInfo {
    pub fn clear(&mut self) {
        self.search_str = String::new();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mouse {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum InputCompleMode {
    None,
    WordComple,
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
            Encode::UTF16LE => &encoding_rs::UTF_16LE_INIT,
            Encode::UTF16BE => &encoding_rs::UTF_16BE_INIT,
            Encode::SJIS => &encoding_rs::SHIFT_JIS_INIT,
            Encode::JIS => &encoding_rs::ISO_2022_JP_INIT,
            Encode::EucJp => &encoding_rs::EUC_JP_INIT,
            Encode::GBK => &encoding_rs::GBK_INIT,
            _ => &encoding_rs::UTF_8_INIT,
        }
    }
    pub fn from_name(name: &str) -> Encode {
        if name == Encode::UTF16LE.to_string() {
            Encode::UTF16LE
        } else if name == Encode::UTF16BE.to_string() {
            Encode::UTF16BE
        } else if name == Encode::SJIS.to_string() {
            Encode::SJIS
        } else if name == Encode::EucJp.to_string() {
            Encode::EucJp
        } else if name == Encode::JIS.to_string() {
            Encode::JIS
        } else if name == Encode::GBK.to_string() {
            Encode::GBK
        } else {
            Encode::UTF8
        }
    }

    pub fn from_encoding(from: &encoding_rs::Encoding) -> Encode {
        if from == &encoding_rs::UTF_16LE_INIT {
            Encode::UTF16LE
        } else if from == &encoding_rs::UTF_16BE_INIT {
            Encode::UTF16BE
        } else if from == &encoding_rs::SHIFT_JIS_INIT {
            Encode::SJIS
        } else if from == &encoding_rs::EUC_JP_INIT {
            Encode::EucJp
        } else if from == &encoding_rs::ISO_2022_JP_INIT {
            Encode::JIS
        } else if from == &encoding_rs::GBK_INIT {
            Encode::GBK
        } else {
            Encode::UTF8
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
        if nl_str == NEW_LINE_CRLF_STR {
            NEW_LINE_CRLF.to_string()
        } else {
            NEW_LINE_LF.to_string()
        }
    }
}
// Cursor direction
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WindowSplitType {
    #[default]
    None,
    Vertical,
    Horizontal,
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
/// SelectRange
pub struct SelRange {
    pub mode: SelMode,
    // y 0-indexed
    pub sy: usize,
    pub ey: usize,
    // x 0-indexed (Not included row width)
    pub sx: usize,
    pub ex: usize,
    // 0-indexed
    pub s_disp_x: usize,
    pub e_disp_x: usize,
}
impl Default for SelRange {
    fn default() -> Self {
        SelRange { mode: SelMode::default(), sy: USIZE_UNDEFINED, ey: USIZE_UNDEFINED, sx: USIZE_UNDEFINED, ex: USIZE_UNDEFINED, s_disp_x: USIZE_UNDEFINED, e_disp_x: USIZE_UNDEFINED }
    }
}
impl fmt::Display for SelRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelRange sy:{}, ey:{}, sx:{}, ex:{}, s_disp_x:{}, e_disp_x:{},", self.sy, self.ey, self.sx, self.ex, self.s_disp_x, self.e_disp_x)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
pub enum SelMode {
    #[default]
    Normal,
    BoxSelect,
}

impl fmt::Display for SelMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelMode::Normal => write!(f, ""),
            SelMode::BoxSelect => write!(f, "{}", Lang::get().box_select),
        }
    }
}

// Keys without modifiers
#[derive(Debug, Copy, Default, Clone, Hash, Eq, PartialEq)]
pub enum OpenFileType {
    #[default]
    Normal,
    JsMacro,
}

#[derive(Debug, Clone)]
pub struct HeaderFile {
    pub filenm: String,
    pub filenm_disp: String,
    pub fullpath: String,
    pub is_disp: bool,
    // pub is_changed: bool,
    pub filenm_area: (usize, usize),
    pub close_area: (usize, usize),
    pub enc: Encode,
    // new line
    pub nl: String,
    pub bom: Option<Encode>,
    pub mod_time: SystemTime,
    pub watch_mode: WatchMode,
}

impl Default for HeaderFile {
    fn default() -> Self {
        HeaderFile {
            filenm: String::new(),
            filenm_disp: String::new(),
            fullpath: String::new(),
            //  ext: String::new(),
            is_disp: false,
            // is_changed: false,
            filenm_area: (0, 0),
            close_area: (0, 0),
            enc: Encode::UTF8,
            nl: NEW_LINE_LF_STR.to_string(),
            bom: None,
            mod_time: SystemTime::UNIX_EPOCH,
            watch_mode: WatchMode::default(),
        }
    }
}

impl HeaderFile {
    pub fn new(filenm: &str) -> Self {
        let path = Path::new(&filenm);
        let setting_filenm;
        let file_fullpath;

        if path.is_absolute() {
            setting_filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string();
            file_fullpath = filenm.to_string();
        } else {
            setting_filenm = filenm.to_string();
            file_fullpath = Path::new(&*CURT_DIR).join(filenm).to_string_lossy().to_string();
        }

        HeaderFile { filenm: if filenm.is_empty() { Lang::get().new_file.clone() } else { Path::new(&setting_filenm).file_name().unwrap().to_string_lossy().to_string() }, fullpath: file_fullpath, ..HeaderFile::default() }
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq)]
pub enum WatchMode {
    // Warning every time it is changed by another app
    #[default]
    Normal,
    NotMonitor,
    NotEditedWillReloadedAuto,
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

#[derive(Debug, PartialEq)]
// ActionType
pub enum ActType {
    Cancel, // Cancel process
    Nothing,
    Exit,
    Next, // Next Process
    Draw(DParts),
}

#[derive(Debug, PartialEq, Clone)]
// DrawParts
pub enum DParts {
    Editor(E_DrawRange), // and StatuusBar
    InputComple,
    Absolute(Range<usize>),
    Prompt,
    MsgBar(String),
    StatusBar,
    MenuBar,
    MenuWidget,
    FileBar,
    CtxMenu,
    All,
    ScrollUpDown(ScrollUpDownType),
    AllMsgBar(String),
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScrollUpDownType {
    Normal,
    Grep,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct TabState {
    pub prom: PromState,
    pub grep: GrepInfo,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollbarV {
    pub is_show: bool,
    pub is_enable: bool,
    // Not include　editor.row_posi
    pub row_posi: usize,
    pub row_posi_org: usize,
    pub bar_width: usize,
    pub bar_len: usize,
    pub move_len: usize,
}

impl Default for ScrollbarV {
    fn default() -> Self {
        ScrollbarV { is_show: false, is_enable: false, row_posi: USIZE_UNDEFINED, row_posi_org: 0, bar_len: 0, bar_width: 0, move_len: 0 }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GrepCancelType {
    None,
    Greping,
    Canceling,
    Canceled,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TermSize {
    pub cols: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
