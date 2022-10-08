use super::term::*;
use ewin_activity_bar::activitybar::*;
use ewin_cfg::log::*;
use ewin_const::{def::*, term::*};
use ewin_help::help::*;
use ewin_key::model::*;
use ewin_menu_bar::menubar::*;
use ewin_msg_bar::msgbar::*;
use ewin_side_bar::sidebar::*;
use ewin_state::term::*;
use ewin_status_bar::statusbar::*;
use ewin_view::traits::view::*;

impl Term {
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));
        MenuBar::get().view.height = if State::get().curt_ref_state().prom == PromState::OpenFile { 0 } else { MENUBAR_HEIGHT };

        Help::get().set_size();
        self.tabs.set_size();
        MsgBar::get().set_size();
        StatusBar::get().set_size();
        SideBar::get().set_size();
        ActivityBar::get().set_size();

        return true;
    }
}
