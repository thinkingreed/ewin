use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, log::*, model::*},
    model::*,
};
impl EvtAct {
    pub fn move_row(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.move_row");

        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_move_row();
                return ActType::Draw(DParts::All);
            }
            KeyCmd::Prom(p_keycmd) => match p_keycmd {
                P_Cmd::InsertStr(str) => {
                    let str = str.clone();
                    if !str.chars().next().unwrap().is_ascii_digit() {
                        return ActType::Cancel;
                    }
                    let entered_str: String = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    if entered_str.chars().count() == term.curt().editor.get_rnw() {
                        return ActType::Cancel;
                    }
                    term.curt().prom.insert_str(str);

                    return ActType::Draw(DParts::Prompt);
                }
                P_Cmd::ConfirmPrompt => {
                    let str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    if str.is_empty() {
                        return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_row_number_to_move.to_string()));
                    }
                    let row_num: usize = str.parse().unwrap();
                    if row_num > term.curt().editor.buf.len_rows() || row_num == 0 {
                        return ActType::Draw(DParts::MsgBar(Lang::get().number_within_current_number_of_rows.to_string()));
                    }
                    term.curt().editor.set_cur_target(row_num - 1, 0, false);

                    term.clear_curt_tab(true);
                    term.curt().editor.set_offset_y_move_row();
                    term.curt().editor.scroll_horizontal();
                    return ActType::Draw(DParts::All);
                }
                _ => return if EvtAct::is_draw_prompt_tgt_keycmd(&term.curt().prom.p_cmd) { ActType::Draw(DParts::Prompt) } else { ActType::Cancel },
            },
            _ => return ActType::Cancel,
        }
    }
}
