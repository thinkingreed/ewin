use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
    model::*,
};
use ewin_editor::model::*;

impl EvtAct {
    pub fn ctrl_input_comple_before(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_input_comple");

        let e_cmd = &term.curt().editor.e_cmd.clone();
        Log::debug("e_cmd", &e_cmd);
        Log::debug("input_comple.window 000", &&term.curt().editor.input_comple.window);

        match e_cmd {
            E_Cmd::InputComple => {
                term.curt().editor.init_input_comple(false);
                return ActType::Render(RParts::All);
            }
            E_Cmd::MouseDownLeft(y, x) => {
                if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, false) {
                    term.curt().editor.clear_input_comple();
                    return EvtAct::select_input_comple(term);
                }
                return ActType::Cancel;
            }
            E_Cmd::MouseMove(y, x) => {
                if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, false) {
                    term.curt().editor.input_comple.window.ctrl_mouse_move(*y, *x);
                    if !term.curt().editor.input_comple.window.is_menu_change() {
                        return ActType::Cancel;
                    }
                    return ActType::Render(RParts::Editor);
                } else if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, true) {
                    term.curt().editor.input_comple.window.clear_select_menu();
                    return ActType::Render(RParts::Editor);
                } else {
                    return ActType::Cancel;
                }
            }
            E_Cmd::CursorDown | E_Cmd::CursorUp | E_Cmd::CursorRight | E_Cmd::CursorLeft => {
                Log::debug("input_comple.window 111", &&term.curt().editor.input_comple.window);
                match e_cmd {
                    E_Cmd::CursorDown => term.curt().editor.input_comple.window.cur_move(Direction::Down),
                    E_Cmd::CursorUp => term.curt().editor.input_comple.window.cur_move(Direction::Up),
                    E_Cmd::CursorRight => term.curt().editor.input_comple.window.cur_move(Direction::Right),
                    E_Cmd::CursorLeft => term.curt().editor.input_comple.window.cur_move(Direction::Left),
                    _ => {}
                }
                return ActType::Render(RParts::Editor);
            }

            E_Cmd::InsertRow => {
                EvtAct::select_input_comple(term);
                term.curt().editor.clear_input_comple();
                return ActType::Render(RParts::Editor);
            }
            _ => return ActType::Next,
        }
    }

    pub fn ctrl_input_comple_after(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_input_comple");
        let e_cmd = &term.curt().editor.e_cmd.clone();
        Log::debug("e_cmd", &e_cmd);

        match e_cmd {
            E_Cmd::InsertStr(_) | E_Cmd::DelPrevChar => {
                term.curt().editor.init_input_comple(false);
                return ActType::Render(RParts::All);
            }
            _ if Editor::is_edit(&term.curt().editor.e_cmd, true) => {
                term.curt().editor.state.input_comple_mode = InputCompleMode::None;
                return ActType::Next;
            }
            _ => return ActType::Next,
        }
    }

    pub fn select_input_comple(term: &mut Terminal) -> ActType {
        Log::debug_key("select_input_comple");
        if let Some((_, _)) = term.curt().editor.input_comple.window.get_curt_child() {
        } else if !term.curt().editor.input_comple.window.is_exist_child_curt_parent() {
            if let Some((menu, _)) = term.curt().editor.input_comple.window.get_curt_parent() {
                let add_str = term.curt().editor.get_input_comple_addstr(&menu.name);
                term.curt().editor.edit_proc(E_Cmd::InsertStr(add_str));
                return ActType::Render(RParts::Editor);
            }
        }
        return ActType::Cancel;
    }

    /*
    pub fn is_input_comple_displayed_area(term: &mut Terminal, y: usize, x: usize) -> bool {
        if y == term.hbar.row_posi {
            for h_file in term.hbar.file_vec.iter() {
                if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 || h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                    return true;
                }
            }
        } else {
            return true;
        }
        return false;
    }
     */
}
