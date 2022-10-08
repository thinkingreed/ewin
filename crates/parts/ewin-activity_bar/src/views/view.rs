use crate::activitybar::*;
use ewin_cfg::model::general::default::*;
use ewin_const::{def::*, term::*};
use ewin_help::help::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for ActivityBar {
    fn view(&self) -> &View {
        &self.view
    }

    fn set_size(&mut self) {
        self.view.x = 0;
        self.view.y = MENUBAR_HEIGHT;
        self.view.height = get_term_size().1 - MENUBAR_HEIGHT - Help::get().view.height - MSGBAR_HEIGHT - STATUSBAR_HEIGHT;
        self.view.width = Cfg::get().general.activitybar.width;

        self.set_seize_cont();
    }
}
