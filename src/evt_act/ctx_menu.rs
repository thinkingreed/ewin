use crate::{_cfg::keys::KeyCmd, ctx_menu::ctx_menu::CtxMenuGroup, log::*, model::*, terminal::Terminal};

impl EvtAct {
    pub fn check_ctx_menu(term: &mut Terminal) -> EvtActType {
        Log::debug_key("check_ctx_menu");
        Log::debug("sel", &term.curt().editor.sel);

        match term.keycmd {
            KeyCmd::MouseDownRight(y, x) => {
                term.ctx_menu_group.clear();
                term.state.is_ctx_menu = true;
                CtxMenuGroup::set_curt_term_place(term, y);
                term.ctx_menu_group.set_parent_disp_area(y, x);
                return EvtActType::Hold;
            }
            KeyCmd::MouseDownLeft(y, x) => {
                if term.state.is_ctx_menu {
                    if term.ctx_menu_group.is_mouse_within_range(y, x) {
                        CtxMenuGroup::click_ctx_menu(term);
                        return EvtActType::DrawOnly;
                    }
                }
                return EvtActType::Hold;
            }
            KeyCmd::MouseMove(y, x) => {
                if term.state.is_ctx_menu && term.ctx_menu_group.is_mouse_within_range(y, x) {
                    term.ctx_menu_group.ctrl_mouse_move(y, x);
                    return EvtActType::Next;
                } else {
                    return EvtActType::None;
                }
            }
            _ => return EvtActType::Hold,
        }
    }
}
