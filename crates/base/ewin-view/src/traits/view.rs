use crate::view::*;
use crossterm::cursor::MoveTo;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::log::*;
use ewin_const::models::view::*;

pub trait ViewTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn view(&self) -> &View;

    fn clear_all(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("ViewTrait.clear_all");

        Log::debug("self.view()", &self.view());

        for i in self.view().y..self.view().y_height() {
            str_vec.push(format!("{}{}", MoveTo(self.view().x as u16, i as u16), get_space(self.view().width)));
        }
        str_vec.push(MoveTo(self.view().x as u16, self.view().y as u16).to_string());
    }

    fn is_range(&self, y: usize, x: usize) -> bool {
        self.view().y <= y && y < self.view().y_height() && self.view().x <= x && x < self.view().x_width()
    }

    fn set_size(&mut self);
}

downcast!(dyn ViewTrait);
dyn_clone::clone_trait_object!(ViewTrait);
