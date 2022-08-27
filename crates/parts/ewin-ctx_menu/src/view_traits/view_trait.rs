use crate::{ctx_menu::*, global::*};
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_cfg::log::*;
use ewin_const::{
    models::{draw::*, evt::*, term::*},
    term::*,
};
use ewin_view::menulists::core::*;
use std::cmp::min;

pub trait ViewCtxMenuTrait: DynClone + Any + 'static + std::fmt::Debug {
    fn init_ctx_menu(&mut self, y: usize, x: usize) -> ActType {
        Log::debug_key("Editor.init_ctx_menu");

        if let Ok(mut ctx_menu) = CTX_MENU.get().unwrap().try_lock() {
            ctx_menu.clear();

            if self.is_tgt_ctx_menu(y, x) {
                /*
                let (y, x) = if y == USIZE_UNDEFINED {
                    (
                        self.win_mgr.curt().cur.y - self.win_mgr.curt().offset.y + self.get_curt_row_posi(),
                        self.win_mgr.curt().cur.disp_x + self.get_rnw_and_margin(), //  if CfgEdit::get().general.editor.row_no.is_enable {  } else { self.tabs.curt().editor.win_mgr.curt().cur.disp_x },
                    )
                } else {
                    (y, x)
                };
                 */
                ctx_menu.is_show = true;
                Log::debug("ctx_menu.is_show", &ctx_menu.is_show);

                ctx_menu.place = self.get_term_place();
                ctx_menu.menulist.cont = ctx_menu.place_menulist_map[&ctx_menu.place].clone();

                ctx_menu.place_info = self.get_place_info(y, x);

                let height = min(ctx_menu.menulist.cont.cont_vec.len(), get_term_size().1);
                ctx_menu.menulist.set_parent_disp_area(y, x, height);
            } else if ctx_menu.is_show {
                ctx_menu.is_show = false;
            }
            return ActType::Draw(DParts::All);
        }

        return ActType::None;
    }

    fn is_tgt_ctx_menu(&mut self, y: usize, x: usize) -> bool;

    fn get_term_place(&mut self) -> CtxMenuPlace;
    fn get_place_info(&mut self, y: usize, x: usize) -> CtxMenuPlaceInfo;
}

downcast!(dyn ViewCtxMenuTrait);
dyn_clone::clone_trait_object!(ViewCtxMenuTrait);
