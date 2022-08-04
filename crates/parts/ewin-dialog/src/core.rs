use crate::{btn_grourp::*, cont::cont::*, dialog::*, global::*};
use ewin_cfg::log::*;
use ewin_const::term::*;
use ewin_key::util::*;
use ewin_view::view::*;
use std::ops::Range;
use tokio::sync::MutexGuard;

impl Dialog {
    pub const HEADER_HEIGHT: usize = 1;
    pub const HEADER_MARGIN: usize = 3;
    pub const BTNAREA_HEIGHT: usize = 2;
    pub const CONT_MARGIN_WIDTH: usize = 2;
    pub const CLOSE_BTN_WIDTH: usize = 3;

    pub fn contain_absolute_range(range: &Range<usize>) -> bool {
        let dialog = Dialog::get();
        if range.contains(&dialog.view.y) || range.contains(&(dialog.view.y + dialog.cont.as_base().view.height)) {
            return true;
        } else {
            return false;
        }
    }

    pub fn clear(&mut self) {
        self.is_show = false;
    }

    #[track_caller]
    pub fn get() -> MutexGuard<'static, Dialog> {
        return DIALOG.get().unwrap().try_lock().unwrap();
    }

    pub fn init(dialog_cont_type: DialogContType) {
        if let Ok(mut dialog) = DIALOG.get().unwrap().try_lock() {
            *dialog = Dialog::to_dialog(dialog_cont_type);
            dialog.set_size();
        };
    }

    pub fn set_size(&mut self) {
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
        self.close_btn = View { x: self.view.x + self.view.width - Dialog::CLOSE_BTN_WIDTH, y: self.view.y, y_org: self.view.y_org, width: Dialog::CLOSE_BTN_WIDTH, height: 1, ..View::default() };

        // btn grourp
        match self.btn_group.btn_type {
            DialogBtnGrourpType::Ok => {
                let mut btn = self.btn_group.vec.get_mut(0).unwrap();
                btn.view = View { x: self.view.x + self.view.width / 2 - btn.name_width / 2, y: self.view.y + self.view.height - 1, y_org: 0, width: btn.name_width, height: 1, ..View::default() }
            }
            DialogBtnGrourpType::OkCancel => {}
        };
    }

    pub fn is_header_range(&mut self, y: usize, x: usize) -> bool {
        return self.view.y == y && self.view.is_x_range(x);
    }
    pub fn is_close_btn_range(&mut self, y: usize, x: usize) -> bool {
        return self.close_btn.is_range(y, x);
    }

    pub fn is_btn_grourp_range(&self, y: usize, x: usize) -> Option<DialogBtn> {
        for btn in self.btn_group.vec.iter() {
            if btn.view.is_range(y, x) {
                return Some(btn.clone());
            }
        }
        return None;
    }
}
