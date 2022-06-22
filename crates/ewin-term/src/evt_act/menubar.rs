use crate::{
    bar::menubar::*,
    ewin_com::{_cfg::key::keys::*, model::*},
    model::*,
};
use ewin_cfg::{global::*, lang::lang_cfg::*, log::*, model::modal::*};
use ewin_com::{
    _cfg::key::{cmd::*, keybind::*},
    util::*,
};
use ewin_const::def::*;
use ewin_editor::model::*;
use ewin_widget::{core::*, widget::menubar::*};
use std::{cmp::min, io::stdout, ops::Range};

impl EvtAct {
    pub fn ctrl_menubar(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_menubar");
        Log::debug("term.menubar.widget.cmd", &term.menubar.widget.cmd);
        match term.menubar.widget.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if y == term.menubar.row_posi {
                    let (is_select, i) = term.menubar.is_menubar_displayed_area(y, x);
                    if is_select {
                        term.menubar.sel_idx = i;
                        MenuBar::init_menubar(term, y);
                        return ActType::Draw(DParts::MenuWidget);
                    }
                } else if term.menubar.widget.curt.is_mouse_within_area(y, x) {
                    return EvtAct::select_menuwidget(term);
                }
                term.state.is_menuwidget = false;
                term.menubar.widget.curt.clear_select_menu();
                return ActType::Draw(DParts::All);
            }
            CmdType::MouseMove(y, x) => {
                if y == term.menubar.row_posi {
                    Log::debug("term.menubar.menu_vec", &term.menubar.menu_vec);
                    let (is_on_mouse, i) = term.menubar.is_menubar_displayed_area(y, x);
                    if is_on_mouse {
                        Log::debug(" term.menubar.on_mouse_idx 111", &term.menubar.on_mouse_idx);
                        term.menubar.on_mouse_idx_org = term.menubar.on_mouse_idx;
                        term.menubar.on_mouse_idx = i;
                        Log::debug(" term.menubar.on_mouse_idx 222", &term.menubar.on_mouse_idx);
                        if term.menubar.sel_idx != USIZE_UNDEFINED {
                            term.menubar.sel_idx = i;
                        }
                        Log::debug("term.menubar.on_mouse_idx", &term.menubar.on_mouse_idx);
                        Log::debug("term.menubar.on_mouse_idx_org", &term.menubar.on_mouse_idx_org);
                        if term.menubar.sel_idx != USIZE_UNDEFINED && term.menubar.is_menu_changed() {
                            Log::debug_s("menu_changed");
                            MenuBar::init_menubar(term, y);
                            let range = term.menubar.widget.curt.get_disp_range_y();
                            return ActType::Draw(DParts::Absolute(Range { start: term.menubar.row_posi, end: range.end }));
                        }

                        if term.menubar.is_menu_changed() {
                            return ActType::Draw(DParts::MenuBar);
                        }
                    }
                    return ActType::Cancel;
                } else if term.menubar.widget.curt.is_mouse_within_area(y, x) {
                    let child_cont_org = &term.menubar.widget.curt.cont.cont_vec.get(term.menubar.widget.curt.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.menubar.widget.curt.ctrl_mouse_move(y, x);

                    if !term.menubar.widget.curt.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.menubar.widget.curt.cont.cont_vec.get(term.menubar.widget.curt.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move || Only child meun move
                    if child_cont_org.is_none() && child_cont.is_none() || term.menubar.widget.curt.parent_sel_y == term.menubar.widget.curt.parent_sel_y_org && term.menubar.widget.curt.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Draw(DParts::MenuWidget);
                    } else {
                        return ActType::Draw(DParts::Absolute(term.menubar.widget.curt.get_disp_range_y()));
                    }
                } else if term.menubar.widget.curt.is_mouse_area_around(y, x) {
                    term.menubar.widget.curt.clear_select_menu();
                    return ActType::Draw(DParts::Absolute(term.menubar.widget.curt.get_disp_range_y()));
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                let direction = match term.menubar.widget.cmd.cmd_type {
                    CmdType::CursorDown => Direction::Down,
                    CmdType::CursorUp => Direction::Up,
                    CmdType::CursorRight => Direction::Right,
                    CmdType::CursorLeft => Direction::Left,
                    _ => Direction::Down,
                };
                term.menubar.widget.curt.cur_move(direction);

                return ActType::Draw(DParts::Absolute(term.menubar.widget.curt.get_disp_range_y()));
            }
            CmdType::MenuWidget(_, _) => {
                // TODO
                return ActType::Draw(DParts::All);
            }
            CmdType::Confirm => return EvtAct::select_menuwidget(term),
            _ => return ActType::Cancel,
        }
    }

