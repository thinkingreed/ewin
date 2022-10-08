use std::cmp::min;

use crate::{explorer::file::*, sidebar::*};
use downcast::{downcast, Any};
use dyn_clone::DynClone;
use ewin_activity_bar::activitybar::*;
use ewin_cfg::{colors::Colors, model::general::default::*};
use ewin_const::models::{draw::DrawParts, event::*};
use ewin_key::key::cmd::*;
use ewin_view::view::*;
pub trait SideBarContTrait: Any + Send + DynClone + 'static + std::fmt::Debug {
    fn as_base(&self) -> &SideBarContBase;
    fn as_mut_base(&mut self) -> &mut SideBarContBase;
    fn draw(&self, str_vec: &mut Vec<String>);
    fn set_size(&mut self);
    fn get_cont_vec_len(&self) -> usize;
    fn get_mut_cont_view(&mut self) -> &mut View;
    fn get_ref_cont_view(&self) -> &View;
    fn get_cont_vec(&mut self) -> &mut Vec<ExplorerFile>;

    fn ctrl_scrl_v(&mut self, cmd_type: &CmdType) -> ActType {
        match cmd_type {
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftDown(_, _) => {
                let scrl_v_view_y_org = self.as_base().scrl_v.view.y;
                let view_y = self.get_ref_cont_view().y;
                let view_height = self.get_ref_cont_view().height;
                self.as_mut_base().scrl_v.ctrl_scrollbar_v(cmd_type, view_y, view_height);

                if scrl_v_view_y_org != self.as_base().scrl_v.view.y {
                    self.as_mut_base().offset.y = min(self.as_base().scrl_v.view.y * self.as_base().scrl_v.move_len, self.get_cont_vec_len() - self.get_mut_cont_view().height);
                }
                return ActType::Draw(DrawParts::SideBar);
            }
            CmdType::MouseUpLeft(_, _) => self.as_mut_base().scrl_v.is_enable = false,
            _ => return ActType::None,
        };
        return ActType::None;
    }

    fn ctrl_scrl_h(&mut self, cmd_type: &CmdType) -> ActType {
        match cmd_type {
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) => {
                let view_x = self.as_base().view.x;
                let view_width = self.as_base().view.width;
                self.as_mut_base().scrl_h.ctrl_scrollbar_h(cmd_type, view_x, view_width);

                self.as_mut_base().offset.x = self.as_base().scrl_h.view.x - self.as_base().view.x;
                return ActType::Draw(DrawParts::SideBar);
            }
            CmdType::MouseUpLeft(_, _) => self.as_mut_base().scrl_h.is_enable = false,
            _ => return ActType::None,
        }
        return ActType::None;
    }
    fn calc_scrlbar_v(&mut self) {
        let offset = self.as_base().offset;
        let height = self.get_mut_cont_view().height;
        let cont_vec_len = self.get_cont_vec_len();
        self.as_mut_base().scrl_v.calc_scrlbar_v(&CmdType::Null, offset, height, cont_vec_len);
    }
    fn calc_scrlbar_h(&mut self) {
        let view_width = self.get_ref_cont_view().width;
        let scrl_h_info = &self.as_base().scrl_h_info.clone();
        let disp_x = self.as_base().offset.disp_x;
        self.as_mut_base().scrl_h.calc_scrlbar_h(view_width, scrl_h_info, disp_x);
    }

    fn draw_scrlbar(&self, str_vec: &mut Vec<String>) {
        self.as_base().scrl_v.draw(str_vec, self.get_ref_cont_view(), Colors::get_statusbar_bg());
        self.as_base().scrl_h.draw(str_vec, self.get_ref_cont_view());
    }

    fn set_size_scrlbar_v(&mut self) {
        self.as_mut_base().scrl_v.is_show = true;
        self.as_mut_base().scrl_v.view.width = Cfg::get().general.sidebar.scrollbar.vertical.width;
        self.get_mut_cont_view().width -= self.as_base().scrl_v.view.width;
        self.as_mut_base().scrl_v.view.x = ActivityBar::get().get_width() + self.as_base().view.width;
    }

    fn is_scrl_v_enable(&self) -> bool {
        self.as_base().scrl_v.is_enable
    }
    fn disable_scrl_v(&mut self) {
        self.as_mut_base().scrl_v.is_enable = false;
    }
}

downcast!(dyn SideBarContTrait);
dyn_clone::clone_trait_object!(SideBarContTrait);
