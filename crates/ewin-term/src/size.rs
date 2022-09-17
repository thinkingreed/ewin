use super::term::*;
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
    term::*,
};
use ewin_help::help::*;
use ewin_key::model::*;
use ewin_menu_bar::menubar::*;
use ewin_msg_bar::msgbar::*;
use ewin_state::term::*;
use ewin_status_bar::statusbar::StatusBar;

impl Term {
    pub fn resize(&mut self) -> ActType {
        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));
        MenuBar::get().view.height = if State::get().curt_state().prom == PromState::OpenFile { 0 } else { MENUBAR_HEIGHT };

        Help::get().set_size();
        self.tabs.set_size();

        MsgBar::get().view.width = cols;
        MsgBar::get().view.height = MSGBAR_ROW_NUM;
        MsgBar::get().view.y = rows - STATUSBAR_ROW_NUM - 1;

        let help_height = Help::get().view.height;

        StatusBar::get().view.y = if help_height == 0 { rows - 1 } else { rows - help_height - 1 };
        StatusBar::get().view.height = STATUSBAR_ROW_NUM;
        StatusBar::get().view.width = cols;

        return true;
    }
}
