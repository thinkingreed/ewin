use crate::{
    ewin_com::{_cfg::key::keys::*, model::*},
    global_term::H_FILE_VEC,
    model::*,
};
use directories::BaseDirs;
use ewin_cfg::{global::*, lang::lang_cfg::*, log::*, model::modal::*};
use ewin_com::_cfg::key::{cmd::CmdType, keybind::Keybind};
use ewin_const::def::*;
use ewin_editor::model::*;
use ewin_widget::{core::*, widget::ctx_menu::*};
use std::{cmp::min, io::stdout};

impl EvtAct {
    pub fn ctrl_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_ctx_menu");

        match term.ctx_widget.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if term.ctx_widget.widget.is_mouse_within_area(y, x) {
                    return EvtAct::select_ctx_menu(term);
                }
                return ActType::Cancel;
            }
            CmdType::MouseMove(y, x) => {
                if term.ctx_widget.widget.is_mouse_within_area(y, x) {
                    let child_cont_org = &term.ctx_widget.widget.cont.cont_vec.get(term.ctx_widget.widget.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.ctx_widget.widget.ctrl_mouse_move(y, x);

                    if !term.ctx_widget.widget.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.ctx_widget.widget.cont.cont_vec.get(term.ctx_widget.widget.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move || Only child meun move
                    if child_cont_org.is_none() && child_cont.is_none() || term.ctx_widget.widget.parent_sel_y == term.ctx_widget.widget.parent_sel_y_org && term.ctx_widget.widget.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Draw(DParts::CtxMenu);
                    } else {
                        // term.set_render_range_ctx_menu();
                        // return ActType::Draw(DParts::Editor);
                        return ActType::Draw(DParts::Absolute(term.ctx_widget.widget.get_disp_range_y()));
                    }
                } else if term.ctx_widget.widget.is_mouse_area_around(y, x) {
                    term.ctx_widget.widget.clear_select_menu();
                    // term.set_render_range_ctx_menu();
                    // return ActType::Draw(DParts::Editor);
                    return ActType::Draw(DParts::Absolute(term.ctx_widget.widget.get_disp_range_y()));
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                match term.ctx_widget.cmd.cmd_type {
                    CmdType::CursorDown => term.ctx_widget.widget.cur_move(Direction::Down),
                    CmdType::CursorUp => term.ctx_widget.widget.cur_move(Direction::Up),
                    CmdType::CursorRight => term.ctx_widget.widget.cur_move(Direction::Right),
                    CmdType::CursorLeft => term.ctx_widget.widget.cur_move(Direction::Left),
                    _ => {}
                }
                if !term.ctx_widget.widget.is_menu_change() {
                    return ActType::Cancel;
                }
                return ActType::Draw(DParts::Absolute(term.ctx_widget.widget.get_disp_range_y()));
            }
            CmdType::CtxMenu(y, x) => {
                // let editor_row_posi = term.curt_mut().editor.row_posi;
                term.init_ctx_menu(y, x);
                return ActType::Draw(DParts::All);
            }
            CmdType::Confirm => {
                EvtAct::select_ctx_menu(term);
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Cancel,
        }
    }

    pub fn select_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("select_ctx_menu");
        if let Some((ctx_menu, _)) = term.ctx_widget.widget.get_curt_child() {
            return EvtAct::check_ctx_menu_func(term, &term.ctx_widget.widget.get_curt_parent().unwrap().0.name, &ctx_menu.name);
        } else if !term.ctx_widget.widget.is_exist_child_curt_parent() {
            if let Some((parent_ctx_menu, _)) = term.ctx_widget.widget.get_curt_parent() {
                return EvtAct::check_ctx_menu_func(term, &parent_ctx_menu.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_ctx_menu_func(term: &mut Terminal, parent_name: &str, child_name: &str) -> ActType {
        Log::debug_key("check_func");

        let mut evt_act_type = ActType::Draw(DParts::All);
        term.clear_ctx_menu();
        if LANG_MAP.get(parent_name).is_some() {
            if &Lang::get().macros == LANG_MAP.get(parent_name).unwrap() {
                if let Some(base_dirs) = BaseDirs::new() {
                    let full_path_str = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR).join(child_name);
                    if full_path_str.exists() {
                        evt_act_type = Macros::exec_js_macro(term, &full_path_str.to_string_lossy().to_string());
                    } else {
                        evt_act_type = ActType::Draw(DParts::MsgBar(Lang::get().file_not_found.clone()));
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
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            // format
            s if s == &Lang::get().json || s == &Lang::get().xml || s == &Lang::get().html => {
                if let Some(err_str) = term.curt().editor.format(FileType::from_str_fmt_type(s)) {
                    return ActType::Draw(DParts::AllMsgBar(err_str));
                } else {
                    // highlight data reset
                    term.editor_draw_vec.clear();
                }
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().cut => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::Cut), &mut stdout(), term);
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().copy => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::Copy), &mut stdout(), term);
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().paste => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::InsertStr("".to_string())), &mut stdout(), term);
            }
            s if s == &Lang::get().all_select => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::AllSelect), &mut stdout(), term);
            }
            //// headerbar
            // close
            s if s == &Lang::get().close => return term.curt().prom_show_com(&CmdType::CloseAllFile),

            // close other than this tab
            s if s == &Lang::get().close_other_than_this_tab => {
                return term.close_tabs(term.tab_idx);
            }
            _ => {}
        };
        return ActType::Draw(DParts::Editor(E_DrawRange::All));
    }

    pub fn set_ctx_menu_curt_term_place(term: &mut Terminal, y: usize) {
        if term.fbar.row_posi == y {
            term.ctx_widget.widget.cont = term.ctx_widget.ctx_menu_place_map[&TermPlace::HeaderBar].clone();
        } else if term.curt().editor.get_curt_row_posi() <= y && y <= term.curt().editor.get_curt_row_posi() + term.curt().editor.get_curt_row_len() {
            let place_cond = if term.curt().editor.win_mgr.curt().sel.is_selected() { TermPlace::Editor(TermPlaceCond::EditorRangeSelected) } else { TermPlace::Editor(TermPlaceCond::EditorRangeNonSelected) };
            term.ctx_widget.widget.cont = term.ctx_widget.ctx_menu_place_map[&place_cond].clone();
        }
    }

    pub fn is_ctx_menu_displayed_area(term: &mut Terminal, y: usize, x: usize) -> bool {
        if y == term.fbar.row_posi {
            for h_file in H_FILE_VEC.get().unwrap().try_lock().unwrap().iter() {
                if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 || h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                    return true;
                }
            }
        } else {
            return true;
        }
        return false;
    }
}

