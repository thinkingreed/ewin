use crate::{model::*, prom_trait::cont_trait::*};

impl PromContTrait for PromContInfo {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, _: bool) {
        for disp_str in &self.desc_str_vec {
            str_vec.push(format!("{}{}", self.fg_color, disp_str));
        }
    }
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContInfo {
    pub fg_color: String,
    pub base: PromptContBase,
    pub desc_str_vec: Vec<String>,
}
