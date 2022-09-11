use crate::view::*;
use crossterm::cursor::MoveTo;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_const::models::view::*;

pub trait ViewEvtTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn is_tgt_mouse_move(&mut self, y: usize, x: usize) -> bool;
    fn view(&self) -> View;

    fn clear_all(&self, str_vec: &mut Vec<String>) {
        for i in self.view().y..self.view().y + self.view().height {
            str_vec.push(format!("{}{}", MoveTo(self.view().x as u16, i as u16), get_space(self.view().width)));
        }
        str_vec.push(format!("{}", MoveTo(self.view().x as u16, self.view().y as u16)));
    }
    fn is_range(&self, y: usize, x: usize) -> bool {
        self.view().y <= y && y < self.view().y + self.view().height && self.view().x <= x && x < self.view().x + self.view().width
    }
}

downcast!(dyn ViewEvtTrait);
dyn_clone::clone_trait_object!(ViewEvtTrait);
