use crate::term::*;
use crossterm::{cursor::MoveTo, execute};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{evt::*, file::CloseFileType, term::*},
    term::*,
};
use ewin_ctx_menu::ctx_menu::*;
use ewin_dialog::dialog::*;
use ewin_file_bar::filebar::*;
use ewin_key::{
    global::*,
    key::{cmd::*, keys::*},
};
use ewin_menulist::menubar::*;
use ewin_state::term::*;
use ewin_tabs::tabs::*;
use ewin_view::{menulists::core::*, view::*, view_traits::view_trait::*};
use std::io::{stdout, Write};

impl EvtAct {
    pub fn match_key<T: Write>(keys: Keys, out: &mut T, term: &mut Term) -> bool {
        // Pressed keys check
        let act_type = EvtAct::check_keys(keys, term);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        term.set_when(keys);
        // Support check for pressed keys
        let act_type = term.set_keys(&keys);
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        let act_type = EvtAct::exec_cmd(term);

        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        return false;
    }

    pub fn exec_cmd(term: &mut Term) -> ActType {
        View::hide_cur();
        // msg
        term.tabs.curt().msgbar.clear_mag();

        return match term.place {
            Place::Tabs => term.tabs.ctrl_tabs(&term.cmd.cmd_type),
            Place::Dialog => Dialog::ctrl_dialog(&term.cmd.cmd_type),
            Place::CtxMenu => CtxMenu::ctrl_ctx_menu(&term.cmd.cmd_type),
            Place::MenuBar => MenuBar::ctrl_menubar(term.cmd.cmd_type.clone()),
            Place::FileBar => FileBar::ctrl_filebar(&term.cmd.cmd_type, term.keys),
            Place::Editor => term.tabs.curt().editor.ctrl_editor(term.cmd.clone()),
            Place::StatusBar => Tabs::ctrl_statusbar(&mut term.tabs, &term.cmd.cmd_type),
            Place::Prom => term.tabs.ctrl_prom(&term.cmd),
        };
    }
    pub fn specify_cmd<T: Write>(term: &mut Term, out: &mut T, cmd_type: CmdType, when: Place, act_type_opt: Option<ActType>) -> bool {
        Log::debug_key("EvtAct::specify_cmd");
        Log::debug("cmd_type", &cmd_type);

        term.cmd = Cmd::to_cmd(cmd_type);
        term.place = when;

        let act_type_org = EvtAct::exec_cmd(term);
        let act_type = if let Some(specify_act_type) = act_type_opt { specify_act_type } else { act_type_org };
        if let Some(rtn) = EvtAct::check_next_process(out, term, act_type) {
            return rtn;
        }
        return false;
    }

    pub fn check_next_process<T: Write>(out: &mut T, term: &mut Term, act_type: ActType) -> Option<bool> {
        return match &act_type {
            ActType::Next => None,
            ActType::Draw(_) => {
                term.draw(out, &act_type);
                term.draw_cur(out);

                Some(false)
            }
            ActType::None => Some(false),
            ActType::Cancel => {
                term.draw_cur(out);
                Some(false)
            }
            ActType::Exit => Some(true),
            ActType::ExitMsg(s) => {
                Term::finalize();
                println!("{}", s);
                Term::exit();
                Some(true)
            }
        };
    }

    pub fn check_keys(keys: Keys, term: &mut Term) -> ActType {
        match keys {
            Keys::MouseMove(y, x) if MenuBar::get().is_menubar_displayed_area(y as usize, x as usize).0 => {}
            Keys::MouseMove(_, _) if MenuBar::get().on_mouse_idx != USIZE_UNDEFINED => {
                MenuBar::get().on_mouse_idx = USIZE_UNDEFINED;
                MenuBar::get().draw_only(&mut stdout());
            }
            Keys::MouseMove(_, _) if MenuBar::get().menulist.is_show || term.tabs.curt().editor.is_input_imple_mode(true) => {}
            Keys::MouseMove(_, _) if term.tabs.curt().is_prom_pulldown() => {}
            Keys::MouseMove(_, _) if CtxMenu::get().is_show => {}
            Keys::MouseMove(y, x) if Dialog::get().is_tgt_mouse_move(y as usize, x as usize) => {}
            Keys::MouseMove(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            Keys::MouseUpLeft(_, _) if FileBar::get().state.is_dragging => {}
            Keys::MouseUpLeft(_, _) => {
                term.cmd = Cmd::to_cmd(CmdType::Null);
                return ActType::None;
            }
            // Because the same key occurs multiple times
            // MouseDragLeft: in the case of Windows and Ubuntu.
            // Resize: in the case of Windows.
            Keys::MouseDragLeft(_, _) | Keys::Resize(_, _) if keys == term.keys_org => return ActType::None,
            Keys::Resize(_, _) => {
                set_term_size();
                View::clear_all();
                if View::check_displayable() {
                    term.set_bg_color();
                    State::get().term.is_displayable = true;
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
                Dialog::get().clear();
                CtxMenu::get().clear();
                term.tabs.curt().clear_curt_tab(true);
            }
        }
        return ActType::Next;
    }
}

/// Event action
#[derive(Debug, Clone)]
pub struct EvtAct {}
