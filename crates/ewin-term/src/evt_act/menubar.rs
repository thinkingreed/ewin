use crate::{bar::menubar::*, model::*, terms::term::*};
use ewin_cfg::{global::*, lang::lang_cfg::*, log::*, model::modal::*};
use ewin_const::{def::*, model::*};
use ewin_dialog::{cont::cont::*, dialog::*};
use ewin_editor::model::*;
use ewin_key::{
    key::{cmd::*, keys::*},
    model::*,
    util::*,
};
use ewin_menulist::{core::*, parts::menubar::*};
use std::{cmp::min, io::stdout, ops::Range};

impl EvtAct {
    pub fn ctrl_menubar(term: &mut Term) -> ActType {
        Log::debug_key("EvtAct.ctrl_menubar");
        Log::debug("term.menubar.widget.cmd", &term.menubar.menulist.cmd);
        match term.menubar.menulist.cmd.cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                if y == term.menubar.row_posi {
                    let (is_select, i) = term.menubar.is_menubar_displayed_area(y, x);
                    if is_select {
                        term.clear_widget_other_than_menulist();

                        // When the same menu is pressed
                        if term.menubar.sel_idx == i && term.state.is_menubar_menulist {
                            term.clear_menulist_other_than_on_monuse();
                            return ActType::Draw(DParts::Absolute(term.menubar.menulist.curt.get_disp_range_y()));
                        } else {
                            term.menubar.sel_idx = i;
                            MenuBar::init_menubar(term, y);
                        }
                        return ActType::Draw(DParts::MenuWidget);
                    }
                } else if term.menubar.menulist.curt.is_mouse_within_area(y, x) {
                    return EvtAct::select_menuwidget(term);
                }
                term.state.is_menubar_menulist = false;
                term.menubar.menulist.curt.clear_select_menu();
                return ActType::Draw(DParts::All);
            }
            CmdType::MouseMove(y, x) => {
                if y == term.menubar.row_posi {
                    Log::debug("term.menubar.menu_vec", &term.menubar.menu_vec);
                    let (is_on_mouse, i) = term.menubar.is_menubar_displayed_area(y, x);
                    if is_on_mouse {
                        term.menubar.on_mouse_idx_org = term.menubar.on_mouse_idx;
                        term.menubar.on_mouse_idx = i;
                        if term.menubar.sel_idx != USIZE_UNDEFINED {
                            term.menubar.sel_idx = i;
                        }

                        if term.menubar.sel_idx != USIZE_UNDEFINED && term.menubar.is_menu_changed() {
                            MenuBar::init_menubar(term, y);
                            let range = term.menubar.menulist.curt.get_disp_range_y();
                            term.menubar.menulist.curt.clear_select_menu();
                            return ActType::Draw(DParts::Absolute(Range { start: term.menubar.row_posi, end: range.end }));
                        }

                        if term.menubar.is_menu_changed() {
                            return ActType::Draw(DParts::MenuBar);
                        }
                    }
                    return ActType::Cancel;
                } else if term.menubar.menulist.curt.is_mouse_within_area(y, x) {
                    let child_cont_org = &term.menubar.menulist.curt.cont.cont_vec.get(term.menubar.menulist.curt.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.menubar.menulist.curt.ctrl_mouse_move(y, x);

                    if !term.menubar.menulist.curt.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.menubar.menulist.curt.cont.cont_vec.get(term.menubar.menulist.curt.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move || Only child meun move
                    if child_cont_org.is_none() && child_cont.is_none() || term.menubar.menulist.curt.parent_sel_y == term.menubar.menulist.curt.parent_sel_y_org && term.menubar.menulist.curt.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Draw(DParts::MenuWidget);
                    } else {
                        return ActType::Draw(DParts::Absolute(term.menubar.menulist.curt.get_disp_range_y()));
                    }
                } else if term.menubar.menulist.curt.is_mouse_area_around(y, x) {
                    term.menubar.menulist.curt.clear_select_menu();
                    return ActType::Draw(DParts::Absolute(term.menubar.menulist.curt.get_disp_range_y()));
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                let direction = match term.menubar.menulist.cmd.cmd_type {
                    CmdType::CursorDown => Direction::Down,
                    CmdType::CursorUp => Direction::Up,
                    CmdType::CursorRight => Direction::Right,
                    CmdType::CursorLeft => Direction::Left,
                    _ => Direction::Down,
                };
                term.menubar.menulist.curt.cur_move(direction);

                return ActType::Draw(DParts::Absolute(term.menubar.menulist.curt.get_disp_range_y()));
            }
            CmdType::MenuWidget(_, _) => {
                // TODO
                return ActType::Draw(DParts::All);
            }
            CmdType::Confirm => return EvtAct::select_menuwidget(term),
            _ => return ActType::Cancel,
        }
    }

