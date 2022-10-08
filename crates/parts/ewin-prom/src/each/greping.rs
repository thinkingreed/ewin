use crate::{
    cont::parts::{info::*, key_desc::*},
    ewin_key::key::cmd::*,
    model::*,
    traits::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};
use ewin_const::models::{draw::*, event::*};
use ewin_key::model::*;
use ewin_state::term::*;

impl PromGreping {
    pub fn new() -> Self {
        let mut prom = PromGreping { ..PromGreping::default() };
        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().long_time_to_search.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![cancel]], ..PromContKeyDesc::default() }));

        return prom;
    }

    pub fn init() -> ActType {
        State::get().curt_mut_state().prom = PromState::Greping;
        Prom::get().init(Box::new(PromGreping::new()));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromGreping {
    pub base: PromBase,
}
impl PromTrait for PromGreping {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
