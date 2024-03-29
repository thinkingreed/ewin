use crate::model::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait PromContTrait: DynClone + Any + Send + 'static + std::fmt::Debug {
    fn as_base(&self) -> &PromptContBase;
    fn as_mut_base(&mut self) -> &mut PromptContBase;
    fn draw(&self, vec: &mut Vec<String>, is_curt: bool);
    fn check_allow_p_cmd(&self) -> bool {
        false
    }
}

downcast!(dyn PromContTrait);
dyn_clone::clone_trait_object!(PromContTrait);
