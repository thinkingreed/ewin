use crate::_cfg::lang::cfg::LangCfg;
use crossterm::event::{Event, Event::Key, KeyCode::End};
use std::path;

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Debug, Clone)]
pub struct MsgBar {
    pub lang: LangCfg,
    pub msg_disp: String,
    /// ターミナル上の表示数
    pub disp_row_posi: usize,
    pub disp_row_num: usize,
    pub disp_col_num: usize,
}

impl Default for MsgBar {
    fn default() -> Self {
        MsgBar {
            lang: LangCfg::default(),
            msg_disp: String::new(),
            disp_row_posi: 0,
            disp_row_num: 0,
            disp_col_num: 0,
        }
    }
}
/// Event後のEditor以外の操作
#[derive(Debug, Clone)]
pub struct EvtAct {}
pub enum EvtActType {
    Hold,
    Next,
    Exit,
}
pub struct Prompt {
    pub lang: LangCfg,
    /// ターミナル上の表示関連
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub buf_posi: PromptBufPosi,
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
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            lang: LangCfg::default(),
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
            cache_search_filenm: String::new(),
            cache_search_folder: String::new(),
            is_close_confirm: false,
            is_save_new_file: false,
            is_search: false,
            is_replace: false,
            is_grep: false,
            is_grep_stdout: false,
            is_grep_stderr: false,
        }
    }
}

