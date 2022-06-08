use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};
use ewin_const::def::*;

impl PromPluginWatchFile {
    pub fn new() -> Self {
        let mut plugin = PromPluginWatchFile { ..PromPluginWatchFile::default() };

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().file_has_been_modified_by_other_app.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let reopen = PromContKeyMenu { disp_str: format!("{}({})", Lang::get().reopen.to_string(), Lang::get().edit_discard.to_string()), key: PromContKeyMenuType::OneChar(CHAR_R.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        let load = PromContKeyMenu { disp_str: Lang::get().not_edited_will_reloaded_auto.to_string(), key: PromContKeyMenuType::OneChar(CHAR_L.to_string()) };
        let not = PromContKeyMenu { disp_str: Lang::get().no_further_monitoring.to_string(), key: PromContKeyMenuType::OneChar(CHAR_N.to_string()) };

        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![reopen]], ..PromContKeyDesc::default() }));
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![cancel]], ..PromContKeyDesc::default() }));
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![load]], ..PromContKeyDesc::default() }));
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![not]], ..PromContKeyDesc::default() }));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromPluginWatchFile {
    pub base: PromPluginBase,
}

impl PromPluginTrait for PromPluginWatchFile {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}
