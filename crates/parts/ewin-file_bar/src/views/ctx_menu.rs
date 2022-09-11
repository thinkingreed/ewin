use crate::filebar::*;
use ewin_cfg::log::Log;
use ewin_const::{def::*, models::term::*};
use ewin_ctx_menu::{ctx_menu::*, view_traits::view_trait::*};

impl ViewCtxMenuTrait for FileBar {
    fn is_tgt_ctx_menu(&mut self, y: usize, x: usize) -> bool {
        if y == self.view.y {
            for h_file in self.file_vec.iter() {
                if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 || h_file.close_area.0 <= x && x <= h_file.close_area.1 {
                    return true;
                }
            }
        } else {
            return true;
        }
        return false;
    }

    fn get_term_place(&mut self) -> CtxMenuPlace {
        CtxMenuPlace::FileBar
    }

    fn get_place_info(&mut self, _: usize, x: usize) -> CtxMenuPlaceInfo {
        Log::debug_key("FileBar.get_place_info");
        let mut ctx_menu_file_idx = USIZE_UNDEFINED;
        for (idx, h_file) in self.file_vec.iter().enumerate() {
            if !h_file.is_disp {
                continue;
            }
            if h_file.filenm_area.0 <= x && x <= h_file.filenm_area.1 {
                ctx_menu_file_idx = idx;
                break;
            };
        }
        return CtxMenuPlaceInfo::FileBar(CtxMenuFileBar { tgt_idx: ctx_menu_file_idx });
    }
}
