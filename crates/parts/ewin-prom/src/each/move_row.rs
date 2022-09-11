use crate::{
    cont::parts::{info::*, input_area::*, key_desc::*},
    ewin_key::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::model::PromState;
use ewin_state::term::*;

impl PromMoveRow {
    pub fn move_row(&mut self) -> ActType {
        Log::debug_key("EvtAct.move_row");

        match &self.base.cmd.cmd_type {
            CmdType::InsertStr(ref str) => {
                let str = str.clone();
                if !str.chars().next().unwrap().is_ascii_digit() {
                    return ActType::Cancel;
                }
                let entered_str = self.as_mut_base().get_tgt_input_area_str(0);

                if format!("{}{}", entered_str, str).chars().count() > self.editor_nrw {
                    return ActType::Cancel;
                }
                let cmd = self.base.cmd.clone();
                self.as_mut_base().get_curt_input_area().unwrap().edit_proc(cmd);
                return ActType::Draw(DrawParts::Prompt);
            }
            CmdType::Confirm => {
                let entered_str = self.as_mut_base().get_tgt_input_area_str(0);
                if entered_str.is_empty() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().not_entered_row_number_to_move.to_string()));
                }
                let row_idx: usize = entered_str.parse().unwrap();
                if row_idx > self.editor_buf_len || row_idx == 0 {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().number_within_current_number_of_rows.to_string()));
                }
                State::get().curt_mut_state().clear();
                Job::send_cmd(CmdType::MoveRow(row_idx));
                return ActType::None;
            }
            _ => {}
        }
        return ActType::Cancel;
    }

    pub fn new(editor_nrw: usize, editor_buf_len: usize) -> Self {
        let mut prom = PromMoveRow { editor_nrw, editor_buf_len, ..PromMoveRow::default() };

        prom.base.cont_vec.push(Box::new(PromContInfo { desc_str_vec: vec![Lang::get().set_move_row.to_string()], fg_color: Colors::get_msg_highlight_fg(), ..PromContInfo::default() }));

        let confirm = PromContKeyMenu { disp_str: Lang::get().move_to_specified_row.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![confirm, cancel]], ..PromContKeyDesc::default() }));

        let input_area = PromContInputArea { buf: vec![], config: PromInputAreaConfig { is_edit_proc_orig: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };
        prom.base.cont_vec.push(Box::new(input_area));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        return prom;
    }

    pub fn init(rnw: usize, len_rows: usize) -> ActType {
        State::get().curt_mut_state().prom = PromState::MoveRow;
        Prom::get().init(Box::new(PromMoveRow::new(rnw, len_rows)));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

#[derive(Default, Debug, Clone)]
pub struct PromMoveRow {
    pub base: PromBase,
    pub editor_nrw: usize,
    pub editor_buf_len: usize,
}

impl PromTrait for PromMoveRow {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}
