use crate::msgbar::*;
use ewin_const::{def::*, term::*};
use ewin_help::help::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for MsgBar {
    fn set_size(&mut self) {
        let (cols, rows) = get_term_size();
        let help_height = Help::get().view.height;
        self.view.width = cols;
        self.view.height = MSGBAR_HEIGHT;
        self.view.y = rows - help_height - STATUSBAR_HEIGHT - 1;
    }

    fn view(&self) -> &View {
        &self.view
    }
}
