use crate::cont::*;
use crossterm::cursor::MoveTo;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::{colors::*, log::*};
use ewin_const::models::view::*;
use unicode_width::UnicodeWidthStr;

pub trait ActivityBarContTrait: Any + DynClone + Send + 'static + std::fmt::Debug {
    fn as_base(&self) -> &ActivityContBase;
    fn as_mut_base(&mut self) -> &mut ActivityContBase;

    fn draw_row(&self, str_vec: &mut Vec<String>, idx: usize) {
        let color = if self.as_base().is_select { Colors::get_activitybar_bg_select() } else { Colors::get_activitybar_bg_default() };
        str_vec.push(color);

        str_vec.push(MoveTo(self.as_base().view.x as u16, (idx) as u16).to_string());
        self.adjust_icon_row(self.as_base().icon.to_string(), str_vec);
        str_vec.push(MoveTo(self.as_base().view.x as u16, (idx + 1) as u16).to_string());
        str_vec.push(get_space(self.as_base().view.width));
    }

    fn adjust_icon_row(&self, icon: String, str_vec: &mut Vec<String>) {
        Log::debug_key("ActivityBarContTrait.adjust_icon_row");

        Log::debug("self.as_base().view.width", &self.as_base().view.width);
        Log::debug("icon", &icon);
        Log::debug("icon.width()", &icon.width());
        let herf_width = (self.as_base().view.width - icon.width()) / 2;
        str_vec.push(get_space(herf_width));
        str_vec.push(icon);
        str_vec.push(get_space(herf_width));
    }
}

downcast!(dyn ActivityBarContTrait);
dyn_clone::clone_trait_object!(ActivityBarContTrait);
