use crate::help::*;
use ewin_const::term::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for Help {
    fn view(&self) -> &View {
        &self.view
    }

    fn set_size(&mut self) {
        let (cols, rows) = get_term_size();
        self.view.width = cols;
        self.view.height = if self.is_show { Help::DISP_ROW_NUM } else { 0 };
        self.view.y = if self.is_show { rows - self.view.height } else { 0 };
    }
}
