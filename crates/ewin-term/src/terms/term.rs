use crate::{
    bar::{filebar::*, menubar::*},
    ewin_com::{
        _cfg::key::cmd::*,
        _cfg::key::{keys::*, keywhen::*},
        model::*,
    },
    tab::Tab,
};
use ewin_cfg::log::*;
use ewin_const::def::*;
use ewin_widget::model::*;
use std::usize;

impl Terminal {
    pub fn curt(&mut self) -> &mut Tab {
        return self.tabs.get_mut(self.tab_idx).unwrap();
    }

    pub fn set_keys(&mut self, keys: &Keys) {
        self.keywhen = self.get_when(keys);
        Log::debug("keywhen", &self.keywhen);
        self.keys = *keys;
        self.cmd = Cmd::keys_to_cmd(keys, Some(&self.keys_org), self.keywhen.clone());
        Log::debug("self.cmd", &self.cmd);
    }

    pub fn get_when(&mut self, keys: &Keys) -> KeyWhen {
        Log::debug("keys", &keys);

        let editor_is_dragging = self.curt().editor.state.is_dragging;

        if self.is_menuwidget_keys(keys) {
            KeyWhen::MenuBar
        } else if self.state.is_menuwidget {
            self.clear_menuwidget();
            KeyWhen::Editor
        } else if self.judge_when_filebar(keys, self.fbar.row_posi, editor_is_dragging) {
            KeyWhen::FileBar
        } else if self.curt().state.judge_when_prompt(keys) {
            KeyWhen::Prom
        } else if self.state.is_ctx_menu {
            if self.is_ctx_menu_keys(keys) {
                KeyWhen::CtxMenu
            } else {
                self.clear_ctx_menu();
                KeyWhen::Editor
            }
        } else {
            let sbar_row_posi = self.curt().sbar.row_posi;
            if self.curt().state.judge_when_statusbar(keys, sbar_row_posi, editor_is_dragging) {
                KeyWhen::StatusBar
            } else {
                KeyWhen::Editor
            }
        }
    }

    pub fn judge_when_filebar(&self, keys: &Keys, fbar_row_posi: usize, editor_is_dragging: bool) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _) if y == &(fbar_row_posi as u16) => return true,

            Keys::MouseDragLeft(y, _) if y == &(fbar_row_posi as u16) => return !editor_is_dragging,
            _ => return false,
        }
    }

    pub fn new() -> Self {
        Terminal { ..Terminal::default() }
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Terminal { draw_parts_org: DParts::All, cmd: Cmd::default(), keys: Keys::Null, keys_org: Keys::Null, keywhen: KeyWhen::All, fbar: FileBar::new(), menubar: MenuBar::new(), tabs: vec![], tab_idx: 0, state: TerminalState::default(), ctx_widget: CtxWidget::default() }
    }
}

impl Default for TerminalState {
    fn default() -> Self {
        TerminalState { is_show_init_info: false, is_all_close_confirm: false, is_all_save: false, close_other_than_this_tab_idx: USIZE_UNDEFINED, is_displayable: true, is_ctx_menu: false, is_ctx_menu_hide_draw: false, is_menuwidget: false, is_menuwidget_hide_draw: false }
    }
}

#[derive(Debug, Clone)]
pub struct Terminal {
    // pub keycmd: KeyCmd,
    pub cmd: Cmd,
    pub keys: Keys,
    pub keys_org: Keys,
    pub keywhen: KeyWhen,
    pub menubar: MenuBar,
    pub fbar: FileBar,
    // pub help: Help,
    pub tabs: Vec<Tab>,
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

/*
impl UT {

    pub fn init_ut() -> (Editor, MsgBar) {
        let mut e = Editor::default();
        e.buf = vec![vec![]];
        e.buf[0] = vec![EOF_MARK];
        e.disp_row_num = 5;
        e.set_cur_default();
        e.d_range = DRnage::default();

        let mbar = MsgBar::new();

        return (e, mbar);
    }

    pub fn insert_str(e: &mut Editor, str: &str) {
        for c in str.chars() {
            e.insert_char(c);
        }
    }
    pub fn undo_all(e: &mut Editor, mbar: &mut MsgBar) {
        let vec = e.undo_vec.clone();
        for evt_proc in vec.iter().rev() {
            Log::ep("undo_all.evt_proc.do_type", evt_proc.do_type);
            e.undo(mbar);
        }
    }
    pub fn get_buf_str(e: &mut Editor) -> String {
        let mut s = String::new();
        for vec in &e.buf {
            s.push_str(&vec.iter().collect::<String>());
        }
        return s;
    }

}
*/
