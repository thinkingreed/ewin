use crate::sidebar::*;
use ewin_activity_bar::activitybar::*;
use ewin_cfg::{log::Log, model::general::default::*};
use ewin_const::{def::*, term::*};
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for SideBar {
    fn view(&self) -> &View {
        &self.cont.as_base().view
    }

    fn set_size(&mut self) {
        self.cont.as_mut_base().view = View { y: MENUBAR_HEIGHT, x: ActivityBar::get().get_width(), width: CfgEdit::get().general.sidebar.width, height: get_term_size().1 - MENUBAR_HEIGHT - MSGBAR_HEIGHT - STATUSBAR_HEIGHT, ..View::default() };

        self.cont.set_size();

        self.cont.set_size_scrlbar_v();

        self.set_size_scrlbar_h();

        self.calc_scrlbar();
    }
}
