use crate::{explorer::explorer::*, sidebar::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{
    models::{draw::*, event::*, term::*},
    term::*,
};
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::{sidebar::*, term::*};
use ewin_view::traits::view::*;

impl SideBar {
    pub fn ctrl_sidebar(cmd_type: &CmdType) -> ActType {
        if let Some(mut sidebar) = SideBar::get_result() {
            Log::debug_key("SideBar::ctrl_sidebar");
            Log::debug("cmd_type", &cmd_type);

            let sidebar_state = State::get().sidebar;
            match cmd_type {
                CmdType::SwitchDispSideBar => {
                    let is_show = State::get().sidebar.is_show;
                    if !is_show {
                        let fullpath = &State::get().curt_ref_state().file.fullpath.clone();
                        sidebar.init(fullpath, true);
                    } else {
                        State::get().sidebar.is_show = false;
                    }
                    return ActType::Draw(DrawParts::All);
                }

                CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) if sidebar_state.resize != SideBarResizeState::None => match sidebar_state.resize {
                    SideBarResizeState::None => {}
                    SideBarResizeState::Start | SideBarResizeState::Resizing => {
                        let curt_width = CfgEdit::get().general.sidebar.width;
                        if matches!(cmd_type, CmdType::MouseDragLeftLeft(_, _)) {
                            if curt_width > SideBar::MINIMUM_WIDTH {
                                CfgEdit::get().general.sidebar.width -= 1;
                            }
                        } else {
                            let allowable_width = (get_term_size().0 as f32 * SideBar::MAXIMUM_WIDTH_PER_TERM).ceil() as usize;
                            if allowable_width > curt_width {
                                CfgEdit::get().general.sidebar.width += 1;
                            }
                        }
                        sidebar.cont.set_size();
                        sidebar.calc_scrlbar();

                        return ActType::Draw(DrawParts::All);
                    }
                },
                /*
                CmdType::MouseScrollUp => {
                    if sidebar.scrl_v.is_show && sidebar.cont.as_base().offset.y > 0 {
                        sidebar.cont.as_mut_base().offset.y -= 1;

                        sidebar.calc_scrlbar_v();

                        return ActType::Draw(DrawParts::SideBar);
                    }
                }
                CmdType::MouseScrollDown => {
                    if sidebar.scrl_v.is_show && sidebar.cont.as_base().offset.y + 1 + sidebar.cont.get_mut_cont_view().height <= sidebar.cont.get_cont_vec_len() {
                        sidebar.cont.as_mut_base().offset.y += 1;

                        sidebar.calc_scrlbar_v();
                        return ActType::Draw(DrawParts::SideBar);
                    }
                }
                 */
                CmdType::ShowSideBar => {
                    State::get().sidebar.is_show = true;
                    return ActType::Draw(DrawParts::All);
                }
                CmdType::HideSideBar => {
                    State::get().sidebar.is_show = false;
                    return ActType::Draw(DrawParts::All);
                }

                // split line
                CmdType::MouseDownLeft(_, x) if *x == sidebar.view().x + sidebar.view().width => {
                    State::get().sidebar.resize = SideBarResizeState::Start;
                }
                _ => {
                    if let Ok(tree_file_view) = sidebar.cont.downcast_mut::<Explorer>() {
                        let act_type = tree_file_view.ctrl_evt(cmd_type);

                        return act_type;
                    }
                }
            }
        }
        return ActType::Cancel;
    }

    pub fn judge_when_sidebar(keys: Keys, place_org: Place) -> bool {
        Log::debug_key("SideBar::judge_when_sidebar");
        Log::debug("keys", &keys);

        let sidebar_state = State::get().sidebar;

        if sidebar_state.is_show {
            let sidebar = SideBar::get();

            return match &keys {
                Keys::MouseDownLeft(y, x) if sidebar.view().y as u16 <= *y && *y <= sidebar.view().y_height() as u16 && sidebar.view().x as u16 <= *x && *x <= sidebar.view().x_width() as u16 => true,
                Keys::MouseScrollUp | Keys::MouseScrollDown if place_org == Place::SideBar => true,
                Keys::MouseDragLeft(_, _) | Keys::MouseUpLeft(_, _) if sidebar.cont.is_scrl_v_enable() | sidebar.scrl_h.is_enable => true,
                _ if sidebar_state.resize != SideBarResizeState::None => true,
                _ => false,
            };
        }
        return false;
    }
}
