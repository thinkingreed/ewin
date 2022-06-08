use crate::{
    bar::{filebar::FileBar, menubar::MenuBar, statusbar::StatusBar},
    help::*,
};
use ewin_com::{
    _cfg::key::{keycmd::*, keys::*},
    model::*,
};
use ewin_editor::model::*;
use ewin_prom::model::Prom;
use ewin_widget::model::CtxWidget;

#[derive(Debug, Clone)]
pub struct Macros {}

#[derive(Debug, Default, Clone)]
pub struct MsgBar {
    pub msg_readonly: String,
    pub msg_keyrecord: String,
    pub msg_keyrecord_org: String,
    pub msg: Msg,
    pub msg_org: Msg,
    pub disp_readonly_row_posi: usize,
    pub disp_keyrecord_row_posi: usize,
    // 0 indexed
    pub disp_row_posi: usize,
    // 0 indexed
    pub disp_readonly_row_num: usize,
    // 0 indexed
    pub disp_keyrecord_row_num: usize,
    pub disp_row_num: usize,
    pub disp_col_num: usize,
}
impl MsgBar {
    pub fn new() -> Self {
        MsgBar { ..MsgBar::default() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Msg {
    pub str: String,
    pub msg_type: MsgType,
}

impl Default for Msg {
    fn default() -> Self {
        Msg { str: String::new(), msg_type: MsgType::Info }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgType {
    Info,
    Error,
}

/// Event action
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, Clone)]
pub struct Terminal {
    pub keycmd: KeyCmd,
    pub keys: Keys,
    pub keys_org: Keys,
    pub menubar: MenuBar,
    pub fbar: FileBar,
    pub help: Help,
    pub tabs: Vec<Tab>,
    pub editor_draw_vec: Vec<EditorDraw>,
    // tab index
    pub tab_idx: usize,
    pub state: TerminalState,
    pub ctx_widget: CtxWidget,
    pub draw_parts_org: DParts,
}

#[derive(Debug, Clone)]
pub struct TerminalState {
    pub is_show_init_info: bool,
    pub is_all_close_confirm: bool,
    pub is_all_save: bool,
    pub close_other_than_this_tab_idx: usize,
    pub is_displayable: bool,
    pub is_ctx_menu: bool,
    pub is_ctx_menu_hide_draw: bool,
    pub is_menuwidget: bool,
    pub is_menuwidget_hide_draw: bool,
}

#[derive(Debug, Default, Clone)]
pub struct HeaderBarState {
    pub is_dragging: bool,
}

impl HeaderBarState {
    pub fn clear(&mut self) {
        self.is_dragging = false;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOpenType {
    Nomal,
    First,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveType {
    Normal,
    Forced,
    NewName,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub editor: Editor,
    // pub editor_draw: Draw,
    pub msgbar: MsgBar,
    pub prom: Prom,
    pub sbar: StatusBar,
    pub state: TabState,
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}
