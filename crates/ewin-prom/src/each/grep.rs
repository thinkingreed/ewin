use std::env;

use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, model::default::*};

impl PromGrep {
    pub fn new() -> Self {
        let mut plugin = PromGrep { base: PromPluginBase { config: PromptPluginConfig { is_updown_valid: true, ..PromptPluginConfig::default() }, ..PromPluginBase::default() } };

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_grep.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let search = PromContKeyMenu { disp_str: Lang::get().search.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Confirm) };
        let switch_input_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![P_Cmd::NextContent, P_Cmd::CursorUp, P_Cmd::CursorDown], &mut vec![P_Cmd::BackContent]) };
        let complement = PromContKeyMenu { disp_str: Lang::get().complement.to_string(), key: PromContKeyMenuType::PCmdAndStr(P_Cmd::NextContent, format!("({})", Lang::get().search_folder)) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![search, switch_input_area, complement, cancel]], ..PromContKeyDesc::default() }));

        plugin.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        plugin.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_str.to_string()], buf: vec![], ..PromContInputArea::default() }));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        // search_file
        // TODO cache
        plugin.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_file.to_string()], buf: "*.*".chars().collect(), ..PromContInputArea::default() }));

        // search_folder
        let mut search_folder = PromContInputArea { desc_str_vec: vec![Lang::get().search_folder.to_string()], buf: vec![], config: PromInputAreaConfig { is_path: true, is_path_dir_only: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        // TODO cache
        if let Ok(path) = env::current_dir() {
            search_folder.buf = path.to_string_lossy().to_string().chars().collect();
        };
        plugin.base.cont_vec.push(Box::new(search_folder));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromGrep {
    pub base: PromPluginBase,
}
impl PromPluginTrait for PromGrep {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
