use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, clipboard::*, global::*, log::*, model::*},
    ewin_prom::model::*,
    model::*,
    tab::Tab,
};
use std::io::Write;

impl EvtAct {
    pub fn ctrl_prom(term: &mut Terminal) -> ActType {
        Log::info_s("ctrl_prom");
        let mut act_type = ActType::Next;
        if !term.curt().state.is_nomal() || EvtAct::check_prom_special_conditions(term) {
            act_type = EvtAct::prompt_check_err(term);
            if ActType::Next != act_type {
                return act_type;
            }
            EvtAct::clear_tab_comp(&mut term.tabs[term.tab_idx]);
            act_type = EvtAct::check_prom(term);
        }
        return act_type;
    }

    pub fn check_prom_special_conditions(term: &mut Terminal) -> bool {
        if term.curt().state.grep.is_result && !term.curt().state.grep.is_greping() && term.curt().prom.p_cmd == P_Cmd::ConfirmPrompt {
            return true;
        }
        return false;
    }

    pub fn check_prom(term: &mut Terminal) -> ActType {
        Log::debug_key("check_prom");

        if !EvtAct::check_promt_suport_keycmd(term) {
            return ActType::Render(RParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }

        // Close・Esc
        match term.curt().prom.p_cmd {
            P_Cmd::EscPrompt => {
                if term.state.is_all_close_confirm {
                    term.cancel_close_all_tab();
                    term.clear_curt_tab(true, true);
                } else if term.state.is_all_save {
                    term.cancel_save_all_tab();
                    term.clear_curt_tab(true, true);
                } else if term.curt().state.grep.is_greping() {
                    GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceling).unwrap();
                    // term.curt().state.grep.clear();
                } else if term.curt().state.grep.is_grep_finished() {
                    term.clear_curt_tab(false, true);
                } else if term.curt().state.is_search {
                    term.curt().editor.search.clear();
                    term.clear_curt_tab(true, true);
                } else {
                    term.clear_curt_tab(true, true);
                }
                return ActType::Render(RParts::All);
            }
            _ => {
                //
                if term.curt().state.grep.is_greping() {
                    term.curt().mbar.clear();
                    return ActType::Cancel;
                }
            }
        }

        let act_type = EvtAct::prompt_check_input_field(term);
        if ActType::Next != act_type {
            return act_type;
        }

        if let Some(act_type) = EvtAct::prompt_check_search_opt(term) {
            return act_type;
        }

        if term.curt().state.is_search {
            return EvtAct::search(term);
        } else if term.curt().state.is_save_new_file {
            return EvtAct::save_new_filenm(term);
        } else if term.curt().state.is_save_confirm {
            return EvtAct::save_confirm(term);
        } else if term.curt().state.is_save_forced {
            return EvtAct::save_forced(term);
        } else if term.curt().state.is_replace {
            return EvtAct::replace(term);
        } else if term.curt().state.is_open_file {
            return EvtAct::open_file(term);
        } else if term.curt().state.is_move_row {
            return EvtAct::move_row(term);
        } else if term.curt().state.is_menu {
            return EvtAct::menu(term);
        } else if term.curt().state.grep.is_grep {
            return EvtAct::grep(term);
        } else if term.curt().state.grep.is_result {
            return EvtAct::grep_result(term);
        } else if term.curt().state.is_enc_nl {
            return EvtAct::enc_nl(term);
        } else if term.curt().state.is_watch_result {
            return EvtAct::watch_result(term);
        } else {
            return ActType::Next;
        }
    }

    pub fn prompt_check_input_field(term: &mut Terminal) -> ActType {
        // contents operation
        if !term.curt().state.is_exists_input_field() {
            return ActType::Next;
        }

        // Clear msg
        term.curt().mbar.clear_mag();
        // Clear sels
        term.curt().prom.clear_sels_keycmd();

        let state = &term.curt().state.clone();
        match term.curt().prom.p_cmd {
            P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect => term.curt().prom.shift_move_com(),
            P_Cmd::InsertStr(_) if state.is_move_row => {}
            P_Cmd::InsertStr(_) => term.curt().prom.operation(),
            P_Cmd::Copy => term.curt().prom.copy(),
            P_Cmd::Cut => term.curt().prom.operation(),
            P_Cmd::Undo => term.curt().prom.undo(),
            P_Cmd::Redo => term.curt().prom.redo(),
            P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::DelNextChar | P_Cmd::DelPrevChar => term.curt().prom.operation(),
            P_Cmd::CursorUp => term.curt().prom.cursor_up(state),
            P_Cmd::CursorDown => term.curt().prom.cursor_down(state),
            P_Cmd::TabNextFocus => term.curt().prom.tab(true, state),
            P_Cmd::BackTabBackFocus => term.curt().prom.tab(false, state),
            P_Cmd::MouseDownLeft(y, x) => term.curt().prom.ctrl_mouse(state, y, x),
            P_Cmd::MouseDragLeft(y, x) => term.curt().prom.ctrl_mouse(state, y, x),
            _ => {}
        }
        // draw Prompt
        if !state.is_move_row && !state.is_search && !state.is_open_file {
            return if EvtAct::is_draw_prompt_tgt_keycmd(&term.curt().prom.p_cmd) { ActType::Render(RParts::Prompt) } else { ActType::Next };
        } else {
            return ActType::Next;
        };
    }

