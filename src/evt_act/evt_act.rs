use crate::{
    _cfg::keys::{KeyCmd, KeyWhen, Keybind, Keys},
    global::*,
    help::Help,
    log::*,
    model::*,
    prompt::prompt::prompt::*,
    sel_range::SelMode,
    tab::Tab,
    terminal::*,
};

use std::io::Write;

impl EvtAct {
    pub fn match_event<T: Write>(keys: Keys, out: &mut T, term: &mut Terminal) -> bool {
        Terminal::hide_cur();
        Log::info("Pressed key", &keys);

        let evt_act_type = EvtAct::check_next_process(keys, out, term);

        match evt_act_type {
            EvtActType::Exit => return true,
            EvtActType::Hold => {}
            EvtActType::DrawOnly | EvtActType::Next => {
                if evt_act_type == EvtActType::DrawOnly {
                    term.curt().editor.d_range.draw_type = DrawType::All;
                }
                if evt_act_type == EvtActType::Next && !EvtAct::check_err(term) {
                    EvtAct::init(term);
                    Editor::set_state_org(term);

                    let keycmd = Keybind::get_keycmd(&keys, KeyWhen::EditorFocus);

                    Log::debug("keybindcmd", &keycmd);

                    match keycmd {
                        // cursor move
                        KeyCmd::CursorUp | KeyCmd::MouseScrollUp | KeyCmd::CursorDown | KeyCmd::MouseScrollDown | KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => term.curt().editor.cur_move_com(),
                        KeyCmd::CursorFileHome => term.curt().editor.ctrl_home(),
                        KeyCmd::CursorFileEnd => term.curt().editor.ctrl_end(),
                        KeyCmd::CursorPageUp => term.curt().editor.page_up(),
                        KeyCmd::CursorPageDown => term.curt().editor.page_down(),
                        // select
                        KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect => term.curt().editor.shift_move_com(),
                        KeyCmd::AllSelect => term.curt().editor.all_select(),
                        // edit
                        // KeyCmd::InsertChar(c) => term.curt().editor.edit_proc(KeyCmd::InsertChar(c)),
                        KeyCmd::InsertStr(str) => term.curt().editor.edit_proc(KeyCmd::InsertStr(str)),
                        KeyCmd::InsertLine => term.curt().editor.edit_proc(KeyCmd::InsertLine),
                        KeyCmd::DeletePrevChar => term.curt().editor.edit_proc(KeyCmd::DeletePrevChar),
                        KeyCmd::DeleteNextChar => term.curt().editor.edit_proc(KeyCmd::DeleteNextChar),
                        KeyCmd::CutSelect => term.curt().editor.edit_proc(KeyCmd::CutSelect),
                        KeyCmd::Copy => term.curt().editor.copy(),
                        KeyCmd::Undo => term.curt().editor.undo(),
                        KeyCmd::Redo => term.curt().editor.redo(),
                        // find
                        KeyCmd::Find => Prompt::search(term),
                        KeyCmd::FindNext => term.curt().editor.search_str(true, false),
                        KeyCmd::FindBack => term.curt().editor.search_str(false, false),
                        KeyCmd::MoveRow => Prompt::move_row(term),
                        KeyCmd::Grep => Prompt::grep(term),
                        // file
                        KeyCmd::NewTab => term.new_tab(),
                        KeyCmd::NextTab => term.next_tab(),
                        KeyCmd::OpenFile => Prompt::open_file(term),
                        KeyCmd::Encoding => Prompt::enc_nl(term),
                        KeyCmd::CloseFile => {
                            if Prompt::close(term) == true {
                                return true;
                            }
                        }
                        KeyCmd::CloseAllFile => {
                            if term.close_all_tab() == true {
                                return true;
                            }
                        }
                        KeyCmd::SaveFile => {
                            let _ = Tab::save(term);
                        }
                        // key record
                        KeyCmd::StartEndRecordKey => term.curt().record_key_start(),
                        KeyCmd::ExecRecordKey => {
                            Tab::exec_record_key(out, term);
                            return false;
                        }
                        // mouse
                        KeyCmd::MouseDownLeft(y, x) => term.curt().editor.ctrl_mouse(y as usize, x as usize, MouseProc::DownLeft),
                        KeyCmd::MouseDragLeft(y, x) => term.curt().editor.ctrl_mouse(y as usize, x as usize, MouseProc::DragLeft),
                        KeyCmd::MouseDownBoxLeft(y, x) => term.curt().editor.ctrl_mouse(y as usize, x as usize, MouseProc::DownLeftBox),
                        KeyCmd::MouseDragBoxLeft(y, x) => term.curt().editor.ctrl_mouse(y as usize, x as usize, MouseProc::DragLeftBox),
                        KeyCmd::MouseOpeSwitch => term.ctrl_mouse_capture(),
                        // menu
                        KeyCmd::Help => Help::disp_toggle(term),
                        KeyCmd::OpenMenu | KeyCmd::OpenMenuFile | KeyCmd::OpenMenuConvert | KeyCmd::OpenMenuEdit | KeyCmd::OpenMenuSelect => Prompt::menu(term),

                        // Mode
                        KeyCmd::CancelMode => term.curt().editor.cancel_mode(),
                        KeyCmd::BoxSelectMode => term.curt().editor.box_select_mode(),

                        KeyCmd::Resize => term.resize(),
                        // empty
                        KeyCmd::Null => {
                            term.curt().editor.d_range.draw_type = DrawType::All;
                        }
                        // Prompt
                        KeyCmd::ReplacePrompt => Prompt::replace(term),
                        KeyCmd::BackTab | KeyCmd::EscPrompt | KeyCmd::ConfirmPrompt | KeyCmd::FindCaseSensitive | KeyCmd::FindRegex | KeyCmd::Tab => {}
                        // EscPrompt is when Prompt
                        KeyCmd::Unsupported => {
                            term.curt().mbar.set_err(&LANG.unsupported_operation);
                        }
                        KeyCmd::NoBind => {}

                        // Internal use
                        KeyCmd::ReplaceExec(_, _) => {}
                        KeyCmd::DelBox(_) => {}
                        //  KeyCmd::InsertStr(_) => {}
                        KeyCmd::InsertBox(_) => {}
                    }

                    if term.curt().state.key_record_state.is_record {
                        term.curt().editor.record_key();
                    }
                }
                EvtAct::finalize(term);

                // When key_record is executed, redraw only at the end
                if term.curt().state.key_record_state.is_exec == true && term.curt().state.key_record_state.is_exec_end == false {
                    return false;
                }
                if term.curt().editor.d_range.draw_type != DrawType::Not {
                    term.draw(out);
                }
            }
        }
        Terminal::draw_cur(out, term);
        return false;
    }

