use crate::{
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        _cfg::lang::lang_cfg::*,
        def::*,
        global::*,
        log::*,
        model::*,
    },
    model::*,
    tab::*,
};
use directories::BaseDirs;
use ewin_window::{ctx_menu::org::*, window::WindowTrait};
use std::io::stdout;

impl EvtAct {
    pub fn ctrl_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_ctx_menu");

        match term.ctx_menu.c_cmd {
            C_Cmd::MouseDownLeft(y, x) => {
                if term.ctx_menu.window.is_mouse_within_range(y, x, false) {
                    return EvtAct::select_ctx_menu(term);
                }
                return ActType::Cancel;
            }
            C_Cmd::MouseMove(y, x) => {
                if term.ctx_menu.window.is_mouse_within_range(y, x, false) {
                    let child_cont_org = &term.ctx_menu.window.curt_cont.menu_vec.get(term.ctx_menu.window.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.ctx_menu.window.ctrl_mouse_move(y, x);

                    if !term.ctx_menu.window.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.ctx_menu.window.curt_cont.menu_vec.get(term.ctx_menu.window.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move || Only child meun move
                    if child_cont_org.is_none() && child_cont.is_none() || term.ctx_menu.window.parent_sel_y == term.ctx_menu.window.parent_sel_y_cache && term.ctx_menu.window.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Render(RParts::CtxMenu);
                    } else {
                        term.set_render_range_ctx_menu();
                        return ActType::Render(RParts::Editor);
                    }
                } else if term.ctx_menu.window.is_mouse_within_range(y, x, true) {
                    term.ctx_menu.window.clear_select_menu();
                    term.set_render_range_ctx_menu();
                    return ActType::Render(RParts::Editor);
                } else {
                    return ActType::Cancel;
                }
            }
            C_Cmd::CursorDown | C_Cmd::CursorUp | C_Cmd::CursorRight | C_Cmd::CursorLeft => {
                match term.ctx_menu.c_cmd {
                    C_Cmd::CursorDown => term.ctx_menu.window.cur_move(Direction::Down),
                    C_Cmd::CursorUp => term.ctx_menu.window.cur_move(Direction::Up),
                    C_Cmd::CursorRight => term.ctx_menu.window.cur_move(Direction::Right),
                    C_Cmd::CursorLeft => term.ctx_menu.window.cur_move(Direction::Left),
                    _ => {}
                }
                term.set_render_range_ctx_menu();
                return ActType::Render(RParts::Editor);
            }
            C_Cmd::CtxMenu(y, x) => {
                EvtAct::show_init_ctx_menu(term, y, x);
                return ActType::Render(RParts::All);
            }
            C_Cmd::ConfirmCtxMenu => {
                EvtAct::select_ctx_menu(term);
                return ActType::Render(RParts::All);
            }
            C_Cmd::Null => return ActType::Cancel,
        }
    }

