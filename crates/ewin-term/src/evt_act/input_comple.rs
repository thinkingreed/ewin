use crate::{
    ewin_com::{_cfg::key::keycmd::*, log::*, model::*},
    model::*,
};
use ewin_com::def::*;
use ewin_editor::model::*;

impl EvtAct {
    pub fn ctrl_input_comple_before(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_input_comple");

        let e_cmd = &term.curt().editor.e_cmd.clone();
        term.curt().editor.input_comple.window.e_cmd = e_cmd.clone();
        Log::debug("e_cmd", &e_cmd);
        Log::debug("input_comple.window 000", &term.curt().editor.input_comple.window);

        match e_cmd {
            E_Cmd::InputComple => {
                term.curt().editor.init_input_comple(false);
                return ActType::Render(RParts::All);
            }

            E_Cmd::MouseDownLeft(y, x) => {
                if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, false) {
                    let evt_act = EvtAct::select_input_comple(term);
                    term.curt().editor.clear_input_comple();
                    return evt_act;
                }
                let x = x - term.curt().editor.get_rnw_and_margin();
                let y = y - HEADERBAR_ROW_NUM + term.curt().editor.offset_y;
                term.curt().editor.set_cur_target_by_disp_x(y, x);

                term.curt().editor.clear_input_comple();
                return ActType::Render(RParts::All);
            }
            E_Cmd::MouseMove(y, x) => {
                if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, false) {
                    term.curt().editor.input_comple.window.ctrl_mouse_move(*y, *x);
                    if !term.curt().editor.input_comple.window.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let (offset_y, editor_row_len) = (term.curt().editor.offset_y, term.curt().editor.row_disp_len);
                    term.curt().editor.draw_range = term.curt().editor.input_comple.window.get_draw_range_y(offset_y, HEADERBAR_ROW_NUM, editor_row_len);
                    return ActType::Render(RParts::Editor);
                } else if term.curt().editor.input_comple.window.is_mouse_within_range(*y, *x, true) {
                    term.curt().editor.input_comple.window.clear_select_menu();
                    return ActType::Render(RParts::Editor);
                } else {
                    return ActType::Cancel;
                }
            }
            E_Cmd::CursorDown | E_Cmd::CursorUp | E_Cmd::CursorRight | E_Cmd::CursorLeft | E_Cmd::MouseScrollUp | E_Cmd::MouseScrollDown => {
                Log::debug("input_comple.window 111", &&term.curt().editor.input_comple.window);
                match e_cmd {
                    E_Cmd::CursorDown | E_Cmd::MouseScrollDown => term.curt().editor.input_comple.window.cur_move(Direction::Down),
                    E_Cmd::CursorUp | E_Cmd::MouseScrollUp => term.curt().editor.input_comple.window.cur_move(Direction::Up),
                    E_Cmd::CursorRight => term.curt().editor.input_comple.window.cur_move(Direction::Right),
                    E_Cmd::CursorLeft => term.curt().editor.input_comple.window.cur_move(Direction::Left),
                    _ => {}
                }
                let (offset_y, editor_row_len) = (term.curt().editor.offset_y, term.curt().editor.row_disp_len);
                term.curt().editor.draw_range = term.curt().editor.input_comple.window.get_draw_range_y(offset_y, HEADERBAR_ROW_NUM, editor_row_len);
                return ActType::Render(RParts::Editor);
            }
            E_Cmd::InsertRow => {
                let act_type = EvtAct::select_input_comple(term);
                term.curt().editor.clear_input_comple();
                return act_type;
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
            Log::debug_s("1111111111111111111111111");
            // parent
        } else if !term.curt().editor.input_comple.window.is_exist_child_curt_parent() {
            Log::debug_s("2222222222222222222222");
            if let Some((menu, _)) = term.curt().editor.input_comple.window.get_curt_parent() {
                Log::debug_s("3333333333333333333333");
                let add_str = term.curt().editor.get_input_comple_addstr(&menu.name);
                term.curt().editor.edit_proc(E_Cmd::InsertStr(add_str));
                return ActType::Render(RParts::All);
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
