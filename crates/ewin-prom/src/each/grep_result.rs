use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};

impl PromPluginGrepResult {
    pub fn new(is_grep_empty: bool, is_cancel: bool) -> Self {
        let mut plugin = PromPluginGrepResult { ..PromPluginGrepResult::default() };

        let cancel_str = if is_cancel { Lang::get().processing_canceled.to_string() } else { "".to_string() };

        if is_grep_empty {
            plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{} {}", Lang::get().show_search_no_result, cancel_str)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
            let close = PromContKeyMenu { disp_str: Lang::get().close.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::CloseFile) };
            plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![close]], ..PromContKeyDesc::default() }));
        } else {
            plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{} {}", Lang::get().show_search_result, cancel_str)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
            let open_target_file = PromContKeyMenu { disp_str: Lang::get().open_target_file.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Confirm) };
            let search = PromContKeyMenu { disp_str: Lang::get().search.to_string(), key: PromContKeyMenuType::ECmd(E_Cmd::Find) };
            let close = PromContKeyMenu { disp_str: Lang::get().close.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::CloseFile) };
            plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![open_target_file, search, close]], ..PromContKeyDesc::default() }));
        }
        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromPluginGrepResult {
    pub base: PromPluginBase,
}

impl PromPluginTrait for PromPluginGrepResult {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
