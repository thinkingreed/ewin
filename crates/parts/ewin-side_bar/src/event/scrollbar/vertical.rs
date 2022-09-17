use crate::sidebar::*;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*};
use ewin_key::key::cmd::*;
use std::cmp::min;

impl SideBar {
    pub fn ctrl_scrl_v(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("SideBar.ctrl_scrl_v");
        Log::debug("cmd_type", &cmd_type);

        match cmd_type {
            CmdType::MouseDownLeft(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftDown(y, _) => {
                Log::debug("self.scrl_v.view.y 111", &self.scrl_v.view.y);
                Log::debug("self.cont.as_base().view", &self.cont.as_base().view);
                self.scrl_v.ctrl_scrollbar_v(*y, cmd_type, self.cont.as_base().view.x + self.cont.as_base().view.width - Cfg::get().general.sidebar.scrollbar.vertical.width, self.cont.get_cont_view().y, self.cont.get_cont_view().height);

                Log::debug("self.scrl_v.view.y 222", &self.scrl_v.view.y);

                self.cont.as_mut_base().offset.y = min(self.scrl_v.view.y * self.scrl_v.move_len, self.cont.get_cont_vec_len() - self.cont.get_cont_view().height);
                return ActType::Draw(DrawParts::SideBar);
            }
            CmdType::MouseUpLeft(_, _) => self.scrl_v.is_enable = false,
            _ => return ActType::None,
        };

        /*
        // scrlbar_h
        let height = Cfg::get().system.scrollbar.horizontal.height;
        match self.cmd.cmd_type {
            CmdType::MouseDownLeft(_, x) if self.win_mgr.curt().scrl_h.view.y <= y && y < self.win_mgr.curt().scrl_h.view.y + height => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            CmdType::MouseDragLeftDown(_, x) | CmdType::MouseDragLeftUp(_, x) | CmdType::MouseDragLeftLeft(_, x) | CmdType::MouseDragLeftRight(_, x) if self.win_mgr.curt().scrl_h.is_enable => {
                self.set_scrlbar_h_posi(x);
                return;
            }
            _ => self.win_mgr.curt().scrl_h.is_enable = false,
        };
        */

        return ActType::Draw(DrawParts::SideBar);
    }
}
