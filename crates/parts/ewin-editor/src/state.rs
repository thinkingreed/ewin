use crate::model::*;
use ewin_ctx_menu::ctx_menu::*;

impl Editor {
    pub fn is_disp_state_normal() -> bool {
        return !(CtxMenu::get().is_show);
    }
}
