use crate::{cont::parts::info::*, cont::parts::key_desc::*, ewin_key::key::cmd::*, model::*, traits::main_trait::*};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::DrawParts, event::*, file::*},
};
use ewin_job::job::*;
use ewin_key::model::*;
use ewin_state::term::*;

impl PromSaveConfirm {
    pub fn save_confirm(&mut self) -> ActType {
        Log::debug_key("PromSaveConfirm.save_confirm");

        match &self.base.cmd.cmd_type {
            CmdType::InsertStr(ref string) => match string.to_uppercase().as_str() {
                CHAR_Y => {
                    return Job::send_cmd(CmdType::SaveFile(SaveFileType::Confirm));
                }
                CHAR_N => {
                    return Job::send_cmd(CmdType::CloseFileCurt(CloseFileType::Forced));
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }

    pub fn new() -> Self {
        let mut prom = PromSaveConfirm { ..PromSaveConfirm::default() };
        let guide = PromContInfo { desc_str_vec: vec![Lang::get().save_confirm_to_close.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() };
        prom.base.cont_vec.push(Box::new(guide));

        let yes = PromContKeyMenu { disp_str: Lang::get().yes.to_string(), key: PromContKeyMenuType::OneChar(CHAR_Y.to_string()) };
        let no = PromContKeyMenu { disp_str: Lang::get().no.to_string(), key: PromContKeyMenuType::OneChar(CHAR_N.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        let key_desc = PromContKeyDesc { desc_vecs: vec![vec![yes, no, cancel]], ..PromContKeyDesc::default() };
        prom.base.cont_vec.push(Box::new(key_desc));

        return prom;
    }

    pub fn init() -> ActType {
        Log::debug_key("Tab::prom_save_confirm");
        if State::get().curt_ref_state().editor.is_changed {
            if !State::get().curt_ref_state().is_nomal() {
                State::get().curt_mut_state().clear();
            }
            Prom::get().init(Box::new(PromSaveConfirm::new()));
            State::get().curt_mut_state().prom = PromState::SaveConfirm;
            return ActType::Draw(DrawParts::TabsAll);
        };
        return ActType::Next;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveConfirm {
    pub base: PromBase,
}
impl PromTrait for PromSaveConfirm {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
