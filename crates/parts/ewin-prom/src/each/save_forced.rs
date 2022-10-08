use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_key::key::cmd::*,
    model::*,
    traits::main_trait::*,
};
use chrono::{prelude::DateTime, Local};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, file::*},
};
use ewin_job::job::*;
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::files::file::*;
use std::time::SystemTime;

impl PromSaveForced {
    pub fn save_forced(&mut self) -> ActType {
        Log::debug_key("EvtAct.save_forced");

        match &self.base.cmd.cmd_type {
            CmdType::InsertStr(ref s) => match s.to_uppercase().as_str() {
                CHAR_Y => {
                    State::get().curt_mut_state().clear();
                    return Job::send_cmd(CmdType::SaveFile(SaveFileType::Forced));
                }
                CHAR_R => {
                    State::get().curt_mut_state().clear();
                    return Job::send_cmd(CmdType::ReOpenFile(FileOpenType::Reopen));
                }
                _ => return ActType::Cancel,
            },
            _ => return ActType::Cancel,
        }
    }

    pub fn new(open_modified_time: &SystemTime, last_modified_time: SystemTime) -> Self {
        let mut prom = PromSaveForced { ..PromSaveForced::default() };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().file_has_been_modified_by_other_app.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let open_datetime = DateTime::<Local>::from(*open_modified_time);
        let open_str = open_datetime.format("%m-%d %H:%M:%S.%3f").to_string();
        let last_datetime = DateTime::<Local>::from(last_modified_time);
        let last_str = last_datetime.format("%m-%d %H:%M:%S.%3f").to_string();

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![format!("{}:{} {}:{}", &Lang::get().open_modified_time, open_str, &Lang::get().last_modified_time, last_str)], fg_color: Colors::get_default_fg(), ..PromContInfo::default() }));

        let forced = PromContKeyMenu { disp_str: Lang::get().save_forced.to_string(), key: PromContKeyMenuType::OneChar(CHAR_Y.to_string()) };
        let reopen = PromContKeyMenu { disp_str: format!("{}{}", Lang::get().reopen, Lang::get().edit_discard), key: PromContKeyMenuType::OneChar(CHAR_R.to_string()) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![forced, reopen, cancel]], ..PromContKeyDesc::default() }));

        return prom;
    }

    pub fn init() -> ActType {
        Log::debug_key("Tab::prom_save_forced");
        let last_modified_time = File::get_modified_time(&State::get().curt_ref_state().file.fullpath).unwrap();
        State::get().curt_mut_state().prom = PromState::SaveForced;
        Prom::get().init(Box::new(PromSaveForced::new(&State::get().curt_ref_state().file.mod_time, last_modified_time)));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSaveForced {
    pub base: PromBase,
}
impl PromTrait for PromSaveForced {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
