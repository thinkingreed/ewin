use crate::{cont::parts::info::*, cont::parts::key_desc::*, ewin_com::_cfg::key::cmd::*, model::*, prom_trait::main_trait::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*};
use ewin_const::def::*;

impl PromSaveConfirm {
    pub fn new() -> Self {
        let mut plugin = PromSaveConfirm { ..PromSaveConfirm::default() };
        let guide = PromContInfo { desc_str_vec: vec![Lang::get().save_confirm_to_close.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() };
        plugin.base.cont_vec.push(Box::new(guide));

        let yes = PromContKeyMenu { disp_str: Lang::get().yes.to_string(), key: PromContKeyMenuType::OneChar(CHAR_Y.to_string()) };
        let no = PromContKeyMenu { disp_str: Lang::get().no.to_string(), key: PromContKeyMenuType::OneChar(CHAR_N.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        let key_desc = PromContKeyDesc { desc_vecs: vec![vec![yes, no, cancel]], ..PromContKeyDesc::default() };
        plugin.base.cont_vec.push(Box::new(key_desc));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveConfirm {
    pub base: PromBase,
}
impl PromPluginTrait for PromSaveConfirm {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
