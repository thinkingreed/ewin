use ewin_core::model::{CurDirection, EvtActType};

use crate::{ctx_menu::init::*, ewin_core::_cfg::keys::*, ewin_core::log::*, model::*, terminal::*};

impl EvtAct {
    pub fn check_ctx_menu(term: &mut Terminal) -> EvtActType {
        Log::debug_key("EvtAct.check_ctx_menu");

        match term.keycmd {
            KeyCmd::MouseDownRight(y, x) | KeyCmd::MouseDragRight(y, x) => {
                term.ctx_menu_group.clear();
                if CtxMenuGroup::is_ctx_menu_displayed_area(term, y, x) {
                    CtxMenuGroup::show_init(term, y, x);
                } else if term.state.is_ctx_menu {
                    term.state.is_ctx_menu = false;
                    return EvtActType::DrawOnly;
                }
                return EvtActType::Hold;
            }
            KeyCmd::MouseDownLeft(y, x) => {
                if term.state.is_ctx_menu {
                    if term.ctx_menu_group.is_mouse_within_range(y, x) {
                        CtxMenuGroup::select_ctx_menu(term);
                        return EvtActType::DrawOnly;
                    } else {
                        term.state.is_ctx_menu = false;
                        return EvtActType::Next;
                    }
                }
                return EvtActType::Hold;
            }
            KeyCmd::MouseMove(y, x) => {
                if term.state.is_ctx_menu && term.ctx_menu_group.is_mouse_within_range(y, x) {
                    term.ctx_menu_group.ctrl_mouse_move(y, x);
                    if !term.ctx_menu_group.is_menu_change() {
                        return EvtActType::None;
                    }
                    return EvtActType::Next;
                } else {
                    return EvtActType::None;
                }
            }
            KeyCmd::CursorDown | KeyCmd::CursorUp | KeyCmd::CursorRight | KeyCmd::CursorLeft => {
                if term.state.is_ctx_menu {
                    match term.keycmd {
                        KeyCmd::CursorDown => term.ctx_menu_group.cur_move(CurDirection::Down),
                        KeyCmd::CursorUp => term.ctx_menu_group.cur_move(CurDirection::Up),
                        KeyCmd::CursorRight => term.ctx_menu_group.cur_move(CurDirection::Right),
                        KeyCmd::CursorLeft => term.ctx_menu_group.cur_move(CurDirection::Left),
                        _ => {}
                    }
                    return EvtActType::DrawOnly;
                } else {
                    return EvtActType::Hold;
                }
            }
            KeyCmd::InsertLine => {
                if term.state.is_ctx_menu {
                    CtxMenuGroup::select_ctx_menu(term);
                    return EvtActType::DrawOnly;
                } else {
                    return EvtActType::Hold;
                }
            }
            _ => {
                term.ctx_menu_group.clear();
                term.state.is_ctx_menu = false;
                return EvtActType::Hold;
            }
        }
    }
}
