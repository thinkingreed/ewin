extern crate ropey;
use crate::{def::*, editor::draw::char_style::*};
use chrono::NaiveDateTime;
use crossterm::event::{Event, Event::Key, KeyCode::End};
use ropey::Rope;
use std::cmp::{max, min};
use std::collections::VecDeque;
use std::fmt;
use std::path;
use syntect::highlighting::HighlightState;
use syntect::parsing::{ParseState, ScopeStackOp};

#[derive(Debug, Clone)]
pub struct MsgBar {
    pub msg_readonly: String,
    pub msg_keyrecord: String,
    pub msg: String,
    pub msg_org: String,
    /// ターミナル上の表示数
    pub disp_readonly_row_posi: usize,
    pub disp_keyrecord_row_posi: usize,
    pub disp_row_posi: usize,
    pub disp_readonly_row_num: usize,
    pub disp_keyrecord_row_num: usize,
    pub disp_row_num: usize,
    pub disp_col_num: usize,
}

impl Default for MsgBar {
    fn default() -> Self {
        MsgBar {
            msg_readonly: String::new(),
            msg_keyrecord: String::new(),
            msg: String::new(),
            msg_org: String::new(),
            disp_readonly_row_posi: 0,
            disp_keyrecord_row_posi: 0,
            disp_row_posi: 0,
            disp_readonly_row_num: 0,
            disp_keyrecord_row_num: 0,
            disp_row_num: 0,
            disp_col_num: 0,
        }
    }
}
/// Event後のEditor以外の操作
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, PartialEq)]
pub enum EvtActType {
    Hold,
    Next,
    Exit,
}
pub struct Prompt {
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub buf_posi: PromptBufPosi,
    pub tab_comp: TabComp,
    // cache
    pub cache_search_filenm: String,
    pub cache_search_folder: String,
    // fn clear not clear
    pub is_change: bool,
    pub is_grep_result: bool,
    pub is_grep_result_cancel: bool,
    // *************
    pub is_close_confirm: bool,
    pub is_save_new_file: bool,
    pub is_search: bool,
    pub is_replace: bool,
    pub is_grep: bool,
    // grep result stdout/stderr output complete flg
    pub is_grep_stdout: bool,
    pub is_grep_stderr: bool,

    pub is_key_record: bool,
    pub is_key_record_exec: bool,
    pub is_key_record_exec_draw: bool,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            is_change: false,
            is_grep_result: false,
            is_grep_result_cancel: false,
            cont_1: PromptCont::default(),
            cont_2: PromptCont::default(),
            cont_3: PromptCont::default(),
            buf_posi: PromptBufPosi::First,
            tab_comp: TabComp::default(),
            cache_search_filenm: String::new(),
            cache_search_folder: String::new(),
            is_close_confirm: false,
            is_save_new_file: false,
            is_search: false,
            is_replace: false,
            is_grep: false,
            is_grep_stdout: false,
            is_grep_stderr: false,
            is_key_record: false,
            is_key_record_exec: false,
            is_key_record_exec_draw: false,
        }
    }
}

impl Prompt {
    pub fn new() -> Self {
        Prompt { ..Prompt::default() }
    }

    pub fn clear(&mut self) {
        //  self = &mut Prompt { disp_row_num: 0, ..Prompt::default() };
        Log::ep_s("　　　　　　　　Prompt clear");
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.cont_1 = PromptCont::default();
        self.cont_2 = PromptCont::default();
        self.cont_3 = PromptCont::default();
        self.buf_posi = PromptBufPosi::First;
        self.is_close_confirm = false;
        self.is_save_new_file = false;
        self.is_search = false;
        self.is_replace = false;
        self.is_grep = false;
        self.is_grep_stdout = false;
        self.is_grep_stderr = false;
    }
}

