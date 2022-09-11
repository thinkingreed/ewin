use crate::sidebar::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;

pub trait SideBarContTrait: Any + Send + DynClone + 'static + std::fmt::Debug {
    fn as_base(&self) -> &SideBarContBase;
    fn as_mut_base(&mut self) -> &mut SideBarContBase;
    fn draw(&self, str_vec: &mut Vec<String>);
    fn draw_scrlbar_v(&self, str_vec: &mut Vec<String>);
}

downcast!(dyn SideBarContTrait);
dyn_clone::clone_trait_object!(SideBarContTrait);
