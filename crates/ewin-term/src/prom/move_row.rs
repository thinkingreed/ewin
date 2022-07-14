use crate::{ewin_com::model::*, model::*, terms::term::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::_cfg::key::cmd::{Cmd, CmdType};

impl EvtAct {
    pub fn move_row(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.move_row");

        match &term.curt().prom.cmd.cmd_type {
            CmdType::InsertStr(ref str) => {
                let str = str.clone();
                if !str.chars().next().unwrap().is_ascii_digit() {
                    return ActType::Cancel;
                }
                let entered_str = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);

                if format!("{}{}", entered_str, str).chars().count() > term.curt().editor.get_rnw() {
                    return ActType::Cancel;
                }
                let cmd = term.curt().prom.cmd.clone();
                term.curt().prom.curt.as_mut_base().get_curt_input_area().unwrap().edit_proc(cmd);
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::Confirm => {
                let entered_str = term.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                if entered_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_row_number_to_move.to_string()));
                }
                let row_num: usize = entered_str.parse().unwrap();
                if row_num > term.curt().editor.buf.len_rows() || row_num == 0 {
                    return ActType::Draw(DParts::MsgBar(Lang::get().number_within_current_number_of_rows.to_string()));
                }
                term.curt().editor.set_cur_target_by_x(row_num - 1, 0, false);

                term.curt().clear_curt_tab(true);
                term.set_size();
                term.curt().editor.cmd = Cmd::to_cmd(CmdType::MoveRowProm);
                term.curt().editor.scroll();
                return ActType::Draw(DParts::All);
            }
            _ => {}
        }
        return ActType::Cancel;
    }
}
