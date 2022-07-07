use std::env;

use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_com::_cfg::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, model::default::*};

impl PromGrep {
    pub fn new() -> Self {
        let mut prom = PromGrep { base: PromBase { config: PromptConfig { is_updown_valid: true }, ..PromBase::default() } };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_grep.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let search = PromContKeyMenu { disp_str: Lang::get().search.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let switch_input_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown], &mut vec![CmdType::BackContent]) };
        let complement = PromContKeyMenu { disp_str: Lang::get().complement.to_string(), key: PromContKeyMenuType::PCmdAndStr(CmdType::NextContent, format!("({})", Lang::get().search_folder)) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![search, switch_input_area, complement, cancel]], ..PromContKeyDesc::default() }));

        prom.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        prom.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_str.to_string()], buf: vec![], ..PromContInputArea::default() }));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        // search_file
        // TODO cache
        prom.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_file.to_string()], buf: "*.*".chars().collect(), ..PromContInputArea::default() }));

        // search_folder
        let mut search_folder = PromContInputArea { desc_str_vec: vec![Lang::get().search_folder.to_string()], buf: vec![], config: PromInputAreaConfig { is_path: true, is_path_dir_only: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        // TODO cache
        if let Ok(path) = env::current_dir() {
            search_folder.buf = path.to_string_lossy().to_string().chars().collect();
        };
        prom.base.cont_vec.push(Box::new(search_folder));

        return prom;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromGrep {
    pub base: PromBase,
}
impl PromPluginTrait for PromGrep {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
