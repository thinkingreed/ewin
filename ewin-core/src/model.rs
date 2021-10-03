use crate::{
    _cfg::key::{keycmd::*, keys::*},
    char_style::*,
    def::*,
    global::*,
};
use chrono::NaiveDateTime;
use clap::ArgMatches;
use crossterm::event::{Event, KeyCode::Null};
use encoding_rs::Encoding;
use serde::Deserialize;
#[cfg(target_os = "linux")]
use std::usize;
use std::{
    cmp::{max, min},
    collections::VecDeque,
    ffi::OsStr,
    fmt,
    path::Path,
};
use syntect::highlighting::HighlightState;
use syntect::parsing::{ParseState, ScopeStackOp};

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
    pub e_cmd: E_Cmd,
    pub p_cmd: P_Cmd,
    // not include lnw
    pub cur_s: Cur,
    pub cur_e: Cur,
    pub str: String,
    pub box_sel_vec: Vec<(SelRange, String)>,
    pub box_sel_redo_vec: Vec<(SelRange, String)>,
    pub sel: SelRange,
    pub draw_type: EditorDrawRange,
}
impl Default for Proc {
    fn default() -> Self {
        Proc { p_cmd: P_Cmd::Null, cur_s: Cur::default(), cur_e: Cur::default(), str: String::new(), e_cmd: E_Cmd::Null, sel: SelRange::default(), draw_type: EditorDrawRange::default(), box_sel_vec: vec![], box_sel_redo_vec: vec![] }
    }
}
impl fmt::Display for Proc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EvtProc cur_s:{}, cur_e:{}, str:{}, e_cmd:{:?}, p_cmd:{:?}, sel:{}, d_range:{}", self.cur_s, self.cur_e, self.str, self.e_cmd, self.p_cmd, self.sel, self.draw_type)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// All edit history including undo and redo
/// History
pub struct History {
    pub mouse_click_vec: VecDeque<(NaiveDateTime, Keys)>,
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
        return GrepResult { filenm, row_num };
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
pub struct KeyMacro {
    pub keys: Keys,
    pub search: Search,
}

impl Default for KeyMacro {
    fn default() -> Self {
        KeyMacro { keys: Keys::Null, search: Search::default() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

impl KeyMacroState {
    pub fn is_running(&self) -> bool {
        return self.is_exec == true && self.is_exec_end == false;
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
pub struct SyntaxState {
    pub highlight_state: HighlightState,
    pub ops: Vec<(usize, ScopeStackOp)>,
    pub parse_state: ParseState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// DrawType
pub enum EditorDrawRange {
    Target(usize, usize), // Target row only redraw
    After(usize),         // Redraw after the specified line
    None,                 // First time
    All,
    ScrollDown(usize, usize),
    ScrollUp(usize, usize),
    MoveCur,
    Not,
}

impl Default for EditorDrawRange {
    fn default() -> Self {
        EditorDrawRange::None
    }
}

impl EditorDrawRange {
    pub fn get_type(sel_mode: SelMode, sy: usize, ey: usize) -> EditorDrawRange {
        match sel_mode {
            SelMode::Normal => {
                return EditorDrawRange::Target(min(sy, ey), max(sy, ey));
            }
            SelMode::BoxSelect => {
                return EditorDrawRange::All;
            }
        }
    }
}

impl fmt::Display for EditorDrawRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EditorDrawRange::Target(_, _) => write!(f, "Target"),
            EditorDrawRange::After(_) => write!(f, "After"),
            EditorDrawRange::None => write!(f, "None"),
            EditorDrawRange::All => write!(f, "All"),
            EditorDrawRange::ScrollDown(_, _) => write!(f, "ScrollDown"),
            EditorDrawRange::ScrollUp(_, _) => write!(f, "ScrollUp"),
            EditorDrawRange::MoveCur => write!(f, "MoveCur"),
            EditorDrawRange::Not => write!(f, "Not"),
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
    pub fn is_greping(&self) -> bool {
        return self.is_result && !(self.is_stdout_end && self.is_stderr_end) && !self.is_cancel;
    }
    pub fn is_grep_finished(&self) -> bool {
        return self.is_result && ((self.is_stdout_end && self.is_stderr_end) || self.is_cancel);
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
        if nl_str == NEW_LINE_CRLF_STR {
            return NEW_LINE_CRLF.to_string();
        } else {
            return NEW_LINE_LF.to_string();
        }
    }
}
// Cursor direction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn keycmd_to_curdirection(keycmd: &KeyCmd) -> Direction {
        return match keycmd {
            KeyCmd::Prom(P_Cmd::CursorLeft) => Direction::Left,
            KeyCmd::Prom(P_Cmd::CursorRight) => Direction::Right,
            KeyCmd::Prom(P_Cmd::CursorUp) => Direction::Up,
            KeyCmd::Prom(P_Cmd::CursorDown) => Direction::Down,
            _ => unreachable!(),
        };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MacrosFunc {
    insertString,
    getSelectedString,
    getAllString,
    searchAll,
}

impl fmt::Display for MacrosFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacrosFunc::insertString => write!(f, "insertString"),
            MacrosFunc::getSelectedString => write!(f, "getSelectedString"),
            MacrosFunc::getAllString => write!(f, "getAllString"),
            MacrosFunc::searchAll => write!(f, "searchAll"),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum SelMode {
    Normal,
    BoxSelect,
}
impl Default for SelMode {
    fn default() -> Self {
        SelMode::Normal
    }
}
impl fmt::Display for SelMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelMode::Normal => write!(f, ""),
            SelMode::BoxSelect => write!(f, "{}", LANG.box_select),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub filenm: String,
    pub out_config_flg: bool,
}

impl Default for Args {
    fn default() -> Self {
        Args { filenm: String::new(), out_config_flg: false }
    }
}
impl Args {
    pub fn new(matches: &ArgMatches) -> Self {
        let file_path: String = matches.value_of_os("file").unwrap_or(OsStr::new("")).to_string_lossy().to_string();
        Args { filenm: file_path, out_config_flg: if matches.is_present("output-config") { true } else { false } }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum OpenFileInitValue {
    None,
    CurtDir,
}

impl Default for OpenFileInitValue {
    fn default() -> Self {
        OpenFileInitValue::CurtDir
    }
}

// Keys without modifiers
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum OpenFileType {
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
    pub nl_org: String,
    pub bom: Option<Encode>,
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
            nl_org: NEW_LINE_LF_STR.to_string(),
            bom: None,
        }
    }
}

impl HeaderFile {
    pub fn new(filenm: &str) -> Self {
        let path = Path::new(&filenm);
        let setting_filenm;
        let file_fullpath;

        if path.is_absolute() {
            setting_filenm = Path::new(&filenm).file_name().unwrap().to_string_lossy().to_string().clone();
            file_fullpath = filenm.to_string();
        } else {
            setting_filenm = filenm.to_string();
            file_fullpath = Path::new(&*CURT_DIR).join(filenm).to_string_lossy().to_string();
        }

        return HeaderFile { filenm: if filenm.is_empty() { LANG.new_file.clone() } else { Path::new(&setting_filenm).file_name().unwrap().to_string_lossy().to_string().clone() }, fullpath: file_fullpath.to_string(), ..HeaderFile::default() };
    }
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
            BoxInsertMode::Insert => write!(f, "{}", LANG.box_insert),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// FormatType
pub enum FmtType {
    JSON,
    XML,
    HTML,
}

impl fmt::Display for FmtType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FmtType::JSON => write!(f, "JSON"),
            FmtType::XML => write!(f, "XML"),
            FmtType::HTML => write!(f, "HTML"),
        }
    }
}
impl FmtType {
    pub fn from_str(s: &str) -> FmtType {
        if s == LANG.json {
            return FmtType::JSON;
        } else if s == LANG.xml {
            return FmtType::XML;
        } else {
            return FmtType::HTML;
        }
    }
}

#[derive(Debug, PartialEq)]
// ActionType
pub enum ActType {
    Cancel, // Cancel process
    Exit,
    Next, // Next Process
    Draw(DParts),
}

#[derive(Debug, PartialEq, Clone)]
// DrawParts
pub enum DParts {
    Editor, // and StatuusBar
    Prompt,
    MsgBar(String),
    CtxMenu,
    All,
    ScrollUpDown(ScrollUpDownType),
    AllMsgBar(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ScrollUpDownType {
    Normal,
    Grep,
}

#[derive(Debug, Clone)]
pub struct TabState {
    pub is_close_confirm: bool,
    pub is_search: bool,
    pub is_replace: bool,
    pub is_save_new_file: bool,
    pub is_move_row: bool,
    //  pub is_key_record: bool,
    pub is_open_file: bool,
    pub is_enc_nl: bool,
    pub grep: GrepState,
    pub is_menu: bool,
}

impl Default for TabState {
    fn default() -> Self {
        TabState { is_close_confirm: false, is_search: false, is_replace: false, is_save_new_file: false, is_move_row: false, is_open_file: false, is_enc_nl: false, grep: GrepState::default(), is_menu: false }
    }
}

impl fmt::Display for TabState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabState is_search:{:?},", self.is_search)
    }
}

impl TabState {
    pub fn clear(&mut self) {
        self.is_close_confirm = false;
        self.is_search = false;
        self.is_replace = false;
        self.is_save_new_file = false;
        self.is_move_row = false;
        self.is_open_file = false;
        self.is_enc_nl = false;
        self.is_menu = false;
    }

    pub fn is_nomal(&self) -> bool {
        if self.is_close_confirm || self.is_search || self.is_replace || self.is_save_new_file || self.is_move_row || self.is_open_file || self.is_enc_nl || self.is_menu
        // grep, grep result 
        || self.grep.is_grep  || self.grep.is_greping()
        {
            return false;
        }
        return true;
    }
    pub fn is_nomal_and_not_result(&self) -> bool {
        if !self.is_nomal() || self.grep.is_result {
            return false;
        }
        return true;
    }

    pub fn judge_when(&self, keys: &Keys) -> bool {
        if !self.is_nomal() || (self.grep.is_grep_finished() && keys == &Keys::Raw(Key::Enter)) {
            return false;
        }
        true
    }

    pub fn is_editor_cur(&self) -> bool {
        if self.is_close_confirm || self.is_search || self.is_replace || self.is_save_new_file || self.is_move_row || self.is_open_file || self.grep.is_grep || self.grep.is_greping() || self.is_enc_nl || self.is_menu {
            return false;
        }
        true
    }

    pub fn is_prom_show_cur(&self) -> bool {
        if self.is_exists_input_field() || self.is_exists_choice() {
            return true;
        }
        false
    }

    pub fn is_exists_input_field(&self) -> bool {
        if self.is_save_new_file || self.is_search || self.is_replace || self.grep.is_grep || self.is_move_row || self.is_open_file {
            return true;
        }
        false
    }
    pub fn is_exists_input_field_not_open_file(&self) -> bool {
        if !self.is_open_file && self.is_exists_input_field() {
            return true;
        }
        false
    }

    pub fn is_exists_choice(&self) -> bool {
        if self.is_enc_nl || self.is_menu {
            return true;
        }
        false
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MsgType {
    Info,
    Err,
    KeyRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
