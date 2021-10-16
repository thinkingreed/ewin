use crate::{
    ctx_menu::init::*,
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        def::*,
        log::*,
        model::*,
    },
    model::*,
    terminal::*,
};

impl EvtAct {
    pub fn ctrl_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.check_ctx_menu");

        match term.ctx_menu_group.c_cmd {
            C_Cmd::MouseDownLeft(y, x) => {
                if term.state.is_ctx_menu {
                    if term.ctx_menu_group.is_mouse_within_range(y, x) {
                        return CtxMenuGroup::select_ctx_menu(term);
                    } else {
                        term.state.is_ctx_menu = false;
                        return ActType::Next;
                    }
                }
                return ActType::Next;
            }
            C_Cmd::MouseMove(y, x) => {
                if term.state.is_ctx_menu && term.ctx_menu_group.is_mouse_within_range(y, x) {
                    let child_cont_org = &term.ctx_menu_group.curt_cont.menu_vec.get(term.ctx_menu_group.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.ctx_menu_group.ctrl_mouse_move(y, x);

                    if !term.ctx_menu_group.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.ctx_menu_group.curt_cont.menu_vec.get(term.ctx_menu_group.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move
                    if child_cont_org.is_none() && child_cont.is_none() {
                        return ActType::Draw(DParts::CtxMenu);
                        // Only child meun move
                    } else if term.ctx_menu_group.parent_sel_y == term.ctx_menu_group.parent_sel_y_cache && term.ctx_menu_group.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Draw(DParts::CtxMenu);
                    } else if (child_cont_org.is_some() || child_cont.is_some()) && term.ctx_menu_group.child_sel_y == USIZE_UNDEFINED {
                        return ActType::Draw(DParts::Editor);
                    } else {
                        return ActType::Draw(DParts::Editor);
                    }
                } else {
                    return ActType::Cancel;
                }
            }
            C_Cmd::CursorDown | C_Cmd::CursorUp | C_Cmd::CursorRight | C_Cmd::CursorLeft => {
                if term.state.is_ctx_menu {
                    match term.ctx_menu_group.c_cmd {
                        C_Cmd::CursorDown => term.ctx_menu_group.cur_move(Direction::Down),
                        C_Cmd::CursorUp => term.ctx_menu_group.cur_move(Direction::Up),
                        C_Cmd::CursorRight => term.ctx_menu_group.cur_move(Direction::Right),
                        C_Cmd::CursorLeft => term.ctx_menu_group.cur_move(Direction::Left),
                        _ => {}
                    }
                    return ActType::Draw(DParts::All);
                } else {
                    return ActType::Next;
                }
            }
            C_Cmd::ConfirmCtxMenu => {
                if term.state.is_ctx_menu {
                    CtxMenuGroup::select_ctx_menu(term);
                    return ActType::Draw(DParts::All);
                } else {
                    return ActType::Next;
                }
            }
            C_Cmd::Null => {
                term.ctx_menu_group.clear();
                term.state.is_ctx_menu = false;
                return ActType::Next;
            }
        }
    }

    pub fn is_ctrl_ctx_keys(keys: &Keys, term: &mut Terminal) -> bool {
        if term.state.is_ctx_menu {
            let rtn = match keys {
                Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) => true,
                Keys::MouseMove(y, x) | Keys::MouseDownLeft(y, x) => {
                    if term.ctx_menu_group.is_mouse_within_range(*y as usize, *x as usize) {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            return rtn;
        }
        return false;
    }
}
