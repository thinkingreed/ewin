use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::Colors, lang::lang_cfg::*, model::default::*};

impl PromReplace {
    pub fn new() -> Self {
        let mut plugin = PromReplace { base: PromPluginBase { config: PromptPluginConfig { is_updown_valid: true, ..PromptPluginConfig::default() }, ..PromPluginBase::default() } };

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_replace.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let all_replace = PromContKeyMenu { disp_str: Lang::get().all_replace.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Confirm) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![P_Cmd::NextContent, P_Cmd::CursorUp, P_Cmd::CursorDown], &mut vec![P_Cmd::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![all_replace, switch_area, cancel]], ..PromContKeyDesc::default() }));

        plugin.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        plugin.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_str.to_string()], buf: vec![], ..PromContInputArea::default() }));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        let input_area = PromContInputArea { desc_str_vec: vec![Lang::get().replace_str.to_string()], buf: vec![], ..PromContInputArea::default() };
        plugin.base.cont_vec.push(Box::new(input_area));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromReplace {
    pub base: PromPluginBase,
}

impl PromPluginTrait for PromReplace {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
