use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait ViewTrait: Any + DynClone + Send + 'static + std::fmt::Debug {
    fn is_tgt_mouse_move(&mut self, y: usize, x: usize) -> bool;
}

downcast!(dyn ViewTrait);
dyn_clone::clone_trait_object!(ViewTrait);
