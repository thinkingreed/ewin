use crate::{
    cont::parts::info::*,
    cont::parts::{input_area::*, key_desc::*, pulldown::*},
    ewin_com::_cfg::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, model::default::*};
use ewin_widget::widget::pulldown::*;
use indexmap::*;

impl PromSaveNewFile {
    pub fn new(candidate_new_filenm: String) -> Self {
        let mut plugin = PromSaveNewFile { base: PromBase { config: PromptConfig { is_updown_valid: true }, ..PromBase::default() }, ..PromSaveNewFile::default() };
        plugin.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{}", &Lang::get().set_new_filenm,)], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let confirm = PromContKeyMenu { disp_str: Lang::get().search_top.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown], &mut vec![CmdType::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![confirm, switch_area, cancel]], ..PromContKeyDesc::default() }));

        let input_area = PromContInputArea { desc_str_vec: vec![Lang::get().filenm.clone()], buf: candidate_new_filenm.chars().collect::<Vec<char>>(), ..PromContInputArea::default() };
        plugin.base.cont_vec.push(Box::new(input_area));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        let mut pulldown_cont = PromContPulldown { desc_str_vec: vec![Lang::get().extension.clone()], pulldown: Pulldown::new(), ..PromContPulldown::default() };

        let mut vec = Cfg::get().general.editor.save.candidate_extension_when_saving_new_file.clone();
        let mut edit_vec = vec![];
        for s in vec.iter_mut() {
            if s.is_empty() {
                edit_vec.push(Lang::get().none.to_string());
            } else {
                edit_vec.push(format!(".{}", &s));
            };
        }
        pulldown_cont.pulldown.set_disp_name(IndexSet::from_iter(edit_vec.iter().cloned()));
        plugin.base.cont_vec.push(Box::new(pulldown_cont));

        return plugin;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveNewFile {
    pub base: PromBase,
}
impl PromPluginTrait for PromSaveNewFile {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
