use crate::term::*;
use crossterm::{cursor::MoveTo, execute};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, file::*, term::*},
    term::*,
};
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_editor::model::Editor;
use ewin_file_bar::filebar::*;
use ewin_key::{
    global::*,
    key::{cmd::*, keys::*},
    model::*,
};
use ewin_menu_bar::menubar::*;
use ewin_msg_bar::msgbar::*;
use ewin_prom::model::*;
use ewin_side_bar::sidebar::*;
use ewin_state::{sidebar::*, term::*};
use ewin_status_bar::statusbar::*;
use ewin_view::{menulists::core::*, view::*, view_traits::view_trait::*};
use std::io::{stdout, Write};

impl Term {
    pub fn match_key<T: Write>(&mut self, keys: Keys, out: &mut T) -> bool {
        // Pressed keys check
        let act_type = self.check_keys(keys);
        if let Some(rtn) = self.check_next_process(out, act_type) {
            return rtn;
        }
        self.set_place(keys);
        // Support check for pressed keys
        let act_type = self.set_keys(&keys);
        if let Some(rtn) = self.check_next_process(out, act_type) {
            return rtn;
        }
        let act_type = self.exec_cmd();
        if let Some(rtn) = self.check_next_process(out, act_type) {
            return rtn;
        }
        return false;
    }

    pub fn exec_cmd(&mut self) -> ActType {
        View::hide_cur();
        // msg
        MsgBar::get().clear_mag();
        Log::debug("self.place", &self.place);

        return match self.place {
            Place::Tabs => self.tabs.ctrl_tabs(&self.cmd.cmd_type),
            Place::Dialog => Dialog::ctrl_dialog(&self.cmd.cmd_type),
            Place::CtxMenu => CtxMenu::ctrl_ctx_menu(&self.cmd.cmd_type),
            Place::MenuBar => MenuBar::ctrl_menubar(&self.cmd.cmd_type),
            Place::FileBar => FileBar::ctrl_filebar(&self.cmd.cmd_type, self.keys),
            Place::Editor => self.tabs.curt().editor.ctrl_editor(self.cmd.clone()),
            Place::StatusBar => StatusBar::get().ctrl_statusbar(&self.cmd.cmd_type),
            Place::Prom => Prom::get().ctrl_prom(&self.cmd),
            Place::SideBar => SideBar::ctrl_sidebar(&self.cmd.cmd_type),
        };
    }
    pub fn specify_cmd<T: Write>(&mut self, out: &mut T, cmd_type: CmdType, when: Place, act_type_opt: Option<ActType>) -> bool {
        Log::debug_key("EvtAct::specify_cmd");
        Log::debug("cmd_type", &cmd_type);
        self.cmd = Cmd::to_cmd(cmd_type);
        self.place = when;

        let act_type_org = self.exec_cmd();
        let act_type = if let Some(specify_act_type) = act_type_opt { specify_act_type } else { act_type_org };
        if let Some(rtn) = self.check_next_process(out, act_type) {
            return rtn;
        }
        return false;
    }

    pub fn check_next_process<T: Write>(&mut self, out: &mut T, act_type: ActType) -> Option<bool> {
        return match &act_type {
            ActType::Next => None,
            ActType::Draw(_) => {
                self.draw(out, &act_type);
                self.draw_cur(out);
                Some(false)
            }
            ActType::None => Some(false),
            ActType::Cancel => {
                self.draw_cur(out);
                Some(false)
            }
            ActType::Exit => Some(true),
            ActType::ExitMsg(s) => {
                Log::error("err", s);
                println!("{}", s);
                Term::finalize();
                Term::exit();
                Some(true)
            }
        };
    }

    pub fn check_keys(&mut self, keys: Keys) -> ActType {
        match keys {
            Keys::MouseMove(y, x) if MenuBar::get().is_menubar_displayed_area(y as usize, x as usize).0 => {}
            Keys::MouseMove(_, _) if MenuBar::get().on_mouse_idx != USIZE_UNDEFINED => {
                MenuBar::get().on_mouse_idx = USIZE_UNDEFINED;
                MenuBar::get().draw_only(&mut stdout());
            }
            Keys::MouseMove(_, _) if MenuBar::get().menulist.is_show || self.tabs.curt().editor.is_input_imple_mode(true) => {}
            Keys::MouseMove(_, _) if Prom::is_prom_pulldown() => {}
            Keys::MouseMove(_, _) if CtxMenu::get().is_show => {}
            Keys::MouseMove(y, x) if Dialog::get().is_tgt_mouse_move(y as usize, x as usize) => {}
            Keys::MouseMove(y, x) => {
                self.set_place_mouse_move(y, x);
                self.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            Keys::MouseUpLeft(_, _) => {
                self.tabs.curt().editor.exec_mouse_up_left();
                SideBar::get().exec_mouse_up_left();
                FileBar::get().exec_mouse_up_left();
                self.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            // Because the same key occurs multiple times
            // MouseDragLeft: in the case of Windows and Ubuntu.
            // Resize: in the case of Windows.
            Keys::MouseDragLeft(_, _) | Keys::Resize(_, _) if keys == self.keys_org => return ActType::None,
            Keys::Resize(_, _) => {
                set_term_size();
                if View::check_displayable() {
                    self.set_bg_color();
                    State::get().term.is_displayable = true;
                    self.clear_state();
                    // self.set_size_init();
                    return ActType::Draw(DrawParts::All);
                } else {
                    State::get().term.is_displayable = false;
                    let _ = execute!(stdout(), MoveTo(0, 0));
                    println!("{}", &Lang::get().increase_height_width_terminal);
                    return ActType::None;
                }
            }
            _ => {}
        };
        if !State::get().term.is_displayable {
            return ActType::None;
        }

        // Judg whether keys are CloseFile
        if let Some(_keys) = CMD_TYPE_MAP.get().unwrap().get(&CmdType::CloseFileCurt(CloseFileType::Normal)) {
            if _keys == &keys {
                self.clear_state();
            }
        }
        return ActType::Next;
    }

    pub fn set_keys(&mut self, keys: &Keys) -> ActType {
        Log::debug_key("Term.set_keys");
        Log::debug("keys", &keys);
        Log::debug("keywhen", &self.place);
        self.keys = *keys;
        self.cmd = Cmd::keys_to_cmd(keys, Some(&self.keys_org), self.place);
        Log::debug("self.cmd before adjust", &self.cmd);

        if keys == &Cmd::cmd_to_keys(CmdType::Confirm) && self.place == Place::Editor && State::get().curt_state().prom == PromState::GrepResult {
            self.cmd = Cmd::to_cmd(CmdType::Confirm);
        }
        Log::debug("self.cmd after adjust", &self.cmd);

        if self.cmd.cmd_type == CmdType::Unsupported {
            return ActType::Draw(DrawParts::MsgBar(Lang::get().unsupported_operation.to_string()));
        }
        return ActType::Next;
    }

    pub fn clear_state(&mut self) {
        Log::debug_key("Term.clear_parts_state");

        Dialog::get().clear();
        CtxMenu::get().clear();
        self.tabs.curt().clear_curt_tab(true);
    }
}
