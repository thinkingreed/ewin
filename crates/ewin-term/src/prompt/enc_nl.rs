use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, model::*},
    model::*,
    terminal::*,
};
use std::io::*;

impl EvtAct {
    pub fn enc_nl(term: &mut Terminal) -> ActType {
        if term.curt().prom.keycmd == KeyCmd::Resize {
            term.curt().prom_enc_nl();
            return ActType::Draw(DParts::All);
        }
        match term.curt().prom.p_cmd {
            P_Cmd::MouseDownLeft(y, x) => {
                term.curt().prom.left_down_choice_enc_nl(y as u16, x as u16);
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::CursorUp | P_Cmd::CursorDown | P_Cmd::CursorLeft | P_Cmd::CursorRight => {
                let curdirection = Direction::keycmd_to_curdirection(&term.curt().prom.keycmd);
                term.curt().prom.move_enc_nl(curdirection);
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::ConfirmPrompt => {
                let (apply_item, enc_item, nl_item, bom_item) = (term.curt().prom.cont_1.get_choice(), term.curt().prom.cont_2.get_choice(), term.curt().prom.cont_3.get_choice(), term.curt().prom.cont_4.get_choice());
                let result = term.tabs[term.idx].editor.buf.set_encoding(&mut term.hbar.file_vec[term.idx], Encode::from_name(&enc_item.name), &nl_item.name, &apply_item.name, &bom_item.name);

                match result {
                    Ok(()) => term.curt().editor.h_file = term.hbar.file_vec[term.idx].clone(),
                    Err(err) => {
                        let err_str = match err.kind() {
                            ErrorKind::PermissionDenied => &Lang::get().no_read_permission,
                            ErrorKind::NotFound => &Lang::get().file_not_found,
                            _ => &Lang::get().file_opening_problem,
                        };
                        return ActType::Draw(DParts::MsgBar(err_str.to_string()));
                    }
                }
                term.clear_curt_tab(true);
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }
}
