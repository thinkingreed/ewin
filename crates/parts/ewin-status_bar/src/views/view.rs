use crate::statusbar::*;
use ewin_const::{def::*, term::*};
use ewin_help::help::*;
use ewin_view::{traits::view::ViewTrait, view::*};

impl ViewTrait for StatusBar {
    fn view(&self) -> &View {
        &self.view
    }

    fn set_size(&mut self) {
        let (cols, rows) = get_term_size();

        let help = Help::get();
        self.view.y = if help.view.height == 0 { rows - 1 } else { rows - help.view.height - 1 };
        self.view.height = STATUSBAR_HEIGHT;
        self.view.width = cols;
    }
}
