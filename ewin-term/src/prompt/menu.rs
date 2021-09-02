use crate::{
    ewin_core::{_cfg::keys::*, global::*, log::*, model::*},
    ewin_prom::{cont::promptcont::*, prompt::choice::*},
    model::*,
    terminal::*,
};
use std::io::stdout;

impl EvtAct {
    pub fn menu(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct.menu");

        let state = term.curt().state.clone();

        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_menu();
                return EvtActType::Next;
            }
            KeyCmd::MouseDownLeft(y, x) => {
                if term.curt().prom.left_down_choice_menu(y as u16, x as u16) {
                    return EvtAct::confirm_menu(term, true);
                }
                return EvtActType::None;
            }
            KeyCmd::BackTab => {
                term.curt().prom.tab(false, &state);
                return EvtActType::Hold;
            }
            KeyCmd::CursorUp => {
                term.curt().prom.change_choice_vec_menu(CurDirection::Up);
                Choices::change_show_choice(&mut term.curt().prom);
                return EvtActType::Hold;
            }
            KeyCmd::CursorDown => {
                term.curt().prom.change_choice_vec_menu(CurDirection::Down);
                Choices::change_show_choice(&mut term.curt().prom);
                return EvtActType::Hold;
            }
            KeyCmd::Tab => {
                term.curt().prom.tab(true, &state);
                return EvtActType::Hold;
            }
            KeyCmd::CursorLeft | KeyCmd::CursorRight => {
                if term.curt().prom.keycmd == KeyCmd::CursorRight {
                    term.curt().prom.change_choice_vec_menu(CurDirection::Right);
                } else {
                    term.curt().prom.change_choice_vec_menu(CurDirection::Left);
                }
                Choices::change_show_choice(&mut term.curt().prom);

                return EvtActType::Hold;
            }
            KeyCmd::ConfirmPrompt => {
                term.curt().prom.cache_menu();
                return EvtAct::confirm_menu(term, false);
            }
            _ => return EvtActType::Hold,
        }
    }

    pub fn confirm_menu(term: &mut Terminal, is_click: bool) -> EvtActType {
        let choice_1 = term.curt().prom.cont_1.get_choice();
        let choice_2 = term.curt().prom.cont_2.get_choice();
        let choice_3 = term.curt().prom.cont_3.get_choice();

        if is_click {
            if term.curt().prom.cont_posi == PromptContPosi::First {
                return EvtActType::DrawOnly;
            } else if term.curt().prom.cont_posi == PromptContPosi::Second && !choice_3.is_none() {
                return EvtActType::DrawOnly;
            }
        }

        // file
        if choice_1.name.contains(&LANG.file) {
            term.clear_curt_tab();
            if choice_2.name.contains(&LANG.encode) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Encoding), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.create_new) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::NewTab), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.open) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::OpenFile(OpenFileType::Normal)), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.save_as) {
                term.curt().prom.save_new_file();
            } else if choice_2.name.contains(&LANG.end_of_all_save) && term.save_all_tab() {
                return EvtActType::Exit;
            }
            // edit
        } else if choice_1.name.contains(&LANG.edit) {
            term.clear_curt_tab();
            if choice_2.name.contains(&LANG.box_select) {
                term.curt().editor.box_select_mode();
            } else if term.curt().editor.sel.is_selected() {
                if choice_2.name.contains(&LANG.convert) {
                    term.curt().editor.convert(ConvType::from_str(&choice_3.name));
                } else if choice_2.name.contains(&LANG.format) {
                    term.curt().editor.format(FmtType::JSON);
                }
            } else {
                term.curt().mbar.set_err(&LANG.no_sel_range)
            }
            // macros
        } else if choice_1.name.contains(&LANG.macros) {
            term.clear_curt_tab();
            if choice_2.name.contains(&LANG.specify_file_and_exec_macro) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::OpenFile(OpenFileType::JsMacro)), &mut stdout(), term);
            }
        }
        return EvtActType::DrawOnly;
    }
}
