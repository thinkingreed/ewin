use crate::_cfg::lang::cfg::LangCfg;
use crossterm::event::{Event, Event::Key, KeyCode::End};
use std::path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terminal {}
impl Default for Terminal {
    fn default() -> Self {
        Terminal {}
    }
}
pub struct StatusBar {
    pub lang: LangCfg,
    pub filenm: String,
    pub filenm_str: String,
    pub filenm_str_base_w: usize,
    pub msg_str: String,
    pub msg_str_base_w: usize,
    pub cur_str: String,
    pub cur_str_base_w: usize,
    pub is_change: bool,
    pub is_save_confirm: bool,
}

impl Default for StatusBar {
    fn default() -> Self {
        StatusBar {
            lang: LangCfg::default(),
            filenm: String::new(),
            msg_str: String::new(),
            filenm_str: String::new(),
            cur_str: String::new(),
            msg_str_base_w: 0,
            filenm_str_base_w: 0,
            cur_str_base_w: 0,
            is_change: false,
            is_save_confirm: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// マウスの選択のコピー範囲
pub struct CopyRange {
    pub y: usize,
    pub sx: usize,
    pub ex: usize,
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
    pub fn is_unselected(&mut self) -> bool {
        return self.sy == 0 && self.ey == 0 && self.s_disp_x == 0 && self.e_disp_x == 0;
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

/// カーソルの位置　0-indexed
///
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
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Log {}
