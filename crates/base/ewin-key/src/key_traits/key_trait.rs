use crate::key::keys::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait KeyTrait: DynClone + Any + Send + 'static + std::fmt::Debug {
    fn is_allow_key(&mut self, key: Keys) -> bool;
}

downcast!(dyn KeyTrait);
dyn_clone::clone_trait_object!(KeyTrait);