#[derive(Debug, Clone)]
pub struct TabComp {
    // 補完候補一覧
    pub dirs: Vec<String>,
    // 補完候補一覧index
    pub index: usize,
}
impl TabComp {}
impl Default for TabComp {
    fn default() -> Self {
        TabComp { index: USIZE_UNDEFINED, dirs: vec![] }
    }
}
impl fmt::Display for TabComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabComp index:{}, dirs:{},", self.index, self.dirs.join(" "),)
    }
}

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub guide: String,
    pub key_desc: String,
    pub buf_desc: String,
    pub buf: Vec<char>,
    pub cur: Cur,
    pub updown_x: usize,
    pub sel: SelRange,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            guide: String::new(),
            key_desc: String::new(),
            buf_desc: String::new(),
            buf: vec![],
            cur: Cur::default(),
            updown_x: 0,
            sel: SelRange::default(),
        }
    }
}
#[derive(PartialEq)]
pub enum PromptBufPosi {
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq)]
pub enum Env {
    WSL,
    Linux,
    Windows,
}
#[derive(Debug)]
pub struct Terminal {
    // pub env: Env,
}
#[derive(Debug, Clone)]
pub struct StatusBar {
    pub filenm: String,
    // 起動時ファイル未指定の場合の仮ファイル名
    pub filenm_tmp: String,
    pub filenm_disp: String,
    pub filenm_disp_flg: bool,
    pub cur_str: String,
    /// ターミナル上の表示数
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            filenm: String::new(),
            filenm_tmp: String::new(),
            filenm_disp: String::new(),
            filenm_disp_flg: false,
            cur_str: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
    }
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
/// 検索範囲
pub struct Search {
    pub str: String,
    pub index: usize,
    pub ranges: Vec<SearchRange>,
    pub file: String,
    pub filenm: String,
    pub folder: String,
    pub row_num: String,
}

impl Search {
    pub fn clear(&mut self) {
        self.str = String::new();
        self.index = USIZE_UNDEFINED;
        self.ranges = vec![];
        // file full path
        self.file = String::new();
        self.filenm = String::new();
        self.folder = String::new();
    }
}
impl Default for Search {
    fn default() -> Self {
        Search {
            str: String::new(),
            index: USIZE_UNDEFINED,
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
    // 0-indexedの為に初期値を-1
    pub fn clear(&mut self) {
        //  Log::ep_s("SelRange.clear");

        self.sy = 0;
        self.ey = 0;
        self.sx = 0;
        self.ex = 0;
        self.s_disp_x = 0;
        self.e_disp_x = 0;
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
    pub fn set_sel_posi(&mut self, is_s: bool, y: usize, x: usize, disp_x: usize) {
        if is_s {
            if !self.is_selected() {
                self.set_s(y, x, disp_x);
            }
        } else {
            self.set_e(y, x, disp_x);
        }
    }
}

impl fmt::Display for SelRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelRange sy:{}, ey:{}, sx:{}, ex:{}, s_disp_x:{}, e_disp_x:{},", self.sy, self.ey, self.sx, self.ex, self.s_disp_x, self.e_disp_x)
    }
}

/// Cursor 　0-indexed
/// Editor,Prompt
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
    /// self.cursor.y < self.buffer.len()
    /// self.cursor.x <= self.buffer[self.cursor.y].len() + self.lnw
    pub cur: Cur,
    /// 画面の一番上はバッファの何行目か
    /// スクロール処理に使う
    pub offset_y: usize,
    pub offset_x: usize,
    // 表示幅単位
    pub offset_disp_x: usize,
    // 元の行のx_offset_di行の算出用
    pub cur_y_org: usize,
    // x_offset対象の行
    // pub x_offset_y: usize,
    // 連続でカーソル上下時の基本x位置(２バイト文字対応)  line_num_width + 1以上 初期値:0
    pub updown_x: usize,
    pub path: Option<path::PathBuf>,
    pub path_str: String,
    pub ext: String,
    // row_number_width
    pub rnw: usize,
    pub sel: SelRange,
    pub evt: Event,
    pub clipboard: String,
    /// number displayed on the terminal
    pub disp_row_num: usize,
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
            // x_offset_disp_org: 0,
            //    x_offset_y: 0,
            updown_x: 0,
            path: None,
            path_str: String::new(),
            ext: String::new(),
            rnw: 0,
            sel: SelRange::default(),
            evt: Key(End.into()),
            clipboard: String::new(),
            // for UT set
            disp_row_num: 5,
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
pub struct Region {
    pub from: CharStyle,
    pub to: CharStyle,
    pub c: char,
}

impl fmt::Display for Region {
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
    pub regions: Vec<Vec<Region>>,
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
            regions: vec![],
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
pub struct Log {
    pub log_path: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UT {}