    pub fn is_draw_prompt_tgt_keycmd(p_cmd: &P_Cmd) -> bool {
        if let P_Cmd::InsertStr(_) | P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect | P_Cmd::BackTabBackFocus | P_Cmd::Copy | P_Cmd::Cut | P_Cmd::Undo | P_Cmd::Redo | P_Cmd::CursorUp | P_Cmd::CursorDown | P_Cmd::TabNextFocus | P_Cmd::MouseDownLeft(_, _) | P_Cmd::MouseDragLeft(_, _) = p_cmd {
            return true;
        } else {
            return false;
        };
    }

    pub fn prompt_check_search_opt(term: &mut Terminal) -> Option<ActType> {
        // Search・replace・grep option
        if term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep.is_grep {
            match term.curt().prom.p_cmd {
                P_Cmd::FindCaseSensitive => {
                    term.curt().prom.cont_1.change_opt_case_sens();
                    return Some(ActType::Render(RParts::Prompt));
                }
                P_Cmd::FindRegex => {
                    term.curt().prom.cont_1.change_opt_regex();
                    return Some(ActType::Render(RParts::Prompt));
                }
                P_Cmd::MouseDownLeft(y, x) => {
                    let (y, x) = (y as u16, x as u16);
                    if term.curt().prom.cont_1.opt_row_posi == y {
                        if term.curt().prom.cont_1.opt_1.mouse_area.0 <= x && x <= term.curt().prom.cont_1.opt_1.mouse_area.1 {
                            term.curt().prom.cont_1.change_opt_case_sens();
                            return Some(ActType::Render(RParts::Prompt));
                        } else if term.curt().prom.cont_1.opt_2.mouse_area.0 <= x && x <= term.curt().prom.cont_1.opt_2.mouse_area.1 {
                            term.curt().prom.cont_1.change_opt_regex();
                            return Some(ActType::Render(RParts::Prompt));
                        }
                    }
                }
                _ => {}
            }
        }

        return None;
    }
    pub fn draw_prompt<T: Write>(out: &mut T, term: &mut Terminal) {
        // Hide the cursor at this position to target anything other than mouse move
        Terminal::hide_cur();
        term.set_disp_size();
        term.curt().mbar.draw_only(out);
        let is_msg_changed = term.curt().mbar.is_msg_changed();
        let state = term.curt().state.clone();
        let h_file = term.curt_h_file().clone();
        term.curt().prom.draw_only(out, &state, is_msg_changed, &h_file);
        Terminal::show_cur();
    }

    pub fn set_org_msg(tab: &mut Tab) {
        tab.mbar.msg_org = tab.mbar.msg.clone();
        tab.mbar.msg_keyrecord_org = tab.mbar.msg_keyrecord.clone();
    }

