use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_const::model::*;

pub trait ViewEvtTrait: DynClone + Any + Send + 'static + std::fmt::Debug {
    fn resize(&mut self) -> ActType {
        return ActType::Draw(DParts::All);
    }
}

downcast!(dyn ViewEvtTrait);
dyn_clone::clone_trait_object!(ViewEvtTrait);
