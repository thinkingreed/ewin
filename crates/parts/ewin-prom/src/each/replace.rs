use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_key::key::cmd::*,
    model::*,
    traits::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::util::*;

impl PromReplace {
    pub fn replace(&mut self) -> ActType {
        Log::info_key("EvtAct.replace");

        match &self.base.cmd.cmd_type {
            CmdType::Confirm => {
                let mut search_str = self.as_mut_base().get_tgt_input_area_str(0);
                let mut replace_str = self.as_mut_base().get_tgt_input_area_str(1);

                search_str = change_regex(search_str);
                replace_str = change_regex(replace_str);

                if search_str.is_empty() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().not_set_search_str.to_string()));
                } else {
                    return Job::send_cmd(CmdType::ReplaceTryExec(search_str, replace_str));
                }
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn new() -> Self {
        let mut prom = PromReplace { base: PromBase { cfg: PromptConfig { is_updown_valid: true }, ..PromBase::default() } };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_replace.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let all_replace = PromContKeyMenu { disp_str: Lang::get().all_replace.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().move_setting_location.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown], &mut vec![CmdType::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![all_replace, switch_area, cancel]], ..PromContKeyDesc::default() }));

        prom.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        prom.base.cont_vec.push(Box::new(PromContInputArea { desc_str_vec: vec![Lang::get().search_str.to_string()], buf: vec![], ..PromContInputArea::default() }));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        let input_area = PromContInputArea { desc_str_vec: vec![Lang::get().replace_str.to_string()], buf: vec![], ..PromContInputArea::default() };
        prom.base.cont_vec.push(Box::new(input_area));

        return prom;
    }

    pub fn init() -> ActType {
        State::get().curt_mut_state().prom = PromState::Replase;
        Prom::get().init(Box::new(PromReplace::new()));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromReplace {
    pub base: PromBase,
}

impl PromTrait for PromReplace {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
