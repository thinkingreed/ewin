use crate::{menubar::*, parts::menubar::*};
use ewin_cfg::log::*;
use ewin_const::{
    def::*,
    models::{draw::*, evt::*, model::*},
    term::*,
};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_view::menulists::core::*;
use std::{cmp::min, ops::Range};

impl MenuBar {
    pub fn ctrl_menubar(cmd_type: CmdType) -> ActType {
        if let Ok(mut menubar) = MenuBar::get_result() {
            Log::debug_key("EvtAct.ctrl_menubar");
            match cmd_type {
                CmdType::MouseDownLeft(y, x) => {
                    if y == menubar.row_posi {
                        let (is_select, i) = menubar.is_menubar_displayed_area(y, x);
                        if is_select {
                            // When the same menu is pressed
                            if menubar.sel_idx == i && menubar.menulist.is_show {
                                let range = menubar.menulist.curt.get_disp_range_y();
                                menubar.clear_menulist_other_than_on_monuse();
                                return ActType::Draw(DParts::Absolute(range));
                            } else {
                                menubar.sel_idx = i;
                                menubar.init_menubar(y);
                            }
                            return ActType::Draw(DParts::MenuBarMenuList);
                        }
                    } else if menubar.menulist.curt.is_mouse_within_area(y, x) {
                        return menubar.select_menulist();
                    }
                    menubar.menulist.is_show = false;
                    menubar.menulist.curt.clear_select_menu();
                    return ActType::Draw(DParts::All);
                }
                CmdType::MouseMove(y, x) => {
                    if y == menubar.row_posi {
                        Log::debug("menubar.menu_vec", &menubar.menu_vec);
                        let (is_on_mouse, i) = menubar.is_menubar_displayed_area(y, x);
                        if is_on_mouse {
                            menubar.on_mouse_idx_org = menubar.on_mouse_idx;
                            menubar.on_mouse_idx = i;
                            if menubar.sel_idx != USIZE_UNDEFINED {
                                menubar.sel_idx = i;
                            }

                            if menubar.sel_idx != USIZE_UNDEFINED && menubar.is_menu_changed() {
                                menubar.init_menubar(y);
                                let range = menubar.menulist.curt.get_disp_range_y();
                                menubar.menulist.curt.clear_select_menu();
                                return ActType::Draw(DParts::Absolute(Range { start: menubar.row_posi, end: range.end }));
                            }

                            if menubar.is_menu_changed() {
                                return ActType::Draw(DParts::MenuBar);
                            }
                        }
                        return ActType::Cancel;
                    } else if menubar.menulist.curt.is_mouse_within_area(y, x) {
                        let child_cont_org = &menubar.menulist.curt.cont.cont_vec.get(menubar.menulist.curt.parent_sel_y).and_then(|cont| cont.1.clone());
                        menubar.menulist.curt.ctrl_mouse_move(y, x);

                        if !menubar.menulist.curt.is_menu_change() {
                            return ActType::Cancel;
                        }
                        let child_cont = &menubar.menulist.curt.cont.cont_vec.get(menubar.menulist.curt.parent_sel_y).and_then(|cont| cont.1.clone());

                        // Only parent meun move || Only child meun move
                        if child_cont_org.is_none() && child_cont.is_none() || menubar.menulist.curt.parent_sel_y == menubar.menulist.curt.parent_sel_y_org && menubar.menulist.curt.child_sel_y != USIZE_UNDEFINED {
                            return ActType::Draw(DParts::MenuBarMenuList);
                        } else {
                            return ActType::Draw(DParts::Absolute(menubar.menulist.curt.get_disp_range_y()));
                        }
                    } else if menubar.menulist.curt.is_mouse_area_around(y, x) {
                        menubar.menulist.curt.clear_select_menu();
                        return ActType::Draw(DParts::Absolute(menubar.menulist.curt.get_disp_range_y()));
                    } else {
                        return ActType::Cancel;
                    }
                }
                CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                    let direction = match cmd_type {
                        CmdType::CursorDown => Direction::Down,
                        CmdType::CursorUp => Direction::Up,
                        CmdType::CursorRight => Direction::Right,
                        CmdType::CursorLeft => Direction::Left,
                        _ => Direction::Down,
                    };
                    menubar.menulist.curt.cur_move(direction);

                    return ActType::Draw(DParts::Absolute(menubar.menulist.curt.get_disp_range_y()));
                }
                CmdType::MenuBarMenulist(_, _) => {
                    // TODO
                    // TODO
                    // TODO
                    return ActType::Draw(DParts::All);
                }
                CmdType::Confirm => return menubar.select_menulist(),
                _ => return ActType::Cancel,
            }
        }
        return ActType::Cancel;
    }

    pub fn select_menulist(&mut self) -> ActType {
        Log::debug_key("select_menulist");
        let menubar_cont = &self.menu_vec[self.sel_idx].clone();
        Log::debug("menubar_cont", &menubar_cont);
        if let Some((child, _)) = self.menulist.curt.get_curt_child() {
            Log::debug("child", &child);
            return self.check_menubar_func(&menubar_cont.menunm, &self.menulist.curt.get_curt_parent().unwrap().0.name, &child.name);
        } else if !self.menulist.curt.is_exist_child_curt_parent() {
            if let Some((parent, _)) = self.menulist.curt.get_curt_parent() {
                Log::debug("parent", &parent);
                return self.check_menubar_func(&menubar_cont.menunm, &parent.name, "");
            }
        }
        return ActType::Cancel;
    }
    pub fn check_menubar_func(&mut self, parent_name: &str, child_name: &str, grandchild_name: &str) -> ActType {
        Log::debug_key("check_menubar_func");
        Log::debug("parent_name", &parent_name);
        Log::debug("child_name", &child_name);
        Log::debug("grandchild_name", &grandchild_name);

        self.clear_menulist_all();

        let func_name = if grandchild_name.is_empty() { child_name } else { grandchild_name };
        Log::debug("func_name", &func_name);

        Job::send_cmd_str(func_name);

        return ActType::Draw(DParts::All);
    }

    pub fn contain_exec_menu(tgt_name: &str, child_name: &str, grandchild_name: &str) -> bool {
        return child_name.contains(tgt_name) || grandchild_name.contains(tgt_name);
    }

    pub fn clear_menulist_all(&mut self) {
        Log::debug_key("Term.clear_menulist_all");
        self.sel_idx = USIZE_UNDEFINED;
        self.on_mouse_idx = USIZE_UNDEFINED;
        self.on_mouse_idx_org = USIZE_UNDEFINED;
        self.menulist.clear();
    }

    pub fn clear_menulist_other_than_on_monuse(&mut self) {
        Log::debug_key("Term.clear_menulist_other_than_on_monuse");
        self.sel_idx = USIZE_UNDEFINED;
        self.menulist.clear();
    }

    pub fn init_menubar(&mut self, y: usize) {
        Log::debug_key("Terminal.init_menubar");
        let menu = &self.menu_vec[self.sel_idx].clone();
        if menu.is_always_reset_name {
            Log::debug_key("is_always_reset_name");
            MenubarMenuList::set_disp_name(&menu.menunm, &mut self.menulist.menu_map[&menu.menunm]);
        }
        self.menulist.is_show = true;
        if self.menulist.curt.cont.x_area.0 != menu.area.start {
            self.menulist.curt.cont = self.menulist.menu_map[&menu.menunm].clone();

            Log::debug("self.widget.curt.cont", &self.menulist.curt.cont);

            let height = min(self.menulist.curt.cont.cont_vec.len(), get_term_size().1 - MENUBAR_ROW_NUM);
            self.menulist.curt.set_parent_disp_area(y, menu.area.start, height);
            MenuBar::set_disable_menu();
        }
    }

    // TODO
    pub fn set_disable_menu() {}

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