    pub fn select_menuwidget(term: &mut Terminal) -> ActType {
        Log::debug_key("select_menubar");
        let menubar_cont = &term.menubar.menu_vec[term.menubar.sel_idx].clone();
        Log::debug("menubar_cont", &menubar_cont);
        if let Some((child, _)) = term.menubar.widget.curt.get_curt_child() {
            Log::debug("child", &child);
            return EvtAct::check_menubar_func(term, &menubar_cont.menunm, &term.menubar.widget.curt.get_curt_parent().unwrap().0.name, &child.name);
        } else if !term.menubar.widget.curt.is_exist_child_curt_parent() {
            if let Some((parent, _)) = term.menubar.widget.curt.get_curt_parent() {
                Log::debug("parent", &parent);
                return EvtAct::check_menubar_func(term, &menubar_cont.menunm, &parent.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_menubar_func(term: &mut Terminal, parent_name: &str, child_name: &str, grandchild_name: &str) -> ActType {
        Log::debug_key("check_menubar_func");
        Log::debug("parent_name", &parent_name);
        Log::debug("child_name", &child_name);

        term.clear_menuwidget();

        // Convert to set language
        let parent_name = get_cfg_lang_name(parent_name);
        let child_name = get_cfg_lang_name(child_name);

        // grandchild_name
        // convert
        if child_name.contains(&Lang::get().convert) {
            return term.curt().editor.convert(ConvType::from_str_conv_type(grandchild_name));
            // format
        } else if child_name.contains(&Lang::get().format) {
            term.curt().editor.format(FileType::from_str_fmt_type(grandchild_name));
        }

        // parent_name
        if get_cfg_lang_name(parent_name).contains(&Lang::get().file) {
            if child_name.contains(&Lang::get().encode) {
                term.curt().editor.cmd = Cmd::to_cmd(CmdType::EncodingProm);
                return EvtAct::ctrl_editor(term);
                // EvtAct::match_event(Keybind::keycmd_to_keys(&KeyCmd::Edit(E_Cmd::Encoding)), &mut stdout(), term);
            } else if child_name.contains(&Lang::get().create_new_file) {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::CreateNewFile), &mut stdout(), term);
            } else if child_name.contains(&Lang::get().open_file) {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::openFileProm(OpenFileType::Normal)), &mut stdout(), term);
            } else if child_name.contains(&Lang::get().save_as.to_string()) {
                term.curt().prom_show_com(&CmdType::SaveNewFile);
            } else if child_name.contains(&Lang::get().end_of_all_save) {
                let act_type = term.save_all_tab();
                if let ActType::Draw(_) = act_type {
                    return act_type;
                } else {
                    return ActType::Exit;
                }
            }
            // edit
        } else if parent_name.contains(&Lang::get().edit) {
            if child_name.contains(&Lang::get().box_select) {
                term.curt().editor.box_select_mode();
            } else if term.curt().editor.sel.is_selected() {
                if child_name.contains(&Lang::get().convert) {
                    term.curt().editor.convert(ConvType::from_str_conv_type(grandchild_name));
                } else if child_name.contains(&Lang::get().format) {
                    term.curt().editor.format(FileType::from_str_fmt_type(grandchild_name));
                }
            } else {
                term.curt().clear_curt_tab(true);
                return ActType::Draw(DParts::AllMsgBar(Lang::get().no_sel_range.to_string()));
            }
            // display
        } else if parent_name.contains(&Lang::get().display) {
            Log::debug_key("11111111111111111111111111111111111");
            if child_name.contains(&Lang::get().scale) {
                Log::debug_key("222222222222222222222222222222222");
                let cmd = Cmd::to_cmd(CmdType::SwitchDispScale);
                term.cmd = cmd;
                return EvtAct::ctrl_editor(term);
            }
            // macros
        } else if parent_name.contains(&Lang::get().macros) && child_name.contains(&Lang::get().specify_file_and_exec_macro) {
            term.curt().prom_open_file(&OpenFileType::JsMacro);
        }

        /*
        // child_name
        if LANG_MAP.contains_key(&child_name.to_case(Case::Snake)) {
            Log::debug_s("LANG_MAP.contains_key");
            let cmd = Cmd::str_to_cmd(child_name);
            Log::debug("cmd", &cmd);
            term.cmd = cmd;
            return EvtAct::ctrl_editor(term);
        }
         */

        return ActType::Draw(DParts::All);
    }

