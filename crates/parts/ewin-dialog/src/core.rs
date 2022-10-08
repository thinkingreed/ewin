use crate::{dialog::*, global::*};
use ewin_const::models::{dialog::*, draw::*, event::*};
use ewin_view::traits::view::*;
use parking_lot::MutexGuard;
use std::ops::Range;

impl Dialog {
    pub const HEADER_HEIGHT: usize = 1;
    pub const HEADER_MARGIN: usize = 3;
    pub const BTNAREA_HEIGHT: usize = 2;
    pub const CONT_MARGIN_WIDTH: usize = 2;
    pub const CLOSE_BTN_WIDTH: usize = 3;

    pub fn contain_absolute_range(range: &Range<usize>) -> bool {
        let dialog = Dialog::get();
        return range.contains(&dialog.view.y) || range.contains(&(dialog.view.y + dialog.cont.as_base().view.height));
    }

    pub fn clear(&mut self) {
        self.is_show = false;
    }

    #[track_caller]
    pub fn get() -> MutexGuard<'static, Dialog> {
        return DIALOG.get().unwrap().lock();
    }

    pub fn init(dialog_cont_type: DialogContType) -> ActType {
        if let Some(mut dialog) = DIALOG.get().unwrap().try_lock() {
            *dialog = Dialog::to_dialog(dialog_cont_type);
            dialog.set_size();
        };
        return ActType::Draw(DrawParts::TabsAll);
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
