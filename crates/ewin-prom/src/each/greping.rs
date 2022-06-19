use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_com::_cfg::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};

impl PromGreping {
    pub fn new() -> Self {
        let mut plugin = PromGreping { ..PromGreping::default() };
        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().long_time_to_search.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![cancel]], ..PromContKeyDesc::default() }));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromGreping {
    pub base: PromBase,
}
impl PromPluginTrait for PromGreping {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