    pub fn check_next_process<T: Write>(keys: Keys, out: &mut T, term: &mut Terminal) -> EvtActType {
        // term.curt().editor.evt = evt.clone();

        term.curt().editor.keys = keys;
        let keywhen = if term.curt().state.is_nomal() { KeyWhen::EditorFocus } else { KeyWhen::PromptFocus };
        term.curt().editor.keycmd = Keybind::get_keycmd(&keys, keywhen);

        Log::info("KeyCmd", &term.curt().editor.keycmd);
        term.curt().prom.set_keys(keys);

        match &term.curt().editor.keycmd {
            KeyCmd::Resize => {
                if !Terminal::check_displayable() {
                    term.state.is_displayable = false;
                    Terminal::clear_display();
                    println!("{}", &LANG.increase_height_width_terminal);
                    return EvtActType::Hold;
                } else {
                    term.state.is_displayable = true;
                    if term.curt().state.is_open_file {
                    } else {
                        return EvtActType::Next;
                    }
                }
            }
            _ => {
                if !term.state.is_displayable {
                    return EvtActType::Hold;
                }
            }
        }

        let evt_act = EvtAct::check_statusbar(term);
        Log::debug("EvtAct::check_statusbar", &evt_act);
        if evt_act == EvtActType::Next || evt_act == EvtActType::DrawOnly {
            return evt_act;
        }

        let evt_act = EvtAct::check_headerbar(term);
        Log::debug("EvtAct::check_headerbar", &evt_act);
        if evt_act == EvtActType::Next || evt_act == EvtActType::DrawOnly || evt_act == EvtActType::Exit {
            return evt_act;
        }

        if EvtAct::check_err_prompt(term) {
            return EvtActType::DrawOnly;
        }
        EvtAct::clear_mag(&mut term.tabs[term.idx]);
        EvtAct::clear_tab_comp(&mut term.tabs[term.idx]);

        let evt_act = EvtAct::check_prom(term);
        Log::debug("EvtAct::check_prom", &evt_act);

        if evt_act == EvtActType::Hold && !term.curt().state.grep_state.is_result {
            term.set_disp_size();
            term.curt().mbar.draw_only(out);
            Prompt::draw_only(out, &mut term.curt());
        }
        return evt_act;
    }

