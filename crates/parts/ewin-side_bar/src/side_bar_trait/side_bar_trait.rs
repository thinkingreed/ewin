use crate::sidebar::*;
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_view::view::*;

pub trait SideBarContTrait: Any + Send + DynClone + 'static + std::fmt::Debug {
    fn as_base(&self) -> &SideBarContBase;
    fn as_mut_base(&mut self) -> &mut SideBarContBase;
    fn draw(&self, str_vec: &mut Vec<String>);
    fn resize(&mut self);
    fn get_cont_vec_len(&self) -> usize;
    fn get_cont_view(&self) -> View;
}

downcast!(dyn SideBarContTrait);
dyn_clone::clone_trait_object!(SideBarContTrait);
