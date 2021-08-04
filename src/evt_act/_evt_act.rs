use crate::{
    _cfg::keys::{KeyCmd, KeyWhen, Keybind, Keys},
    global::*,
    help::Help,
    log::*,
    model::*,
    prompt::prompt::prompt::*,
    sel_range::{BoxInsertMode, SelMode},
    tab::Tab,
    terminal::*,
};
use std::io::Write;

impl EvtAct {
    pub fn match_event<T: Write>(keys: Keys, out: &mut T, term: &mut Terminal) -> bool {
        Terminal::hide_cur();

        EvtAct::set_keycmd(keys, term);
        let evt_act_type = EvtAct::check_next_process(out, term);

        match evt_act_type {
            EvtActType::Exit => return true,
            EvtActType::Hold | EvtActType::None => {}
            EvtActType::DrawOnly | EvtActType::Next => {
                Log::info("Pressed key", &keys);

                if evt_act_type == EvtActType::DrawOnly {
                    term.curt().editor.draw_type = DrawType::All;
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
                        KeyCmd::OpenFile(_) => Prompt::open_file(term),
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
                        KeyCmd::StartEndRecordKey => term.curt().record_key_macro_start(),
                        KeyCmd::ExecRecordKey => {
                            Tab::exec_macro_key(out, term);
                            return false;
                        }
                        // mouse
                        KeyCmd::MouseDownLeft(_, _) | KeyCmd::MouseDragLeft(_, _) | KeyCmd::MouseDownBoxLeft(_, _) | KeyCmd::MouseDragBoxLeft(_, _) => term.curt().editor.ctrl_mouse(),
                        KeyCmd::MouseOpeSwitch => term.ctrl_mouse_capture(),
                        // menu
                        KeyCmd::Help => Help::disp_toggle(term),
                        KeyCmd::OpenMenu | KeyCmd::OpenMenuFile | KeyCmd::OpenMenuConvert | KeyCmd::OpenMenuEdit | KeyCmd::OpenMenuSearch | KeyCmd::OpenMenuMacro => Prompt::menu(term),
                        // Mode
                        KeyCmd::CancelMode => term.curt().editor.cancel_mode(),
                        KeyCmd::BoxSelectMode => term.curt().editor.box_select_mode(),

                        KeyCmd::Resize => term.resize(),
                        // empty
                        KeyCmd::Null => {
                            term.curt().editor.draw_type = DrawType::All;
                        }
                        // Prompt
                        KeyCmd::ReplacePrompt => Prompt::replace(term),
                        KeyCmd::BackTab | KeyCmd::EscPrompt | KeyCmd::ConfirmPrompt | KeyCmd::FindCaseSensitive | KeyCmd::FindRegex | KeyCmd::Tab => {}
                        // EscPrompt is when Prompt
                        KeyCmd::Unsupported => {
                            term.curt().mbar.set_err(&LANG.unsupported_operation);
                        }
                        //
                        KeyCmd::NoBind => {}

                        //// Internal use
                        KeyCmd::ReplaceExec(_, _, _) => {}
                        KeyCmd::DelBox(_) => {}
                        KeyCmd::InsertBox(_) => {}
                        KeyCmd::Format(_) => {}
                        KeyCmd::ExecMacro => {}
                        // ctx_menu
                        KeyCmd::MouseDownRight(_, _) => {}
                        KeyCmd::MouseMove(_, _) => {}
                    }

                    if term.curt().state.key_macro_state.is_record {
                        term.curt().editor.record_key();
                    }
                    EvtAct::finalize(term);
                }
                EvtAct::check_msg(term);

                // When key_record is executed, redraw only at the end
                if term.curt().state.key_macro_state.is_exec == true && term.curt().state.key_macro_state.is_exec_end == false {
                    return false;
                }
                if term.curt().editor.draw_type != DrawType::Not {
                    term.draw(out);
                }
            }
        }
        Terminal::draw_cur(out, term);
        return false;
    }
    pub fn set_keycmd(keys: Keys, term: &mut Terminal) {
        let keywhen = if term.curt().state.is_nomal() { KeyWhen::EditorFocus } else { KeyWhen::PromptFocus };
        let keycmd = Keybind::get_keycmd(&keys, keywhen);

        term.keycmd = keycmd.clone();
        term.curt().editor.keys = keys;
        term.curt().editor.keycmd = keycmd;
        term.curt().prom.set_keys(keys);
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Terminal) -> EvtActType {
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
            KeyCmd::MouseMove(_, _) => {
                if !term.state.is_ctx_menu {
                    return EvtActType::Hold;
                }
            }
            _ => {
                if !term.state.is_displayable {
                    return EvtActType::Hold;
                }
            }
        }
        Log::info("KeyCmd", &term.curt().editor.keycmd);

        // ctx_menu
        let evt_act_type = EvtAct::check_ctx_menu(term);
        if EvtActType::check_next_process_type(&evt_act_type) {
            return evt_act_type;
        }

        // statusbar
        let evt_act_type = EvtAct::check_statusbar(term);
        if EvtActType::check_next_process_type(&evt_act_type) {
            return evt_act_type;
        }

        // headerbar
        let evt_act_type = EvtAct::check_headerbar(term);
        if EvtActType::check_next_process_type(&evt_act_type) {
            return evt_act_type;
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

        // Box Mode
        match &term.curt().editor.keycmd {
            KeyCmd::InsertStr(_) => {
                if term.curt().editor.sel.mode == SelMode::BoxSelect {
                    term.curt().editor.box_insert.mode = BoxInsertMode::Insert;
                }
            }
            KeyCmd::Undo | KeyCmd::Redo | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => {}
            _ => term.curt().editor.box_insert.mode = BoxInsertMode::Normal,
        }
    }

    pub fn finalize(term: &mut Terminal) {
        Log::debug_key("EvtAct.finalize");

        // set sel draw range, Clear sel range
        match term.curt().editor.keycmd {
            // Shift
            KeyCmd::CursorUpSelect | KeyCmd::CursorDownSelect | KeyCmd::CursorRightSelect | KeyCmd::CursorLeftSelect | KeyCmd::CursorRowHomeSelect | KeyCmd::CursorRowEndSelect | KeyCmd::FindBack => {}
            // Ctrl
            KeyCmd::AllSelect | KeyCmd::OpenFile(_) => {}
            // Alt
            KeyCmd::OpenMenu | KeyCmd::OpenMenuFile | KeyCmd::OpenMenuConvert | KeyCmd::OpenMenuEdit | KeyCmd::OpenMenuSearch => {}
            // Raw
            KeyCmd::FindNext => {}
            // mouse
            KeyCmd::MouseScrollUp | KeyCmd::MouseScrollDown | KeyCmd::MouseDownLeft(_, _) | KeyCmd::MouseDownRight(_, _) | KeyCmd::MouseDragLeft(_, _) | KeyCmd::MouseDownBoxLeft(_, _) | KeyCmd::MouseDragBoxLeft(_, _) | KeyCmd::MouseMove(_, _) => {}
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
        term.set_draw_range_ctx_menu();

        // All draw at the end of key record
        if term.curt().state.key_macro_state.is_exec_end == true {
            term.curt().editor.draw_type = DrawType::All;
        }
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
                Log::debug_s("check_err KeyCmd::Undo");

                if term.curt().editor.history.len_undo() == 0 {
                    Log::debug_s("check_err KeyCmd::Undo 1111111111111");

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

    pub fn check_msg(term: &mut Terminal) {
        // Msg changed or
        if term.curt().mbar.is_msg_changed() {
            term.set_disp_size();

            // When displaying a message on the cursor line
            if !term.curt().mbar.msg.str.is_empty() && term.hbar.disp_row_num + term.curt().editor.cur.y - term.curt().editor.offset_y == term.curt().mbar.disp_row_posi {
                term.curt().editor.scroll();
            }
            term.curt().editor.draw_type = DrawType::All;
        }
    }
}