    pub fn exec_menubar_func(term: &mut Terminal, name: &str) -> ActType {
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
                    return ActType::Draw(DParts::AllMsgBar(err_str));
                } else {
                    // highlight data reset
                    term.editor_draw_vec[term.tab_idx].clear();
                }
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().cut => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::Cut), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().copy => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::Copy), &mut stdout(), term);
                term.curt().editor.sel.clear();
            }
            s if s == &Lang::get().paste => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::InsertStr("".to_string())), &mut stdout(), term);
            }
            s if s == &Lang::get().all_select => {
                EvtAct::match_event(Keybind::cmd_to_keys(&CmdType::AllSelect), &mut stdout(), term);
            }
            //// headerbar
            // close
            s if s == &Lang::get().close => return term.curt().prom_show_com(&CmdType::CloseFile),
            // close other than this tab
            s if s == &Lang::get().close_other_than_this_tab => return term.close_tabs(term.tab_idx),
            _ => {}
        };
        return ActType::Draw(DParts::Editor(E_DrawRange::All));
    }
}

impl Terminal {
    pub fn clear_menuwidget(&mut self) {
        Log::debug_key("clear_menuwidget");
        self.state.is_menuwidget = false;
        self.state.is_menuwidget_hide_draw = true;
        self.menubar.sel_idx = USIZE_UNDEFINED;
        self.menubar.on_mouse_idx = USIZE_UNDEFINED;
        self.menubar.on_mouse_idx_org = USIZE_UNDEFINED;
        self.menubar.widget.clear();
    }

    pub fn is_menuwidget_keys(&mut self, keys: &Keys) -> bool {
        let rtn = match keys {
            Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) if self.state.is_menuwidget => true,
            Keys::MouseMove(y, _) if y == &(self.menubar.row_posi as u16) || self.state.is_menuwidget => true,
            Keys::MouseDownLeft(y, x) => {
                if y == &(self.menubar.row_posi as u16) && self.menubar.is_menubar_displayed_area(*y as usize, *x as usize).0 {
                    true
                } else {
                    self.menubar.widget.curt.is_mouse_within_area(*y as usize, *x as usize)
                }
            }
            _ => false,
        };
        return rtn;
    }
}
impl MenuBar {
    pub fn init_menubar(term: &mut Terminal, y: usize) {
        Log::debug_key("Terminal.init_menubar");
        let menu = &term.menubar.menu_vec[term.menubar.sel_idx].clone();
        if menu.is_always_reset_name {
            MenubarWidget::set_disp_name(&menu.menunm, &mut term.menubar.widget.menu_map[&menu.menunm]);
        }
        term.state.is_menuwidget = true;
        if term.menubar.widget.curt.cont.x_area.0 != menu.area.start {
            term.menubar.widget.curt.cont = term.menubar.widget.menu_map[&menu.menunm].clone();

            Log::debug("term.menubar.widget.curt.cont", &term.menubar.widget.curt.cont);

            let height = min(term.menubar.widget.curt.cont.cont_vec.len(), Editor::get_disp_row_num());
            term.menubar.widget.curt.set_parent_disp_area(y, menu.area.start, height);
            MenuBar::set_disable_menu(term);
        }
    }

    // TODO
    pub fn set_disable_menu(term: &mut Terminal) {}

    pub fn is_menubar_displayed_area(&mut self, y: usize, x: usize) -> (bool, usize) {
        if y == self.row_posi {
            for (i, menu) in self.menu_vec.iter().enumerate() {
                if menu.area.contains(&x) {
                    return (true, i);
                }
            }
        }
        return (false, USIZE_UNDEFINED);
    }

    pub fn is_menu_changed(&mut self) -> bool {
        self.on_mouse_idx != self.on_mouse_idx_org
    }
}
