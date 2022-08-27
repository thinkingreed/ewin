use super::term::*;
use ewin_cfg::log::*;
use ewin_const::{def::*, term::*};
use ewin_help::help::*;
use ewin_key::model::*;
use ewin_menulist::menubar::*;
use ewin_state::term::*;

impl Term {
    pub fn set_size_init(&mut self) {
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        MenuBar::get().set_posi(cols);
        MenuBar::get().set_menunm();
    }
    pub fn set_size(&mut self) -> bool {
        Log::debug_s("set_size");
        let (cols, rows) = get_term_size();
        Log::debug("rows, cols", &format!("{},{}", &rows, &cols));

        MenuBar::get().row_num = if State::get().curt_state().prom == PromState::OpenFile { 0 } else { MSGBAR_ROW_NUM };

        Help::get().set_size();

        self.tabs.set_size();

        return true;
    }
}
