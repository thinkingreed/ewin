use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*, search_opt::*},
    ewin_key::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*, types::*};
use ewin_job::job::*;
use ewin_key::model::*;
use ewin_state::term::*;

impl PromSearch {
    pub fn search(&mut self) -> ActType {
        Log::debug_key("EvtAct.search");
        let search_str = self.as_mut_base().get_curt_input_area_str();

        match self.base.cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::Cut | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Undo | CmdType::Redo => {
                Job::send_cmd(CmdType::Search(SearchType::Incremental, search_str));
                return ActType::None;
            }
            CmdType::FindNext | CmdType::FindBack => {
                Job::send_cmd(CmdType::Search(SearchType::Confirm, search_str));
                return ActType::None;
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn new() -> Self {
        let mut prom = PromSearch { ..PromSearch::default() };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_search.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let find_next = PromContKeyMenu { disp_str: Lang::get().search_bottom.to_string(), key: PromContKeyMenuType::Cmd(CmdType::FindNext) };
        let find_back = PromContKeyMenu { disp_str: Lang::get().search_top.to_string(), key: PromContKeyMenuType::Cmd(CmdType::FindBack) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![find_next, find_back, cancel]], ..PromContKeyDesc::default() }));

        prom.base.cont_vec.push(Box::new(PromContSearchOpt::get_searh_opt(&CfgEdit::get_search())));

        let input_area = PromContInputArea { buf: vec![], config: PromInputAreaConfig { is_edit_proc_later: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        prom.base.cont_vec.push(Box::new(input_area));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        return prom;
    }

    pub fn init() -> ActType {
        State::get().curt_mut_state().prom = PromState::Search;
        Prom::get().init(Box::new(PromSearch::new()));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromSearch {
    pub base: PromBase,
}
impl PromTrait for PromSearch {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