    pub fn is_ctx_menu_keys(keys: &Keys, term: &mut Terminal) -> bool {
        if term.state.is_ctx_menu {
            let rtn = match keys {
                Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) => true,
                Keys::MouseMove(_, _) => true,
                Keys::MouseDownLeft(y, x) => term.ctx_menu.window.is_mouse_within_range(*y as usize, *x as usize, false),
                Keys::MouseDragRight(_, _) => true,
                _ => true,
            };
            return rtn;
        }
        return false;
    }
    pub fn select_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("select_ctx_menu");
        if let Some((ctx_menu, _)) = term.ctx_menu.window.get_curt_child() {
            return EvtAct::check_ctx_menu_func(term, &term.ctx_menu.window.get_curt_parent().unwrap().0.name, &ctx_menu.name);
        } else if !term.ctx_menu.window.is_exist_child_curt_parent() {
            if let Some((parent_ctx_menu, _)) = term.ctx_menu.window.get_curt_parent() {
                return EvtAct::check_ctx_menu_func(term, &parent_ctx_menu.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_ctx_menu_func(term: &mut Terminal, parent_name: &str, child_name: &str) -> ActType {
        Log::debug_key("check_func");

        let mut evt_act_type = ActType::Render(RParts::Editor);
        term.clear_ctx_menu();
        if LANG_MAP.get(parent_name).is_some() {
            if &Lang::get().macros == LANG_MAP.get(parent_name).unwrap() {
                if let Some(base_dirs) = BaseDirs::new() {
                    let full_path_str = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR).join(child_name);
                    if full_path_str.exists() {
                        evt_act_type = Macros::exec_js_macro(term, &full_path_str.to_string_lossy().to_string());
                    } else {
                        evt_act_type = ActType::Render(RParts::MsgBar(Lang::get().file_not_found.clone()));
                    }
                }
            } else if LANG_MAP.get(child_name).is_some() {
                evt_act_type = EvtAct::exec_ctx_menu_func(term, child_name);
            } else {
                evt_act_type = EvtAct::exec_ctx_menu_func(term, parent_name);
            }
        } else if LANG_MAP.get(child_name).is_some() {
            evt_act_type = EvtAct::exec_ctx_menu_func(term, child_name);
        }
        return evt_act_type;
    }

    pub fn exec_ctx_menu_func(term: &mut Terminal, name: &str) -> ActType {
        Log::debug_key("exec_func");
        Log::debug("select name", &name);

        match &LANG_MAP[name] {
            //// editor
            // convert
            s if s == &Lang::get().to_uppercase || s == &Lang::get().to_lowercase || s == &Lang::get().to_full_width || s == &Lang::get().to_half_width || s == &Lang::get().to_space || s == &Lang::get().to_tab => {
                term.curt().editor.convert(ConvType::from_str_conv_type(&LANG_MAP[name]));
                term.curt().editor.sel.clear();
            }
            // format
            s if s == &Lang::get().json || s == &Lang::get().xml || s == &Lang::get().html => {
                if let Some(err_str) = term.curt().editor.format(FileType::from_str_fmt_type(s)) {
                    return ActType::Render(RParts::AllMsgBar(err_str));
                } else {
                    // highlight data reset
                    term.editor_draw_vec[term.tab_idx].clear();
                }
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().cut => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Cut)), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().copy => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Copy)), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().paste => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::InsertStr("".to_string()))), &mut stdout(), term);
            }
            s if s == &Lang::get().all_select => {
                EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::AllSelect)), &mut stdout(), term);
            }
            //// headerbar
            // close
            s if s == &Lang::get().close => {
                if Tab::prom_save_confirm(term) {
                    Terminal::exit();
                }
                return ActType::Render(RParts::All);
            }
            // close other than this tab
            s if s == &Lang::get().close_other_than_this_tab => {
                let _ = term.close_tabs(term.tab_idx);
                return ActType::Render(RParts::All);
            }
            _ => {}
        };
        term.curt().editor.draw_range = E_DrawRange::All;
        return ActType::Render(RParts::Editor);
    }

    pub fn set_ctx_menu_curt_term_place(term: &mut Terminal, y: usize) {
        if term.hbar.row_posi == y {
            term.ctx_menu.window.curt_cont = term.ctx_menu.ctx_menu_place_map[&TermPlace::HeaderBar].clone();
        } else if term.curt().editor.row_posi <= y && y <= term.curt().editor.row_posi + term.curt().editor.row_disp_len {
            let place_cond = if term.curt().editor.sel.is_selected() { TermPlace::Editor(TermPlaceCond::EditorRangeSelected) } else { TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected) };
            term.ctx_menu.window.curt_cont = term.ctx_menu.ctx_menu_place_map[&place_cond].clone();
        }
    }

    pub fn is_ctx_menu_displayed_area(term: &mut Terminal, y: usize, x: usize) -> bool {
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

    pub fn show_init_ctx_menu(term: &mut Terminal, y: usize, x: usize) {
        term.ctx_menu.clear();

        if EvtAct::is_ctx_menu_displayed_area(term, y, x) {
            let (y, x) = if y == USIZE_UNDEFINED {
                (term.curt().editor.cur.y - term.curt().editor.offset_y + term.hbar.row_num, if term.curt().editor.state.mouse_mode == MouseMode::Normal { term.curt().editor.cur.disp_x + term.curt().editor.get_rnw_and_margin() } else { term.curt().editor.cur.disp_x })
            } else {
                (y, x)
            };
            term.state.is_ctx_menu = true;
            EvtAct::set_ctx_menu_curt_term_place(term, y);
            term.ctx_menu.window.set_parent_disp_area(y, x);
        } else if term.state.is_ctx_menu {
            term.state.is_ctx_menu = false;
        }
    }
}
