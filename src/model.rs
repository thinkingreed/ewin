use crate::_cfg::lang::cfg::LangCfg;
use crossterm::event::{Event, Event::Key, KeyCode::End};
use std::path;

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
#[derive(Debug, Clone)]
pub struct Process {}
pub enum EvtProcess {
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
    pub search_str: String,
    // Prompt Content
    pub cont: PromptCont,
    pub cont_sub: PromptCont,
    pub buf_posi: PromptBufPosi,
    pub is_change: bool,
    pub is_close_confirm: bool,
    pub is_save_new_file: bool,
    pub is_search: bool,
    pub is_replace: bool,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            lang: LangCfg::default(),
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            cont: PromptCont::default(),
            cont_sub: PromptCont::default(),
            buf_posi: PromptBufPosi::Main,
            search_str: String::new(),
            is_change: false,
            is_close_confirm: false,
            is_save_new_file: false,
            is_search: false,
            is_replace: false,
        }
    }
}

impl Prompt {
    pub fn new(lang_cfg: LangCfg) -> Self {
        Prompt { lang: lang_cfg, ..Prompt::default() }
    }

    pub fn clear(&mut self) {
        //  self = &mut Prompt { disp_row_num: 0, ..Prompt::default() };

        Log::ep_s("★　Prompt clear");
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.is_close_confirm = false;
        self.is_save_new_file = false;
        self.is_search = false;
        self.search_str = String::new();
        self.is_replace = false;
        self.cont = PromptCont::default();
        self.cont_sub = PromptCont::default();
        self.buf_posi = PromptBufPosi::Main;
    }
}

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub lang: LangCfg,
    pub guide: String,
    pub key_desc: String,
    pub buf_desc: String,
    pub buf: Vec<char>,
    pub cur: Cursor,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            lang: LangCfg::default(),
            guide: String::new(),
            key_desc: String::new(),
            buf_desc: String::new(),
            buf: vec![],
            cur: Cursor { y: 0, x: 0, disp_x: 1, updown_x: 0 },
        }
    }
}
#[derive(PartialEq)]
pub enum PromptBufPosi {
    Main,
    Sub,
}

#[derive(Debug, Clone)]
pub struct Terminal {}
impl Default for Terminal {
    fn default() -> Self {
        Terminal {}
    }
}
#[derive(Debug, Clone)]
pub struct StatusBar {
    pub lang: LangCfg,
    pub filenm: String,
    // 起動時ファイル未指定の場合の仮ファイル名
    pub filenm_tmp: String,
    pub filenm_disp: String,
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
            cur_str: String::new(),
            disp_row_num: 1,
            disp_row_posi: 0,
            disp_col_num: 0,
        }
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
pub struct Search {
    pub str: String,
    pub index: usize,
    pub search_ranges: Vec<SearchRange>,
}

impl Search {
    pub const INDEX_UNDEFINED: usize = usize::MAX;
    pub fn clear(&mut self) {
        self.str = String::new();
        self.index = Search::INDEX_UNDEFINED;
        self.search_ranges = vec![];
    }
}

impl Default for Search {
    fn default() -> Self {
        Search {
            str: String::new(),
            // 未設定
            index: Search::INDEX_UNDEFINED,
            search_ranges: vec![],
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
/// マウスの選択操作関連 0-indexed
pub struct SelectRange {
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

impl SelectRange {
    // 0-indexedの為に初期値を-1
    pub fn clear(&mut self) {
        // Log::ep_s("SelectRange clear");
        self.sy = 0;
        self.ey = 0;
        self.sx = 0;
        self.ex = 0;
        self.s_disp_x = 0;
        self.e_disp_x = 0;
    }
    pub fn is_selected(&mut self) -> bool {
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
        SelectRange {
            sy: sy,
            ey: ey,
            sx: sx,
            ex: ex,
            s_disp_x: s_disp_x,
            e_disp_x: e_disp_x,
        }
    }
}

/// カーソル位置 　0-indexed
/// Editor,Prompt
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cursor {
    // Editor.bufferの[y]
    pub y: usize,
    // Editor.bufferの[y][x] + line_num_width
    pub x: usize,
    // カーソルの表示位置(２バイト文字対応) line_num_width + 1以上
    pub disp_x: usize,
    // 連続でカーソル上下時の基本x位置(２バイト文字対応)  line_num_width + 1以上 初期値:0
    pub updown_x: usize,
}
// エディタの内部状態
#[derive(Debug, Clone)]
pub struct Editor {
    /// テキスト本体
    /// buffer[i][j]はi行目のj列目の文字 0-indexed
    pub buf: Vec<Vec<char>>,
    /// 現在のカーソルの位置
    /// self.cursor.y < self.buffer.len()
    /// self.cursor.x <= self.buffer[self.cursor.y].len() + self.x_default_posi
    /// を常に保証する
    pub cur: Cursor,
    /// 画面の一番上はバッファの何行目か
    /// スクロール処理に使う
    pub y_offset: usize,
    pub x_offset: usize,
    // 表示幅単位
    pub x_offset_disp: usize,
    // x_offset対象の行
    pub x_offset_y: usize,
    pub path: Option<path::PathBuf>,
    /// 行番号の列数 line_num_width
    pub lnw: usize,
    pub sel: SelectRange,
    pub curt_evt: Event,
    pub is_all_redraw: bool,
    pub clipboard: String,
    /// ターミナル上の表示数
    pub disp_row_num: usize,
    pub disp_col_num: usize,
    pub search: Search,
    //pub str_vec: Vec<String>,
    // edit_ranges
    pub e_ranges: Vec<EditRnage>,
}

impl Editor {}
impl Default for Editor {
    fn default() -> Self {
        Editor {
            buf: vec![Vec::new()],
            cur: Cursor { y: 0, x: 0, disp_x: 0, updown_x: 0 },
            y_offset: 0,
            x_offset: 0,
            x_offset_disp: 0,
            x_offset_y: 0,
            path: None,
            lnw: 0,
            sel: SelectRange {
                sy: 0,
                ey: 0,
                sx: 0,
                ex: 0,
                s_disp_x: 0,
                e_disp_x: 0,
            },
            curt_evt: Key(End.into()),
            is_all_redraw: false,
            clipboard: String::new(),
            disp_row_num: 0,
            disp_col_num: 0,
            search: Search::default(),
            // str_vec: vec![],
            e_ranges: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditRnage {
    pub y: usize,
    // pub vec: Vec<char>,
    pub e_type: EType,
}
#[derive(Debug, Clone, PartialEq, Eq)]
/// EditType
pub enum EType {
    Add,
    Mod,
    Del,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Log {
    pub log_path: String,
}

impl Log {}
