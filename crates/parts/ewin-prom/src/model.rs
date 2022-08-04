use super::{
    each::search::*,
    prom_trait::{cont_trait::*, main_trait::*},
};
use ewin_key::key::cmd::*;
use std::ops::Range;

#[derive(Default, Debug, Clone)]
pub struct PromptContOpt {
    pub key_str: String,
    pub key_desc_str: String,
    pub is_check: bool,
    pub mouse_area: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct Prom {
    // pub keycmd: KeyCmd,
    pub cmd: Cmd,
    pub row_num: usize,
    pub row_posi: usize,
    pub row_bottom_posi: usize,
    pub col_num: usize,
    pub curt: Box<dyn PromTrait>,
}

impl Default for Prom {
    fn default() -> Self {
        Self { row_num: 0, row_posi: 0, row_bottom_posi: 0, col_num: 0, curt: Box::new(PromSearch::new()), cmd: Cmd::default() }
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromBase {
    pub cmd: Cmd,
    pub cfg: PromptConfig,
    pub curt_cont_idx: usize,
    pub curt_cont_idx_org: usize,
    // Hold PromptCont for each line
    pub cont_vec: Vec<Box<dyn PromContTrait>>,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromptConfig {
    pub is_updown_valid: bool,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromptContBase {
    pub cmd: Cmd,
    pub row_posi_range: Range<usize>,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContChoiceConfig {
    pub is_multi_row: bool,
}
