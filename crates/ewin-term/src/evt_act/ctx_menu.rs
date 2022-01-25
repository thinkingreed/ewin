use crate::{
    ctx_menu::init::*,
    ewin_com::{
        _cfg::key::{keycmd::*, keys::*},
        def::*,
        log::*,
        model::*,
    },
    model::*,
};

impl EvtAct {
    pub fn ctrl_ctx_menu(term: &mut Terminal) -> ActType {
        Log::debug_key("EvtAct.ctrl_ctx_menu");

        match term.ctx_menu_group.c_cmd {
            C_Cmd::MouseDownLeft(y, x) => {
                if term.ctx_menu_group.is_mouse_within_range(y, x, false) {
                    return CtxMenuGroup::select_ctx_menu(term);
                }
                return ActType::Cancel;
            }
            C_Cmd::MouseMove(y, x) => {
                if term.ctx_menu_group.is_mouse_within_range(y, x, false) {
                    let child_cont_org = &term.ctx_menu_group.curt_cont.menu_vec.get(term.ctx_menu_group.parent_sel_y).and_then(|cont| cont.1.clone());
                    term.ctx_menu_group.ctrl_mouse_move(y, x);

                    if !term.ctx_menu_group.is_menu_change() {
                        return ActType::Cancel;
                    }
                    let child_cont = &term.ctx_menu_group.curt_cont.menu_vec.get(term.ctx_menu_group.parent_sel_y).and_then(|cont| cont.1.clone());

                    // Only parent meun move || Only child meun move
                    if child_cont_org.is_none() && child_cont.is_none() || term.ctx_menu_group.parent_sel_y == term.ctx_menu_group.parent_sel_y_cache && term.ctx_menu_group.child_sel_y != USIZE_UNDEFINED {
                        return ActType::Draw(DParts::CtxMenu);
                    } else {
                        term.set_render_range_ctx_menu();
                        return ActType::Draw(DParts::Editor);
                    }
                } else if term.ctx_menu_group.is_mouse_within_range(y, x, true) {
                    term.ctx_menu_group.clear_select_menu();
                    term.set_render_range_ctx_menu();
                    return ActType::Draw(DParts::Editor);
                } else {
                    return ActType::Cancel;
                }
            }
            C_Cmd::CursorDown | C_Cmd::CursorUp | C_Cmd::CursorRight | C_Cmd::CursorLeft => {
                match term.ctx_menu_group.c_cmd {
                    C_Cmd::CursorDown => term.ctx_menu_group.cur_move(Direction::Down),
                    C_Cmd::CursorUp => term.ctx_menu_group.cur_move(Direction::Up),
                    C_Cmd::CursorRight => term.ctx_menu_group.cur_move(Direction::Right),
                    C_Cmd::CursorLeft => term.ctx_menu_group.cur_move(Direction::Left),
                    _ => {}
                }
                term.set_render_range_ctx_menu();
                return ActType::Draw(DParts::Editor);
            }
            C_Cmd::CtxMenu(y, x) => {
                CtxMenuGroup::show_init(term, y, x);
                return ActType::Draw(DParts::All);
            }
            C_Cmd::ConfirmCtxMenu => {
                CtxMenuGroup::select_ctx_menu(term);
                return ActType::Draw(DParts::All);
            }
            C_Cmd::Null => return ActType::Cancel,
        }
    }

    pub fn is_ctrl_ctx_keys(keys: &Keys, term: &mut Terminal) -> bool {
        if term.state.is_ctx_menu {
            let rtn = match keys {
                Keys::Raw(Key::Left) | Keys::Raw(Key::Right) | Keys::Raw(Key::Up) | Keys::Raw(Key::Down) => true,
                Keys::MouseMove(_, _) => true,
                Keys::MouseDownLeft(y, x) => term.ctx_menu_group.is_mouse_within_range(*y as usize, *x as usize, false),
                Keys::MouseDragRight(_, _) => true,
                _ => true,
            };
            return rtn;
        }
        return false;
    }
}
