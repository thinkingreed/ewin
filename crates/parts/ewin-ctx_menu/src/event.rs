use crate::{ctx_menu::*, global::*};
use directories::BaseDirs;
use ewin_cfg::{global::*, lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{dialog::*, draw::*, event::*, model::*, term::*},
};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_plugin::plugin::*;
use ewin_view::menulists::core::*;

impl CtxMenu {
    pub fn ctrl_ctx_menu(cmd_type: &CmdType) -> ActType {
        Log::debug_key("CtxMenu.ctrl_ctx_menu");

        if let Some(mut ctx_menu) = CTX_MENU.get().unwrap().try_lock() {
            match cmd_type {
                CmdType::MouseDownLeft(y, x) => {
                    if ctx_menu.menulist.is_mouse_within_area(*y, *x) {
                        return ctx_menu.select_ctx_menu();
                    }
                    return ActType::Cancel;
                }
                CmdType::MouseMove(y, x) => {
                    if ctx_menu.menulist.is_mouse_within_area(*y, *x) {
                        Log::debug_key("ctx_menu.menulist.is_mouse_within_area");
                        let child_cont_org = &ctx_menu.menulist.cont.cont_vec.get(ctx_menu.menulist.parent_sel_y).and_then(|cont| cont.1.clone());
                        ctx_menu.menulist.ctrl_mouse_move(*y, *x);

                        if !ctx_menu.menulist.is_menu_change() {
                            return ActType::Cancel;
                        }
                        let child_cont = &ctx_menu.menulist.cont.cont_vec.get(ctx_menu.menulist.parent_sel_y).and_then(|cont| cont.1.clone());

                        // Only parent meun move || Only child meun move
                        if child_cont_org.is_none() && child_cont.is_none() || ctx_menu.menulist.parent_sel_y == ctx_menu.menulist.parent_sel_y_org && ctx_menu.menulist.child_sel_y != USIZE_UNDEFINED {
                            return ActType::Draw(DrawParts::CtxMenu);
                        } else {
                            return ActType::Draw(DrawParts::Absolute(ctx_menu.menulist.get_disp_range_y()));
                        }
                    } else if ctx_menu.menulist.is_mouse_area_around(*y, *x) {
                        ctx_menu.menulist.clear_select_menu();
                        return ActType::Draw(DrawParts::Absolute(ctx_menu.menulist.get_disp_range_y()));
                    } else {
                        return ActType::Cancel;
                    }
                }
                CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                    match cmd_type {
                        CmdType::CursorDown => ctx_menu.menulist.cur_move(Direction::Down),
                        CmdType::CursorUp => ctx_menu.menulist.cur_move(Direction::Up),
                        CmdType::CursorRight => ctx_menu.menulist.cur_move(Direction::Right),
                        CmdType::CursorLeft => ctx_menu.menulist.cur_move(Direction::Left),
                        _ => {}
                    }
                    if !ctx_menu.menulist.is_menu_change() {
                        return ActType::Cancel;
                    }
                    return ActType::Draw(DrawParts::Absolute(ctx_menu.menulist.get_disp_range_y()));
                }
                CmdType::Confirm => return ctx_menu.select_ctx_menu(),

                _ => return ActType::Cancel,
            }
        }
        return ActType::Cancel;
    }

    pub fn select_ctx_menu(&mut self) -> ActType {
        Log::debug_key("select_ctx_menu");
        if let Some((ctx_menu, _)) = self.menulist.get_curt_child() {
            return self.check_ctx_menu(&self.menulist.get_curt_parent().unwrap().0.name, &ctx_menu.name);
        } else if !self.menulist.is_exist_child_curt_parent() {
            if let Some((parent_ctx_menu, _)) = self.menulist.get_curt_parent() {
                return self.check_ctx_menu(&parent_ctx_menu.name, "");
            }
        }
        return ActType::Cancel;
    }

    pub fn check_ctx_menu(&mut self, parent_name: &str, child_name: &str) -> ActType {
        Log::debug_key("check_func");

        self.clear();
        if LANG_MAP.get(parent_name).is_some() {
            if &Lang::get().macros == LANG_MAP.get(parent_name).unwrap() {
                return self.exec_macro(child_name);
            } else if LANG_MAP.get(child_name).is_some() {
                return self.exec_ctx_menu(child_name);
            } else {
                return self.exec_ctx_menu(parent_name);
            }
        } else if LANG_MAP.get(child_name).is_some() {
            return self.exec_ctx_menu(child_name);
        }
        return ActType::None;
    }

    pub fn exec_macro(&mut self, func: &str) -> ActType {
        if let Some(base_dirs) = BaseDirs::new() {
            let full_path_str = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR).join(func);
            if full_path_str.exists() {
                self.clear();
                Macros::exec_js_macro(&full_path_str.to_string_lossy());
            } else {
                return ActType::Draw(DrawParts::MsgBar(Lang::get().file_not_found.clone()));
            }
        }
        return ActType::None;
    }
    pub fn exec_ctx_menu(&mut self, name: &str) -> ActType {
        Log::debug_key("CtxMenu::exec_ctx_menu_func");
        Log::debug("select name", &name);

        self.clear();

        match self.place {
            CtxMenuPlace::FileBar => {
                if let CtxMenuPlaceInfo::FileBar(file_bar) = self.place_info {
                    match &LANG_MAP[name] {
                        s if s == &Lang::get().close_other_than_this_tab => Job::send_cmd(CmdType::CloseOtherThanThisTab(file_bar.tgt_idx)),
                        s if s == &Lang::get().close => Job::send_cmd(CmdType::CloseFileTgt(file_bar.tgt_idx)),
                        s if s == &Lang::get().file_property => Job::send_cmd(CmdType::DialogShow(DialogContType::FileProp(file_bar.tgt_idx))),
                        _ => return ActType::None,
                    };
                }
            }
            CtxMenuPlace::Editor(_) => Job::send_cmd_str(name),
            CtxMenuPlace::None => {}
        }

        return ActType::None;
    }
}
