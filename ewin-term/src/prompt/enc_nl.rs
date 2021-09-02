use crate::{
    ewin_core::{_cfg::keys::*, global::*, model::*},
    model::*,
    terminal::*,
};
use std::io::*;

impl EvtAct {
    pub fn enc_nl(term: &mut Terminal) -> EvtActType {
        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                let h_file = term.curt_h_file().clone();
                term.curt().prom_enc_nl(&h_file);
                return EvtActType::Next;
            }
            KeyCmd::MouseDownLeft(y, x) => {
                term.curt().prom.left_down_choice_enc_nl(y as u16, x as u16);
                return EvtActType::Hold;
            }
            KeyCmd::CursorUp => {
                term.curt().prom.move_enc_nl(CurDirection::Up);
                return EvtActType::Hold;
            }
            KeyCmd::CursorDown => {
                term.curt().prom.move_enc_nl(CurDirection::Down);
                return EvtActType::Hold;
            }
            KeyCmd::CursorRight | KeyCmd::Tab => {
                term.curt().prom.move_enc_nl(CurDirection::Right);
                return EvtActType::Hold;
            }
            KeyCmd::CursorLeft | KeyCmd::BackTab => {
                term.curt().prom.move_enc_nl(CurDirection::Left);
                return EvtActType::Hold;
            }
            KeyCmd::ConfirmPrompt => {
                let (apply_item, enc_item, nl_item, bom_item) = (term.curt().prom.cont_1.get_choice(), term.curt().prom.cont_2.get_choice(), term.curt().prom.cont_3.get_choice(), term.curt().prom.cont_4.get_choice());
                let result = term.tabs[term.idx].editor.buf.set_encoding(&mut term.hbar.file_vec[term.idx], Encode::from_name(&enc_item.name), &nl_item.name, &apply_item.name, &bom_item.name);

                match result {
                    Ok(()) => term.curt().editor.h_file = term.hbar.file_vec[term.idx].clone(),
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::PermissionDenied => term.curt().mbar.set_err(&LANG.no_read_permission),
                            ErrorKind::NotFound => term.curt().mbar.set_err(&LANG.file_not_found),
                            _ => term.curt().mbar.set_err(&LANG.file_opening_problem),
                        };
                        return EvtActType::DrawOnly;
                    }
                }
                term.clear_curt_tab();
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
    }
}
