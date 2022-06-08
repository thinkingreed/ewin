use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::Colors, lang::lang_cfg::*, model::default::*};

impl PromPluginSearch {
    pub fn new() -> Self {
        let mut plugin = PromPluginSearch { ..PromPluginSearch::default() };

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_search.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let find_next = PromContKeyMenu { disp_str: Lang::get().search_bottom.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::FindNext) };
        let find_back = PromContKeyMenu { disp_str: Lang::get().search_top.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::FindBack) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![find_next, find_back, cancel]], ..PromContKeyDesc::default() }));

        plugin.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        let input_area = PromContInputArea { buf: vec![], config: PromInputAreaConfig { is_edit_proc_later: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        plugin.base.cont_vec.push(Box::new(input_area));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromPluginSearch {
    pub base: PromPluginBase,
}
impl PromPluginTrait for PromPluginSearch {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
