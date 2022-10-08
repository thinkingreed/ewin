use crate::{btn_grourp::*, dialog::*};
use ewin_cfg::log::*;
use ewin_const::term::*;
use ewin_utils::str_edit::*;
use ewin_view::{traits::view::*, view::*};

impl ViewTrait for Dialog {
    fn view(&self) -> &View {
        return &self.view;
    }

    fn set_size(&mut self) {
        Log::debug_key("set_size");
        let (cols, rows) = get_term_size();

        self.cont.as_mut_base().cont_vec = self.cont.create_cont_vec();

        Log::debug("self.cont.as_mut_base().cont_vec", &self.cont.as_mut_base().cont_vec);

        self.cont.as_mut_base().view.height = self.cont.as_mut_base().cont_vec.len();
        self.view.width = get_str_width(&self.cont.as_mut_base().cont_vec[0]);

        self.view.height = self.cont.as_mut_base().view.height + Dialog::HEADER_HEIGHT + Dialog::BTNAREA_HEIGHT;
        self.view.y = rows / 2 - self.view.height / 2;
        self.view.x = cols / 2 - self.view.width / 2;

        // close_btn
        self.close_btn = View {
            x: self.view.x + self.view.width - Dialog::CLOSE_BTN_WIDTH,
            y: self.view.y,
            // y_org: self.view.y_org,
            width: Dialog::CLOSE_BTN_WIDTH,
            height: 1,
            ..View::default()
        };

        // btn grourp
        match self.btn_group.btn_type {
            DialogBtnGrourpType::Ok => {
                let mut btn = self.btn_group.vec.get_mut(0).unwrap();
                btn.view = View { x: self.view.x + self.view.width / 2 - btn.name_width / 2, y: self.view.y + self.view.height - 1, y_org: 0, width: btn.name_width, height: 1, ..View::default() }
            }
            DialogBtnGrourpType::OkCancel => {}
        };
    }
}