    pub fn select_menuwidget(term: &mut Term) -> ActType {
        Log::debug_key("select_menubar");
        let menubar_cont = &term.menubar.menu_vec[term.menubar.sel_idx].clone();
        Log::debug("menubar_cont", &menubar_cont);
        if let Some((child, _)) = term.menubar.menulist.curt.get_curt_child() {
            Log::debug("child", &child);
            return EvtAct::check_menubar_func(term, &menubar_cont.menunm, &term.menubar.menulist.curt.get_curt_parent().unwrap().0.name, &child.name);
        } else if !term.menubar.menulist.curt.is_exist_child_curt_parent() {
            if let Some((parent, _)) = term.menubar.menulist.curt.get_curt_parent() {
                Log::debug("parent", &parent);
                return EvtAct::check_menubar_func(term, &menubar_cont.menunm, &parent.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_menubar_func(term: &mut Term, parent_name: &str, child_name: &str, grandchild_name: &str) -> ActType {
        Log::debug_key("check_menubar_func");
        Log::debug("parent_name", &parent_name);
        Log::debug("child_name", &child_name);
        Log::debug("grandchild_name", &grandchild_name);

        term.clear_menulist_all();

        // Convert to set language
        let parent_name = get_cfg_lang_name(parent_name);
        let child_name = get_cfg_lang_name(child_name);
        let grandchild_name = get_cfg_lang_name(grandchild_name);

        // convert
        if child_name.contains(&Lang::get().convert) {
            return term.curt().editor.convert(ConvType::from_str_conv_type(grandchild_name));
            // format
        } else if child_name.contains(&Lang::get().format) {
            term.curt().editor.format(FileType::from_str_fmt_type(grandchild_name));
        }

        Log::debug("get_cfg_lang_name(parent_name).contains(&Lang::get().file)", &get_cfg_lang_name(parent_name).contains(&Lang::get().file));
        Log::debug("child_name.contains(&Lang::get().create_new_file)", &child_name.contains(&Lang::get().create_new_file));
        Log::debug("Lang::get().create_new_file", &Lang::get().create_new_file);
        Log::debug("get_cfg_lang_name(child_name)", &get_cfg_lang_name(child_name));

        // Other
        if EvtAct::contain_exec_menu(&Lang::get().about_app, child_name, grandchild_name) {
            Dialog::init(DialogContType::AboutApp)
        } else
        // parent_name
        if parent_name.contains(&Lang::get().file) {
            if child_name.contains(&Lang::get().encode) {
                EvtAct::ctrl_editor_cmd_type(CmdType::EncodingProm, term);
            } else if child_name.contains(&Lang::get().create_new_file) {
                EvtAct::ctrl_editor_cmd_type(CmdType::CreateNewFile, term);
            } else if child_name.contains(&Lang::get().open_file) {
                EvtAct::ctrl_editor_cmd_type(CmdType::openFileProm(OpenFileType::Normal), term);
            } else if child_name.contains(&Lang::get().save_as.to_string()) {
                term.curt().prom_show_com(&CmdType::SaveNewFile);
            } else if child_name.contains(&Lang::get().end_of_all_save) {
                let act_type = term.save_all_tab();
                if let ActType::Draw(_) = act_type {
                    return act_type;
                } else {
                    return ActType::Exit;
                }
            } else if child_name.contains(&Lang::get().file_property.to_string()) {
                Dialog::init(DialogContType::FileProp)
            }
            // edit
        } else if parent_name.contains(&Lang::get().edit) {
            if child_name.contains(&Lang::get().box_select) {
                term.curt().editor.box_select_mode();
            } else if term.curt().editor.win_mgr.curt().sel.is_selected() {
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
            if child_name.contains(&Lang::get().scale) {
                return EvtAct::ctrl_editor_cmd_type(CmdType::SwitchDispScale, term);
            } else if child_name.contains(&Lang::get().row_no) {
                return EvtAct::ctrl_editor_cmd_type(CmdType::SwitchDispRowNo, term);
            }
            // macros
        } else if parent_name.contains(&Lang::get().macros) && child_name.contains(&Lang::get().specify_file_and_exec_macro) {
            term.curt().prom_open_file(&OpenFileType::JsMacro);
            // window
        } else if parent_name.contains(&Lang::get().window) {
            EvtAct::ctrl_editor_cmd_type(CmdType::WindowSplit(if child_name.contains(&Lang::get().left_and_right_split) { WindowSplitType::Vertical } else { WindowSplitType::Horizontal }), term);
        }

        return ActType::Draw(DParts::All);
    }

    pub fn contain_exec_menu(tgt_name: &str, child_name: &str, grandchild_name: &str) -> bool {
        return child_name.contains(tgt_name) || grandchild_name.contains(tgt_name);
    }

    pub fn exec_menubar_func(term: &mut Term, name: &str) -> ActType {
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
                    let tab_idx = term.tab_idx;
                    term.curt().editor_draw_vec[tab_idx].clear();
                }
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().cut => {
                EvtAct::match_event(Cmd::cmd_to_keys(CmdType::Cut), &mut stdout(), term);
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().copy => {
                EvtAct::match_event(Cmd::cmd_to_keys(CmdType::Copy), &mut stdout(), term);
                term.curt().editor.win_mgr.curt().sel.clear();
            }
            s if s == &Lang::get().paste => {
                EvtAct::match_event(Cmd::cmd_to_keys(CmdType::InsertStr("".to_string())), &mut stdout(), term);
            }
            s if s == &Lang::get().all_select => {
                EvtAct::match_event(Cmd::cmd_to_keys(CmdType::AllSelect), &mut stdout(), term);
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

impl Term {
    pub fn clear_menulist_all(&mut self) {
        Log::debug_key("Term.clear_menulist_all");
        self.state.is_menubar_menulist = false;
        self.state.is_menuwidget_hide_draw = true;
        self.menubar.sel_idx = USIZE_UNDEFINED;
        self.menubar.on_mouse_idx = USIZE_UNDEFINED;
        self.menubar.on_mouse_idx_org = USIZE_UNDEFINED;
        self.menubar.menulist.clear();
    }
    pub fn clear_menulist_other_than_on_monuse(&mut self) {
        Log::debug_key("Term.clear_menulist_other_than_on_monuse");
        self.state.is_menubar_menulist = false;
        self.state.is_menuwidget_hide_draw = true;
        self.menubar.sel_idx = USIZE_UNDEFINED;
        self.menubar.menulist.clear();
    }

    pub fn is_menuwidget_keys(&mut self, keys: &Keys) -> bool {
        let rtn = match keys {
            Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) if self.state.is_menubar_menulist => true,
            Keys::MouseMove(y, _) if y == &(self.menubar.row_posi as u16) || self.state.is_menubar_menulist => true,
            Keys::MouseDownLeft(y, x) => {
                if y == &(self.menubar.row_posi as u16) && self.menubar.is_menubar_displayed_area(*y as usize, *x as usize).0 {
                    true
                } else {
                    self.menubar.menulist.curt.is_mouse_within_area(*y as usize, *x as usize)
                }
            }
            _ => false,
        };
        return rtn;
    }
}
impl MenuBar {
    pub fn init_menubar(term: &mut Term, y: usize) {
        Log::debug_key("Terminal.init_menubar");
        let menu = &term.menubar.menu_vec[term.menubar.sel_idx].clone();
        if menu.is_always_reset_name {
            Log::debug_key("is_always_reset_name");
            MenubarMenuList::set_disp_name(&menu.menunm, &mut term.menubar.menulist.menu_map[&menu.menunm]);
        }
        term.state.is_menubar_menulist = true;
        if term.menubar.menulist.curt.cont.x_area.0 != menu.area.start {
            term.menubar.menulist.curt.cont = term.menubar.menulist.menu_map[&menu.menunm].clone();

            Log::debug("term.menubar.widget.curt.cont", &term.menubar.menulist.curt.cont);

            let height = min(term.menubar.menulist.curt.cont.cont_vec.len(), Editor::get_disp_row_num());
            term.menubar.menulist.curt.set_parent_disp_area(y, menu.area.start, height);
            MenuBar::set_disable_menu(term);
        }
    }

    // TODO
    pub fn set_disable_menu(term: &mut Term) {}

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
