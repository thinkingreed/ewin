use crate::{bar::msgbar::*, bar::statusbar::*, model::*, prompt::prompt::prompt::*};
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
        self.is_move_row = false;
        self.is_open_file = false;
        self.is_enc_nl = false;
        self.is_menu = false;
    }

    pub fn clear_grep_info(&mut self) {
        self.grep_state.clear();
    }

    pub fn is_nomal(&self) -> bool {
        if self.is_close_confirm || self.is_search || self.is_replace || self.is_save_new_file || self.is_move_row || self.is_read_only || self.is_open_file || self.grep_state.is_grep || self.grep_state.is_result || self.is_enc_nl || self.is_menu {
            return false;
        }
        return true;
    }

    pub fn is_editor_cur(&self) -> bool {
        if self.is_close_confirm || self.is_search || self.is_replace || self.is_save_new_file || self.is_move_row || self.is_open_file || self.grep_state.is_grep || self.is_enc_nl || self.is_menu {
            return false;
        }
        return true;
    }

    pub fn is_prom_show_cur(&self) -> bool {
        if self.is_exists_input_field() || self.is_exists_choice() {
            return true;
        }
        return false;
    }

    pub fn is_exists_input_field(&self) -> bool {
        if self.is_save_new_file || self.is_search || self.is_replace || self.grep_state.is_grep || self.is_move_row || self.is_open_file {
            return true;
        }
        return false;
    }

    pub fn is_exists_choice(&self) -> bool {
        if self.is_enc_nl || self.is_menu {
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
    pub is_move_row: bool,
    //  pub is_key_record: bool,
    pub is_read_only: bool,
    pub is_open_file: bool,
    pub is_enc_nl: bool,
    pub grep_state: GrepState,
    pub key_record_state: KeyRecordState,
    pub is_menu: bool,
}

impl Default for TabState {
    fn default() -> Self {
        TabState {
            is_close_confirm: false,
            is_search: false,
            is_replace: false,
            is_save_new_file: false,
            is_move_row: false,
            is_read_only: false,
            is_open_file: false,
            is_enc_nl: false,
            key_record_state: KeyRecordState::default(),
            grep_state: GrepState::default(),
            is_menu: false,
        }
    }
}

impl fmt::Display for TabState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabState is_search:{:?},", self.is_search)
    }
}
