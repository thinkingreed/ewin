use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_com::_cfg::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use chrono::{prelude::DateTime, Local};
use ewin_cfg::{colors::*, lang::lang_cfg::*};
use ewin_const::def::*;
use std::time::SystemTime;

impl PromSaveForced {
    pub fn new(open_modified_time: &SystemTime, last_modified_time: SystemTime) -> Self {
        let mut plugin = PromSaveForced { ..PromSaveForced::default() };

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().file_has_been_modified_by_other_app.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let open_datetime = DateTime::<Local>::from(*open_modified_time);
        let open_str = open_datetime.format("%m-%d %H:%M:%S.%3f").to_string();
        let last_datetime = DateTime::<Local>::from(last_modified_time);
        let last_str = last_datetime.format("%m-%d %H:%M:%S.%3f").to_string();

        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{}:{} {}:{}", &Lang::get().open_modified_time, open_str, &Lang::get().last_modified_time, last_str)], fg_color: Colors::get_default_fg(), ..PromContInfo::default() }));

        let forced = PromContKeyMenu { disp_str: Lang::get().save_forced.to_string(), key: PromContKeyMenuType::OneChar(CHAR_Y.to_string()) };
        let reopen = PromContKeyMenu { disp_str: format!("{}{}", Lang::get().reopen.to_string(), Lang::get().edit_discard.to_string()), key: PromContKeyMenuType::OneChar(CHAR_R.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![forced, reopen, cancel]], ..PromContKeyDesc::default() }));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveForced {
    pub base: PromBase,
}
impl PromPluginTrait for PromSaveForced {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