    pub fn clear_tab_comp(tab: &mut Tab) {
        Log::debug_s("check_grep_clear_tab_comp");

        if tab.state.grep.is_grep || tab.state.is_open_file || tab.state.is_save_new_file {
            // Check clear tab candidate
            match tab.prom.p_cmd {
                P_Cmd::InsertStr(_) | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect => {
                    if tab.state.grep.is_grep {
                        tab.prom.prom_grep.tab_comp.clear_tab_comp()
                    } else if tab.state.is_open_file {
                        tab.prom.prom_open_file.tab_comp.clear_tab_comp()
                    } else if tab.state.is_save_new_file {
                        tab.prom.prom_save_new_file.tab_comp.clear_tab_comp()
                    }
                }
                _ => {}
            }
        }
    }
    pub fn prompt_check_err(term: &mut Terminal) -> ActType {
        // Check if sel range is set
        if term.curt().state.is_exists_input_field() {
            match &term.curt().prom.p_cmd {
                P_Cmd::Copy | P_Cmd::Cut => {
                    let is_selected = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.sel.is_selected(),
                        PromptContPosi::Second => term.curt().prom.cont_2.sel.is_selected(),
                        PromptContPosi::Third => term.curt().prom.cont_3.sel.is_selected(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.sel.is_selected(),
                    };
                    if !is_selected {
                        return ActType::Render(RParts::MsgBar(Lang::get().no_sel_range.to_string()));
                    }
                }
                P_Cmd::InsertStr(str) => {
                    if str.is_empty() {
                        if let Some(err_str) = EvtAct::check_clipboard(term) {
                            return ActType::Render(RParts::MsgBar(err_str));
                        };
                    };
                }
                P_Cmd::Undo => {
                    let is_empty_undo = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.history.undo_vec.is_empty(),
                        PromptContPosi::Second => term.curt().prom.cont_2.history.undo_vec.is_empty(),
                        PromptContPosi::Third => term.curt().prom.cont_3.history.undo_vec.is_empty(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.history.undo_vec.is_empty(),
                    };
                    if is_empty_undo {
                        return ActType::Render(RParts::MsgBar(Lang::get().no_undo_operation.to_string()));
                    }
                }
                P_Cmd::Redo => {
                    let is_empty_redo = match term.curt().prom.cont_posi {
                        PromptContPosi::First => term.curt().prom.cont_1.history.redo_vec.is_empty(),
                        PromptContPosi::Second => term.curt().prom.cont_2.history.redo_vec.is_empty(),
                        PromptContPosi::Third => term.curt().prom.cont_3.history.redo_vec.is_empty(),
                        PromptContPosi::Fourth => term.curt().prom.cont_4.history.redo_vec.is_empty(),
                    };
                    if is_empty_redo {
                        return ActType::Render(RParts::MsgBar(Lang::get().no_redo_operation.to_string()));
                    }
                }
                _ => return ActType::Next,
            }
        }
        return ActType::Next;
    }

    pub fn check_clipboard(term: &mut Terminal) -> Option<String> {
        let clipboard = get_clipboard().unwrap_or_else(|_| "".to_string());

        if clipboard.is_empty() {
            return Some(Lang::get().no_value_in_clipboard.to_string());
        }
        // Do not paste multiple lines for Prompt
        if term.curt().state.is_save_new_file || term.curt().state.is_search || term.curt().state.is_replace || term.curt().state.grep.is_grep || term.curt().state.is_move_row {
            // Check multiple lines
            if clipboard.match_indices(&NL::get_nl(&term.curt().editor.h_file.nl)).count() > 0 {
                return Some(Lang::get().cannot_paste_multi_rows.to_string());
            };
        }
        return None;
    }

    pub fn check_promt_suport_keycmd(term: &mut Terminal) -> bool {
        Log::debug_key("check_promt_suport_keycmd");
        Log::debug("term.curt().prom.p_cmd", &term.curt().prom.p_cmd);

        match term.keycmd {
            KeyCmd::Prom(P_Cmd::Resize(_, _)) | KeyCmd::Prom(P_Cmd::CloseFile) => return true,
            _ => {}
        }
        return matches!(
            term.curt().prom.p_cmd,
            P_Cmd::ConfirmPrompt
                | P_Cmd::EscPrompt
                | P_Cmd::CursorLeft
                | P_Cmd::CursorRight
                | P_Cmd::CursorUp
                | P_Cmd::CursorDown
                | P_Cmd::CursorRowHome
                | P_Cmd::CursorRowEnd
                | P_Cmd::CursorLeftSelect
                | P_Cmd::CursorRightSelect
                | P_Cmd::CursorRowHomeSelect
                | P_Cmd::CursorRowEndSelect
                | P_Cmd::MouseScrollUp
                | P_Cmd::MouseScrollDown
                | P_Cmd::BackTabBackFocus
                | P_Cmd::Copy
                | P_Cmd::Cut
                | P_Cmd::Undo
                | P_Cmd::Redo
                | P_Cmd::TabNextFocus
                | P_Cmd::MouseDownLeft(_, _)
                | P_Cmd::MouseDragLeft(_, _)
                | P_Cmd::DelNextChar
                | P_Cmd::DelPrevChar
                | P_Cmd::InsertStr(_)
                | P_Cmd::FindNext
                | P_Cmd::FindBack
                | P_Cmd::FindCaseSensitive
                | P_Cmd::FindRegex
        );
    }
}
