use super::{
    each::search::*,
    prom_trait::{cont_trait::*, main_trait::*},
};
use ewin_com::_cfg::key::keycmd::*;
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
    pub keycmd: KeyCmd,
    pub p_cmd: P_Cmd,
    pub row_num: usize,
    pub row_posi: usize,
    pub col_num: usize,
    pub curt: Box<dyn PromPluginTrait>,
}

impl Default for Prom {
    fn default() -> Self {
        Self { row_num: 0, row_posi: 0, col_num: 0, curt: Box::new(PromPluginSearch::new()), keycmd: KeyCmd::Null, p_cmd: P_Cmd::Null }
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromPluginBase {
    pub p_cmd: P_Cmd,
    pub config: PromptPluginConfig,
    pub curt_cont_idx: usize,
    pub curt_cont_idx_org: usize,
    // Hold PromptCont for each line
    pub cont_vec: Vec<Box<dyn PromContPluginTrait>>,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromptPluginConfig {
    pub is_updown_valid: bool,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromptContBase {
    pub keycmd: KeyCmd,
    pub p_cmd: P_Cmd,
    pub row_posi_range: Range<usize>,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContChoiceConfig {
    pub is_multi_row: bool,
}
