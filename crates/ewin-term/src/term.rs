use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*, term::Place};
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_key::{
    global::*,
    key::{cmd::*, keys::*},
    key_traits::key_trait::*,
    model::*,
};
use ewin_menulist::menubar::*;
use ewin_state::term::*;
use ewin_tabs::tabs::*;

impl Term {
    pub fn set_keys(&mut self, keys: &Keys) -> ActType {
        Log::debug_key("Term.set_keys");
        Log::debug("keys", &keys);
        Log::debug("keywhen", &self.place);
        self.keys = *keys;
        self.cmd = Cmd::keys_to_cmd(keys, Some(&self.keys_org), self.place);
        Log::debug("self.cmd", &self.cmd);

        if self.cmd.cmd_type == CmdType::Unsupported {
            return ActType::Draw(DParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        return ActType::Next;
    }

    pub fn set_when(&mut self, keys: Keys) {
        Log::debug("keys", &keys);

        let editor_is_dragging = State::get().curt_state().editor.is_dragging;
        // pre-processing
        CtxMenu::get().is_check_clear(keys);
        MenuBar::get().is_check_clear(keys);

        self.place_org = self.place;

        self.place = if Dialog::get().is_show {
            Place::Dialog
        } else if CtxMenu::get().is_show {
            Place::CtxMenu
        } else if MenuBar::get().is_allow_key(keys) {
            Place::MenuBar
        } else if self.judge_when_filebar(keys, editor_is_dragging) {
            Place::FileBar
        } else if self.judge_when_prompt(keys) {
            Place::Prom
        } else if self.judge_when_statusbar(keys, editor_is_dragging) {
            Place::StatusBar
        } else if CMD_MAP.get().unwrap().get(&(keys, Place::Tabs)).is_some() {
            Place::Tabs
        } else {
            Place::Editor
        };
    }

    pub fn judge_when_filebar(&self, keys: Keys, editor_is_dragging: bool) -> bool {
        match keys {
            Keys::MouseDownLeft(y, _) | Keys::MouseDownRight(y, _) if y == FileBar::get().row_posi as u16 => return true,
            Keys::MouseDragLeft(y, _) if y == FileBar::get().row_posi as u16 => return !editor_is_dragging,
            _ => return false,
        }
    }
    pub fn judge_when_statusbar(&mut self, keys: Keys, editor_is_dragging: bool) -> bool {
        match &keys {
            Keys::MouseDownLeft(y, _) if y == &(self.tabs.curt().sbar.row_posi as u16) => return true,
            Keys::MouseDragLeft(y, _) if y == &(self.tabs.curt().sbar.row_posi as u16) => return !editor_is_dragging,
            _ => return false,
        }
    }
    pub fn judge_when_prompt(&self, keys: Keys) -> bool {
        Log::debug_key("judge_when_prompt");

        let is_nomal = State::get().curt_state().is_nomal();
        if !is_nomal || (State::get().curt_state().prom == PromState::GrepResult && keys == Cmd::cmd_to_keys(CmdType::Confirm)) {
            return true;
        }
        return false;
    }

    pub fn new() -> Self {
        Term { ..Term::default() }
    }
}

impl Default for Term {
    fn default() -> Self {
        Term { cmd: Cmd::default(), keys: Keys::Null, keys_org: Keys::Null, place: Place::Tabs, place_org: Place::Tabs, tabs: Tabs::default(), state: State::default() }
    }
}

#[derive(Debug, Clone)]
// Terminal
pub struct Term {
    pub cmd: Cmd,
    pub keys: Keys,
    pub keys_org: Keys,
    pub place: Place,
    pub place_org: Place,
    pub tabs: Tabs,
    pub state: State,
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
