use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait ViewEvtTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn is_tgt_mouse_move(&mut self, y: usize, x: usize) -> bool;

    fn exec_mouse_up_left(&mut self);
}

downcast!(dyn ViewEvtTrait);
dyn_clone::clone_trait_object!(ViewEvtTrait);