impl Terminal {
    pub fn init_ctx_menu(&mut self, y: usize, x: usize) {
        Log::debug_key("Terminal.init_ctx_menu");
        self.ctx_widget.clear();

        if EvtAct::is_ctx_menu_displayed_area(self, y, x) {
            let (y, x) = if y == USIZE_UNDEFINED {
                (
                    self.curt().editor.win_mgr.curt().cur.y - self.curt().editor.win_mgr.curt().offset.y + self.curt().editor.get_curt_row_posi(),
                    self.curt().editor.win_mgr.curt().cur.disp_x + self.curt().editor.get_rnw_and_margin(), //  if CfgEdit::get().general.editor.row_no.is_enable {  } else { self.curt().editor.win_mgr.curt().cur.disp_x },
                )
            } else {
                (y, x)
            };
            self.state.is_ctx_menu = true;
            EvtAct::set_ctx_menu_curt_term_place(self, y);

            let height = min(self.ctx_widget.widget.cont.cont_vec.len(), Editor::get_disp_row_num());
            self.ctx_widget.widget.set_parent_disp_area(y, x, height);
        } else if self.state.is_ctx_menu {
            self.state.is_ctx_menu = false;
        }
    }
    pub fn is_ctx_menu_keys(&mut self, keys: &Keys) -> bool {
        if self.state.is_ctx_menu {
            let rtn = match keys {
                Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) => true,
                Keys::MouseMove(_, _) => true,
                Keys::MouseDownLeft(y, x) => self.ctx_widget.widget.is_mouse_within_area(*y as usize, *x as usize),
                Keys::MouseDragRight(_, _) => true,
                _ => true,
            };
            return rtn;
        }
        return false;
    }
    pub fn clear_ctx_menu(&mut self) {
        Log::debug_key("clear_ctx_menu");
        self.state.is_ctx_menu = false;
        self.state.is_ctx_menu_hide_draw = true;
        self.ctx_widget.clear();
    }
}
