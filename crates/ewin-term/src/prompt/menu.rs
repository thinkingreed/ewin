use crate::{
    ewin_com::{_cfg::key::keycmd::*, global::*, log::*, model::*},
    ewin_prom::{model::*, prompt::choice::*},
    model::*,
    terminal::*,
};
use std::io::stdout;

impl EvtAct {
    pub fn menu(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.menu");
        let state = term.curt().state.clone();

        match &term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.curt().prom_menu();
                return ActType::Draw(DParts::All);
            }
            _ => {}
        }
        match term.curt().prom.p_cmd {
            P_Cmd::MouseDownLeft(y, x) => {
                if term.curt().prom.left_down_choice_menu(y as u16, x as u16) {
                    return EvtAct::confirm_menu(term, true);
                }
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::TabNextFocus | P_Cmd::BackTabBackFocus => {
                let is_asc = if term.curt().prom.p_cmd == P_Cmd::TabNextFocus { true } else { false };
                term.curt().prom.tab(is_asc, &state);
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::CursorUp | P_Cmd::CursorDown | P_Cmd::CursorLeft | P_Cmd::CursorRight => {
                let curdirection = Direction::keycmd_to_curdirection(&term.curt().prom.keycmd);
                term.curt().prom.change_choice_vec_menu(curdirection);
                Choices::change_show_choice(&mut term.curt().prom);
                return ActType::Draw(DParts::Prompt);
            }
            P_Cmd::ConfirmPrompt => {
                term.curt().prom.cache_menu();
                return EvtAct::confirm_menu(term, false);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn confirm_menu(term: &mut Terminal, is_click: bool) -> ActType {
        Log::debug_key("confirm_menu");

        let (choice_1, choice_2, choice_3) = (term.curt().prom.cont_1.get_choice(), term.curt().prom.cont_2.get_choice(), term.curt().prom.cont_3.get_choice());
        if is_click {
            if term.curt().prom.cont_posi == PromptContPosi::First || (term.curt().prom.cont_posi == PromptContPosi::Second && !choice_3.is_none()) {
                return ActType::Draw(DParts::All);
            }
        }
        term.clear_curt_tab(true);
        // file
        if choice_1.name.contains(&LANG.file) {
            if choice_2.name.contains(&LANG.encode) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Encoding)), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.create_new) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::NewTab)), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.open) {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::OpenFile(OpenFileType::Normal))), &mut stdout(), term);
            } else if choice_2.name.contains(&LANG.save_as) {
                term.curt().prom.save_new_file();
            } else if choice_2.name.contains(&LANG.end_of_all_save) {
                let act_type = term.save_all_tab();
                if let ActType::Draw(_) = act_type {
                    return act_type;
                } else {
                    return ActType::Exit;
                }
            }
            // edit
        } else if choice_1.name.contains(&LANG.edit) {
            if choice_2.name.contains(&LANG.box_select) {
                term.curt().editor.box_select_mode();
            } else if term.curt().editor.sel.is_selected() {
                if choice_2.name.contains(&LANG.convert) {
                    term.curt().editor.convert(ConvType::from_str(&choice_3.name));
                } else if choice_2.name.contains(&LANG.format) {
                    term.curt().editor.format(FmtType::from_str(&choice_3.name));
                }
            } else {
                term.clear_curt_tab(true);
                return ActType::Draw(DParts::AllMsgBar(LANG.no_sel_range.to_string()));
            }
            // macros
        } else if choice_1.name.contains(&LANG.macros) {
            if choice_2.name.contains(&LANG.specify_file_and_exec_macro) {
                term.curt().prom_open_file(OpenFileType::JsMacro);
                // EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::OpenFile(OpenFileType::JsMacro))), &mut stdout(), term);
            }
        }
        return ActType::Draw(DParts::All);
    }
}
