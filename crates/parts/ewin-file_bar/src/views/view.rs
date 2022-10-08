use crate::filebar::*;
use ewin_activity_bar::activitybar::*;
use ewin_const::term::*;
use ewin_side_bar::sidebar::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for FileBar {
    fn view(&self) -> &View {
        &self.view
    }

    fn set_size(&mut self) {
        let activity_bar_width = ActivityBar::get().get_width();
        let side_bar_width = SideBar::get().get_width_all();
        let cols = get_term_size().0;
        self.view.x = activity_bar_width + side_bar_width;
        self.view.width = cols - activity_bar_width - side_bar_width;
        self.all_filenm_space_w = self.view.width - FileBar::MENU_BTN_WITH;

        self.set_filenm();
    }
}
