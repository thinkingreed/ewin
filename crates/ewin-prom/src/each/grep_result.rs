use crate::{
    cont::parts::{info::*, key_desc::*},
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};
use ewin_com::_cfg::key::cmd::*;

impl PromGrepResult {
    pub fn new(is_grep_empty: bool, is_cancel: bool) -> Self {
        let mut plugin = PromGrepResult { ..PromGrepResult::default() };

        let cancel_str = if is_cancel { Lang::get().processing_canceled.to_string() } else { "".to_string() };
        if is_grep_empty {
            plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{} {}", Lang::get().show_search_no_result, cancel_str)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
            let close = PromContKeyMenu { disp_str: Lang::get().close.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CloseFile) };
            plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![close]], ..PromContKeyDesc::default() }));
        } else {
            plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{} {}", Lang::get().show_search_result, cancel_str)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
            let open_target_file = PromContKeyMenu { disp_str: Lang::get().open_target_file.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
            let search = PromContKeyMenu { disp_str: Lang::get().search.to_string(), key: PromContKeyMenuType::Cmd(CmdType::FindProm) };
            let close = PromContKeyMenu { disp_str: Lang::get().close.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CloseFile) };
            plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![open_target_file, search, close]], ..PromContKeyDesc::default() }));
        }
        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromGrepResult {
    pub base: PromBase,
}

impl PromPluginTrait for PromGrepResult {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
