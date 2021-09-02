use crate::{
    ewin_core::{_cfg::keys::*, global::*, log::*, model::*},
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn move_row(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct.move_row");

        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_move_row();
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(str) => {
                let str = str.clone();
                if !str.chars().nth(0).unwrap().is_ascii_digit() {
                    return EvtActType::Hold;
                }
                let entered_str: String = term.curt().prom.cont_1.buf.iter().collect::<String>();
                if entered_str.chars().count() == term.curt().editor.get_rnw() {
                    return EvtActType::Hold;
                }
                term.curt().prom.insert_str(str);

                return EvtActType::DrawOnly;
            }
            KeyCmd::ConfirmPrompt => {
                let str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                if str.is_empty() {
                    term.curt().mbar.set_err(&LANG.not_entered_row_number_to_move);
                    return EvtActType::Hold;
                }
                let row_num: usize = str.parse().unwrap();
                if row_num > term.curt().editor.buf.len_lines() || row_num == 0 {
                    term.curt().mbar.set_err(&LANG.number_within_current_number_of_rows);
                    return EvtActType::Hold;
                }
                term.curt().editor.set_cur_target(row_num - 1, 0, false);

                term.clear_curt_tab();
                term.curt().editor.move_row();
                term.curt().editor.scroll_horizontal();
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}