impl Prompt {
    pub fn new(lang_cfg: LangCfg) -> Self {
        Prompt { lang: lang_cfg, ..Prompt::default() }
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
pub struct PromptCont {
    pub lang: LangCfg,
    pub guide: String,
    pub key_desc: String,
    pub buf_desc: String,
    pub buf: Vec<char>,
    pub cur: Cur,
    pub updown_x: usize,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            lang: LangCfg::default(),
            guide: String::new(),
            key_desc: String::new(),
            buf_desc: String::new(),
            buf: vec![],
            cur: Cur::default(),
            updown_x: 0,
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
}

#[derive(Debug)]
pub struct Terminal {
    pub env: Env,
}
impl Default for Terminal {
    fn default() -> Self {
        Terminal { env: Env::Linux }
    }
}
#[derive(Debug, Clone)]
pub struct StatusBar {
    pub lang: LangCfg,
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
            lang: LangCfg::default(),
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
    pub do_type: DoType,
    // lnwを含まない
    pub cur_s: Cur,
    pub cur_e: Cur,
    pub str_vec: Vec<String>,
    pub sel: SelRange,
    pub d_range: DRnage,
}

impl Default for EvtProc {
    fn default() -> Self {
        EvtProc {
            cur_s: Cur::default(),
            cur_e: Cur::default(),
            str_vec: vec![],
            do_type: DoType::None,
            sel: SelRange::default(),
            d_range: DRnage::default(),
        }
    }
}
impl EvtProc {
    pub fn new(do_type: DoType, editor: &Editor) -> Self {
        return EvtProc {
            do_type: do_type,
            cur_s: Cur {
                y: editor.cur.y,
                x: editor.cur.x,
                disp_x: editor.cur.disp_x,
            },
            d_range: editor.d_range,
            ..EvtProc::default()
        };
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// コピー範囲
pub struct CopyRange {
    pub y: usize,
    pub sx: usize,
    pub ex: usize,
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
#[derive(Debug, Clone, PartialEq, Eq)]
/// 検索範囲
pub struct Search {
    pub str: String,
    pub index: usize,
    pub search_ranges: Vec<SearchRange>,
    pub file: String,
    pub filenm: String,
    pub folder: String,
    pub row_num: String,
}

impl Search {
    pub const INDEX_UNDEFINED: usize = usize::MAX;
    pub fn clear(&mut self) {
        self.str = String::new();
        self.index = Search::INDEX_UNDEFINED;
        self.search_ranges = vec![];
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
            // 未設定
            index: Search::INDEX_UNDEFINED,
            search_ranges: vec![],
            file: String::new(),
            filenm: String::new(),
            folder: String::new(),
            row_num: String::new(),
        }
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
    pub fn get_range(&mut self) -> Self {
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
}

/// Cursor 　0-indexed
/// Editor,Prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cur {
    // Editor.bufferの[y]
    pub y: usize,
    // Editor.bufferの[y][x] + line_num_width
    pub x: usize,
    // カis_redraw文字対応) line_num_width + 1以上
    pub disp_x: usize,
}
impl Default for Cur {
    fn default() -> Self {
        Cur { y: 0, x: 0, disp_x: 1 }
    }
}
// エディタの内部状態
#[derive(Debug, Clone)]
pub struct Editor {
    /// テキスト本体
    /// buffer[i][j]はi行目のj列目の文字 0-indexed
    pub buf: Vec<Vec<char>>,
    /// 現在のカーソルの位置
    /// self.cursor.y < self.buffer.len()
    /// self.cursor.x <= self.buffer[self.cursor.y].len() + self.lnw
    /// を常に保証する
    pub cur: Cur,
    /// 画面の一番上はバッファの何行目か
    /// スクロール処理に使う
    pub y_offset: usize,
    pub x_offset: usize,
    // 表示幅単位
    pub x_offset_disp: usize,
    // 元の行のx_offset_di行の算出用
    pub cur_y_org: usize,
    // x_offset対象の行
    // pub x_offset_y: usize,
    // 連続でカーソル上下時の基本x位置(２バイト文字対応)  line_num_width + 1以上 初期値:0
    pub updown_x: usize,
    pub path: Option<path::PathBuf>,
    /// 行番号の列数 row_num_width
    pub rnw: usize,
    pub sel: SelRange,
    pub curt_evt: Event,
    pub is_redraw: bool,
    pub is_undo: bool,
    pub clipboard: String,
    /// ターミナル上の表示数
    pub disp_row_num: usize,
    pub disp_col_num: usize,
    pub search: Search,
    // draw_ranges
    pub d_range: DRnage,
    pub undo_vec: Vec<EvtProc>,
    pub redo_vec: Vec<EvtProc>,
    pub grep_result_vec: Vec<GrepResult>,
}

impl Default for Editor {
    fn default() -> Self {
        Editor {
            buf: vec![vec![]],
            cur: Cur::default(),
            y_offset: 0,
            x_offset: 0,
            x_offset_disp: 0,
            cur_y_org: 0,
            // x_offset_disp_org: 0,
            //    x_offset_y: 0,
            updown_x: 0,
            path: None,
            rnw: 0,
            sel: SelRange::default(),
            curt_evt: Key(End.into()),
            is_redraw: false,
            is_undo: false,
            clipboard: String::new(),
            disp_row_num: 0,
            disp_col_num: 0,
            search: Search::default(),
            // str_vec: vec![],
            d_range: DRnage::default(),
            undo_vec: vec![],
            redo_vec: vec![],
            grep_result_vec: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// DrawRnage
pub struct DRnage {
    pub sy: usize,
    pub ey: usize,
    pub d_type: DType,
}

impl Default for DRnage {
    fn default() -> Self {
        DRnage { sy: 0, ey: 0, d_type: DType::All }
    }
}

impl DRnage {
    pub fn new(sy: usize, ey: usize, d_type: DType) -> Self {
        return DRnage { sy: sy, ey: ey, d_type: d_type };
    }

    pub fn get_range(&mut self) -> Self {
        let mut sy = self.sy;
        let mut ey = self.ey;

        if self.sy > self.ey {
            sy = self.ey;
            ey = self.sy;
        }
        return DRnage { sy: sy, ey: ey, d_type: self.d_type };
    }
    pub fn clear(&mut self) {
        self.sy = 0;
        self.ey = 0;
        self.d_type = DType::None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// DrawType
pub enum DType {
    Target, // Target row only redraw
    After,  // Redraw after the specified line
    None,
    All,
    Not,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// UnDo ReDo Type
pub enum DoType {
    Del,
    Enter,
    BS,
    Cut,
    Paste,
    InsertChar,
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub log_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Colors {}
