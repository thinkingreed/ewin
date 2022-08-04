use ewin_view::view::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogCont {
    pub base: DialogContBase,
}

impl DialogCont {
    pub fn clear(&mut self) {}
}

impl Default for DialogCont {
    fn default() -> Self {
        DialogCont { base: DialogContBase::default() }
    }
}

#[derive(Debug, PartialEq, Default, Eq, Clone, Hash, Copy)]
pub enum DialogContType {
    #[default]
    FileProp,
    AboutApp,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DialogContCfg {
    pub max_width: usize,
    pub min_width: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DialogContBase {
    pub view: View,
    pub cfg: DialogContCfg,
    pub title: String,
    pub cont_type: DialogContType,
    pub cont_vec: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DialogContKVS {}
