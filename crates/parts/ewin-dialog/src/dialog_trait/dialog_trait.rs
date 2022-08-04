use crate::cont::cont::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait DialogContTrait: Any + DynClone + Send + 'static + std::fmt::Debug {
    fn as_base(&self) -> &DialogContBase;
    fn as_mut_base(&mut self) -> &mut DialogContBase;
    fn create_cont_vec(&mut self) -> Vec<String>;
}

downcast!(dyn DialogContTrait);
dyn_clone::clone_trait_object!(DialogContTrait);
