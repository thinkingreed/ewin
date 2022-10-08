use crate::{cont::*, traits::traits::*};

impl ActivutyBarSearch {
    pub fn new(base: ActivityContBase) -> Self {
        return Self { base };
    }
}

#[derive(Default, Debug, Clone)]
pub struct ActivutyBarSearch {
    pub base: ActivityContBase,
}
impl ActivityBarContTrait for ActivutyBarSearch {
    fn as_base(&self) -> &ActivityContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut ActivityContBase {
        &mut self.base
    }
}