    pub fn init(term: &mut Terminal) {
        Log::debug_key("EvtAct.init");

        // let tab = term.tabs.get_mut(term.idx).unwrap();
        match term.curt().editor.keycmd {
            // Up, Down
            KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::MouseScrollUp | KeyCmd::MouseScrollDown => {}
            _ => term.curt().editor.updown_x = 0,
        }

        Editor::set_draw_range_init(term.curt());

        // Edit is_change=true, Clear redo_vec,
        if Keybind::is_edit(&term.curt().editor.keycmd, false) {
            term.hbar.file_vec[term.idx].is_changed = true;
            term.curt().editor.history.clear_redo_vec();
        }

        term.curt().mbar.clear_mag();

        // Box Mode cancel
        match term.curt().editor.keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {
                if term.curt().editor.sel.mode == SelMode::BoxSelect {
                    term.curt().editor.box_sel.mode = BoxInsertMode::Insert;
                }
            }
            _ => term.curt().editor.box_sel.mode = BoxInsertMode::Nomal,
        }
    }

    pub fn finalize(term: &mut Terminal) {
        Log::debug_key("EvtAct.finalize");

        // set sel draw range, Clear sel range
        match term.curt().editor.keycmd {
            // Shift
            KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::FindBack => {}
            // Ctrl
            KeyCmd::AllSelect => {}
            // Alt
            KeyCmd::OpenMenu | KeyCmd::OpenMenuFile | KeyCmd::OpenMenuConvert | KeyCmd::OpenMenuEdit | KeyCmd::OpenMenuSelect => {}
            // Raw
            KeyCmd::FindNext => {}
            // mouse
            KeyCmd::MouseScrollUp | KeyCmd::MouseScrollDown | KeyCmd::MouseDownLeft(_, _) | KeyCmd::MouseDragLeft(_, _) | KeyCmd::MouseDownBoxLeft(_, _) | KeyCmd::MouseDragBoxLeft(_, _) => {}
            // other
            KeyCmd::Resize | KeyCmd::BoxSelectMode => {}
            _ => {
                if term.curt().editor.sel.mode == SelMode::BoxSelect {
                    match term.curt().editor.keycmd {
                        KeyCmd::CursorUp | KeyCmd::CursorDown | KeyCmd::CursorLeft | KeyCmd::CursorRight => {}
                        _ => {
                            term.curt().editor.sel.clear();
                            term.curt().editor.sel.mode = SelMode::Normal;
                        }
                    }
                } else {
                    term.curt().editor.sel.clear();
                    term.curt().editor.sel.mode = SelMode::Normal;
                }
            }
        }

        if Keybind::is_edit(&term.curt().editor.keycmd, true) && term.curt().editor.search.ranges.len() > 0 {
            let len_chars = term.curt().editor.buf.len_chars();
            let search_str = &term.curt().editor.search.str.clone();
            term.curt().editor.search.ranges = term.curt().editor.get_search_ranges(search_str, 0, len_chars, 0);
        }

        term.curt().editor.set_draw_range_finalize();

        // Msg changed or
        if term.curt().mbar.is_msg_changed() {
            term.set_disp_size();

            // When displaying a message on the cursor line
            if !term.curt().mbar.msg.str.is_empty() && term.hbar.disp_row_num + term.curt().editor.cur.y - term.curt().editor.offset_y == term.curt().mbar.disp_row_posi {
                term.curt().editor.scroll();
            }
            term.curt().editor.d_range.draw_type = DrawType::All;
        }
        // All draw at the end of key record
        if term.curt().state.key_record_state.is_exec_end == true {
            term.curt().editor.d_range.draw_type = DrawType::All;
        }

        // sel mode cancel
    }
    pub fn check_err(term: &mut Terminal) -> bool {
        if term.curt().editor.keys == Keys::Unsupported {
            term.curt().mbar.set_err(&LANG.unsupported_operation.to_string());
            return true;
        }

        match term.curt().editor.keycmd {
            KeyCmd::CutSelect | KeyCmd::Copy => {
                if !term.curt().editor.sel.is_selected() {
                    term.curt().mbar.set_err(&LANG.no_sel_range.to_string());
                    return true;
                }
            }
            KeyCmd::Undo => {
                if term.curt().editor.history.len_undo() == 0 {
                    term.curt().mbar.set_err(&LANG.no_undo_operation.to_string());
                    return true;
                }
            }
            KeyCmd::Redo => {
                if term.curt().editor.history.len_redo() == 0 {
                    term.curt().mbar.set_err(&LANG.no_redo_operation.to_string());
                    return true;
                }
            }

            _ => {}
        }
        return false;
    }
}
