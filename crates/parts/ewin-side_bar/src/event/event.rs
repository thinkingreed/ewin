use crate::{sidebar::*, tree_file_view::tree::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{
    models::{draw::*, event::*, term::*},
    term::get_term_size,
};
use ewin_key::key::{cmd::*, keys::*};
use ewin_state::{sidebar::*, term::*};
use ewin_view::view_traits::view_trait::*;

impl SideBar {
    pub fn ctrl_sidebar(cmd_type: &CmdType) -> ActType {
        if let Some(mut sidebar) = SideBar::get_result() {
            Log::debug_key("SideBar::ctrl_sidebar");
            Log::debug("sidebar.scrl_v.view", &sidebar.scrl_v.view);

            let sidebar_state = State::get().sidebar;
            match cmd_type {
                CmdType::SwitchDispSideBar => {
                    let is_show = State::get().sidebar.is_show;
                    if !is_show {
                        let fullpath = &State::get().curt_state().file.fullpath.clone();
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
                        sidebar.cont.resize();
                        sidebar.judge_show_scrollbar();
                        return ActType::Draw(DrawParts::All);
                    }
                },
                CmdType::MouseScrollUp => {
                    if sidebar.cont.as_base().offset.y > 0 {
                        sidebar.cont.as_mut_base().offset.y -= 1;

                        sidebar.calc_scrlbar_v();
                        return ActType::Draw(DrawParts::SideBar);
                    }
                }
                CmdType::MouseScrollDown => {
                    if sidebar.cont.as_base().offset.y + 1 + sidebar.cont.get_cont_view().height <= sidebar.cont.get_cont_vec_len() {
                        sidebar.cont.as_mut_base().offset.y += 1;

                        sidebar.calc_scrlbar_v();
                        return ActType::Draw(DrawParts::SideBar);
                    }
                }

                // split line
                CmdType::MouseDownLeft(_, x) if *x == sidebar.view().x + sidebar.view().width => {
                    State::get().sidebar.resize = SideBarResizeState::Start;
                }
                // mouse
                CmdType::MouseDownLeft(y, x) if sidebar.view().is_y_range(*y) && sidebar.view().x + sidebar.view().width - Cfg::get().general.sidebar.scrollbar.vertical.width - 1 <= *x => return sidebar.ctrl_scrl_v(cmd_type),
                CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseUpLeft(_, _) if sidebar.scrl_v.is_enable => return sidebar.ctrl_scrl_v(cmd_type),
                _ => {
                    if let Ok(tree_file_view) = sidebar.cont.downcast_mut::<TreeFileView>() {
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

        let sidebar_state = State::get().sidebar;

        Log::debug("keys", &keys);

        Log::debug("sidebar_state", &sidebar_state);

        if sidebar_state.is_show {
            let sidebar = SideBar::get();

            return match &keys {
                Keys::MouseDownLeft(y, x) if sidebar.view().y as u16 <= *y && *y <= (sidebar.view().y + sidebar.view().height) as u16 && sidebar.view().x as u16 <= *x && *x <= (sidebar.view().x + sidebar.view().width) as u16 => true,
                Keys::MouseScrollUp | Keys::MouseScrollDown if place_org == Place::SideBar => true,
                Keys::MouseDragLeft(_, _) | Keys::MouseUpLeft(_, _) if sidebar.scrl_v.is_enable => true,
                _ if sidebar_state.resize != SideBarResizeState::None => true,
                _ => false,
            };
        }
        return false;
    }

    pub fn calc_scrlbar_v(&mut self) {
        let offset = self.cont.as_base().offset;
        let height = self.cont.get_cont_view().height;
        let cont_vec_len = self.cont.get_cont_vec_len();
        self.scrl_v.calc_scrlbar_v(&CmdType::Null, offset, height, cont_vec_len, true);
    }
}
