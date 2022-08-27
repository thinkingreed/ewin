use crate::tabs::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::models::{draw::*, evt::*};
use ewin_key::key::cmd::*;

impl Tabs {
    pub fn move_row(&mut self) -> ActType {
        Log::debug_key("EvtAct.move_row");

        match &self.curt().prom.cmd.cmd_type {
            CmdType::InsertStr(ref str) => {
                let str = str.clone();
                if !str.chars().next().unwrap().is_ascii_digit() {
                    return ActType::Cancel;
                }
                let entered_str = self.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);

                if format!("{}{}", entered_str, str).chars().count() > self.curt().editor.get_rnw() {
                    return ActType::Cancel;
                }
                let cmd = self.curt().prom.cmd.clone();
                self.curt().prom.curt.as_mut_base().get_curt_input_area().unwrap().edit_proc(cmd);
                return ActType::Draw(DParts::Prompt);
            }
            CmdType::Confirm => {
                let entered_str = self.curt().prom.curt.as_mut_base().get_tgt_input_area_str(0);
                if entered_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_row_number_to_move.to_string()));
                }
                let row_num: usize = entered_str.parse().unwrap();
                if row_num > self.curt().editor.buf.len_rows() || row_num == 0 {
                    return ActType::Draw(DParts::MsgBar(Lang::get().number_within_current_number_of_rows.to_string()));
                }
                self.curt().editor.set_cur_target_by_x(row_num - 1, 0, false);

                self.curt().clear_curt_tab(true);
                self.set_size();
                self.curt().editor.cmd = Cmd::to_cmd(CmdType::MoveRowProm);
                self.curt().editor.scroll();
                return ActType::Draw(DParts::All);
            }
            _ => {}
        }
        return ActType::Cancel;
    }
}
