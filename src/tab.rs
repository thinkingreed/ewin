use crate::{bar::msgbar::*, bar::statusbar::*, model::*, prompt::prompt::*};
use std::fmt;

impl Tab {
    pub fn new() -> Self {
        Tab {
            editor: Editor::new(),
            mbar: MsgBar::new(),
            prom: Prompt::new(),
            sbar: StatusBar::new(),
            state: TabState::default(),
        }
    }
}

impl TabState {
    pub fn clear(&mut self) {
        self.is_close_confirm = false;
        self.is_search = false;
        self.is_replace = false;
        self.is_save_new_file = false;
        self.is_move_line = false;
        self.is_open_file = false;
    }
    pub fn clear_grep_info(&mut self) {
        self.grep_info.clear();
    }
    pub fn is_not_normal(&mut self) -> bool {
        if self.is_close_confirm || self.is_search || self.is_replace || self.is_save_new_file || self.is_move_line || self.is_key_record || self.is_key_record_exec || self.is_key_record_exec_draw || self.is_read_only || self.is_open_file || self.grep_info.is_grep || self.grep_info.is_result {
            return true;
        }
        return false;
    }
    pub fn is_exists_buf(&mut self) -> bool {
        if self.is_save_new_file || self.is_search || self.is_replace || self.grep_info.is_grep || self.is_move_line || self.is_open_file {
            return true;
        }
        return false;
    }
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub editor: Editor,
    pub mbar: MsgBar,
    pub prom: Prompt,
    pub sbar: StatusBar,
    pub state: TabState,
}
#[derive(Debug, Clone)]
pub struct TabState {
    pub is_close_confirm: bool,
    pub is_search: bool,
    pub is_replace: bool,
    pub is_save_new_file: bool,
    pub is_move_line: bool,
    pub is_key_record: bool,
    pub is_key_record_exec: bool,
    pub is_key_record_exec_draw: bool,
    pub is_read_only: bool,
    pub is_open_file: bool,
    pub is_unknown_encoding: bool,
    pub grep_info: GrepInfo,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            is_close_confirm: false,
            is_search: false,
            is_replace: false,
            is_save_new_file: false,
            is_move_line: false,
            is_key_record: false,
            is_key_record_exec: false,
            is_key_record_exec_draw: false,
            is_read_only: false,
            is_open_file: false,
            is_unknown_encoding: false,
            grep_info: GrepInfo::default(),
        }
    }
}

impl fmt::Display for TabState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabState is_search:{:?},", self.is_search)
    }
}
