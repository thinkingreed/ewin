use crate::{sidebar::*, tree_file_view::tree::*};
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, event::*, term::*};
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::term::*;
use ewin_view::view_traits::view_trait::*;

impl SideBar {
    pub fn ctrl_sidebar(cmd_type: &CmdType) -> ActType {
        if let Some(mut sidebar) = SideBar::get_result() {
            Log::debug_key("SideBar::ctrl_sidebar");
            match cmd_type {
                CmdType::SwitchDispSideBar => {
                    let is_show = State::get().term.is_sidebar_show;
                    if !is_show {
                        let fullpath = &State::get().curt_state().file.fullpath.clone();
                        sidebar.init(fullpath, true);
                    } else {
                        State::get().term.is_sidebar_show = false;
                    }
                    return ActType::Draw(DrawParts::All);
                }
                _ => {
                    if let Ok(tree_file_view) = sidebar.cont.downcast_mut::<TreeFileView>() {
                        return tree_file_view.ctrl_evt(cmd_type);
                    }
                }
            }
        }
        return ActType::Cancel;
    }

    pub fn judge_when_sidebar(keys: Keys, place_org: Place) -> bool {
        Log::debug_key("SideBar::judge_when_sidebar");
        let view = SideBar::get().view();

        match &keys {
            Keys::MouseDownLeft(y, x) if view.y as u16 <= *y && *y <= (view.y + view.height) as u16 && view.x as u16 <= *x && *x <= (view.x + view.width) as u16 => return true,
            Keys::MouseScrollUp | Keys::MouseScrollDown if place_org == Place::SideBar => return true,
            _ => return false,
        }
    }
}
