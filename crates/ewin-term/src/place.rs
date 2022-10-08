use ewin_activity_bar::activitybar::*;
use ewin_cfg::log::*;
use ewin_const::models::term::*;
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_key::{global::*, key::keys::*, key_traits::key_trait::*};
use ewin_menu_bar::menubar::*;
use ewin_prom::model::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;
use ewin_status_bar::statusbar::*;
use ewin_view::traits::view::*;

use crate::term::*;

impl Term {
    pub fn set_place(&mut self, keys: Keys) {
        Log::debug("keys", &keys);

        let editor_is_dragging = State::get().curt_ref_state().editor.is_dragging;
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
        } else if FileBar::judge_when_filebar(keys, editor_is_dragging) {
            Place::FileBar
        } else if Prom::judge_when_prompt() {
            Place::Prom
        } else if StatusBar::judge_when_statusbar(keys, editor_is_dragging) {
            Place::StatusBar
        } else if SideBar::judge_when_sidebar(keys, self.place_org) {
            Place::SideBar
        } else if ActivityBar::judge_when_activitybar(keys) {
            Place::ActivityBar
        } else if CMD_MAP.get().unwrap().get(&(keys, Place::Tabs)).is_some() {
            Place::Tabs
        } else {
            Place::Editor
        };
        Log::debug("place", &self.place);
    }

    pub fn set_place_mouse_move(&mut self, y: u16, x: u16) {
        Log::debug_key("Term.set_place_mouse_move");
   
        let (y, x) = (y as usize, x as usize);

        self.place_org = self.place;

        self.place = if Dialog::get().is_show {
            Place::Dialog
        } else if CtxMenu::get().is_show {
            Place::CtxMenu
        } else if MenuBar::get().is_allow_key(self.keys) {
            Place::MenuBar
        } else if Prom::judge_when_prompt() {
            Place::Prom
        } else if SideBar::get().is_range(y, x) {
            Place::SideBar
        } else if ActivityBar::get().is_range(y, x) {
            Place::ActivityBar
        } else {
            Place::Editor
        };
    }
}
