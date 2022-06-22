use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*},
    ewin_com::_cfg::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*};

impl PromMoveRow {
    pub fn new() -> Self {
        let mut prom = PromMoveRow { ..PromMoveRow::default() };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_move_row.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let confirm = PromContKeyMenu { disp_str: Lang::get().move_to_specified_row.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![confirm, cancel]], ..PromContKeyDesc::default() }));

        let input_area = PromContInputArea { buf: vec![], config: PromInputAreaConfig { is_edit_proc_orig: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        prom.base.cont_vec.push(Box::new(input_area));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        return prom;
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromMoveRow {
    pub base: PromBase,
}
impl PromPluginTrait for PromMoveRow {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
