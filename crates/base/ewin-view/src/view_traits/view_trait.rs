use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_const::models::evt::*;

pub trait ViewEvtTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn resize(&mut self) -> ActType;
    fn is_tgt_mouse_move(&mut self, y: usize, x: usize) -> bool;
}

downcast!(dyn ViewEvtTrait);
dyn_clone::clone_trait_object!(ViewEvtTrait);
