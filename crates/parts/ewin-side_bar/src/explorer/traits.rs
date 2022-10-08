use std::cmp::min;

use crate::{
    explorer::{explorer::Explorer, file::ExplorerFile},
    sidebar::*,
    traits::traits::*,
};
use crossterm::cursor::MoveTo;
use ewin_activity_bar::activitybar::*;
use ewin_cfg::{colors::*, log::*, model::general::default::*};
use ewin_const::models::{draw::DrawParts, event::ActType, view::*};
use ewin_key::key::cmd::CmdType;
use ewin_utils::str_edit::*;
use ewin_view::view::*;

use super::quick_access::ExplorerQuickAccess;

impl SideBarContTrait for Explorer {
    fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("TreeFileView.draw");

        str_vec.push(Colors::get_sidebar_bg_header());

        Log::debug("self.base.view", &self.base.view);

        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, self.base.view.y as u16), get_space(self.base.view.width)));
        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, (self.base.view.y + 1) as u16), get_space(self.base.view.width)));
        str_vec.push(Colors::get_sidebar_fg_bg());
        str_vec.push(MoveTo(self.base.view.x as u16, (self.tree_view.y) as u16).to_string());

        self.quick_access.draw(str_vec);

        let tgt_vec = if self.base.offset.y + self.tree_view.height > self.vec_show.len() { self.vec_show[self.base.offset.y..].to_vec() } else { self.vec_show[self.base.offset.y..self.base.offset.y + self.tree_view.height].to_vec() };
        Log::debug("show tgt_vec", &tgt_vec);

        for (idx, tree_file) in (self.tree_view.y..self.tree_view.y_height()).zip(tgt_vec.iter()) {
            str_vec.push(format!("{}", MoveTo(self.tree_view.x as u16, idx as u16)));

            str_vec.push(if tree_file.is_tgt_file { Colors::get_sidebar_bg_open_file() } else { Colors::get_sidebar_bg() });
            let dispnm = adjust_str_len(&tree_file.get_path(true).chars().collect::<Vec<char>>().get(self.base.offset.x..).unwrap_or(&[]).iter().collect::<String>(), self.tree_view.width, false, true);

            str_vec.push(dispnm);
        }

        // sidebar split line
        str_vec.push(Colors::get_statusbar_bg());
        for y in self.base.view.y..self.base.view.y + self.base.view.height {
            str_vec.push(format!("{}{}", MoveTo((self.base.view.x_width()) as u16, y as u16), " "));
        }
    }

    fn get_cont_vec(&mut self) -> &mut Vec<ExplorerFile> {
        &mut self.vec_show
    }

    fn get_cont_vec_len(&self) -> usize {
        self.vec_show.len()
    }
    fn get_mut_cont_view(&mut self) -> &mut View {
        &mut self.tree_view
    }
    fn get_ref_cont_view(&self) -> &View {
        &self.tree_view
    }

    fn as_base(&self) -> &SideBarContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut SideBarContBase {
        &mut self.base
    }

    fn set_size(&mut self) {
        Log::debug_key("TreeFileView.set_size");
        Log::debug("self.base.view", &self.base.view);

        Log::debug("CfgEdit::get().general.sidebar.width", &CfgEdit::get().general.sidebar.width);

        self.quick_access.set_size(&self.base.view);

        let mut tree_view = self.base.view.clone();
        tree_view.height = self.base.view.height - Explorer::HEADER_HEIGHT;

        let cfg_edit = CfgEdit::get();

        self.base.view.width = cfg_edit.general.sidebar.width;
        tree_view.width = cfg_edit.general.sidebar.width - cfg_edit.general.sidebar.explorer.quick_access.width;
        tree_view.y += Explorer::HEADER_HEIGHT;
        tree_view.x = ActivityBar::get().get_width() + cfg_edit.general.sidebar.explorer.quick_access.width + ExplorerQuickAccess::SPLIT_LINE_WIDTH;
        self.tree_view = tree_view;
    }

    fn ctrl_scrl_v(&mut self, cmd_type: &CmdType) -> ActType {
        // return SideBar::com_ctrl_scrl_v(cmd_type, &mut self.base.scrl_v, &mut self.base.offset, &mut self.tree_view, self.vec_show.len());

        match cmd_type {
            CmdType::MouseDownLeft(y, _) | CmdType::MouseDragLeftUp(y, _) | CmdType::MouseDragLeftDown(y, _) => {
                let view_y_org = self.base.scrl_v.view.y;
                self.base.scrl_v.ctrl_scrollbar_v(cmd_type, self.tree_view.y, self.tree_view.height);

                if view_y_org != self.base.scrl_v.view.y {
                    self.base.offset.y = min(self.base.scrl_v.view.y * self.base.scrl_v.move_len, self.vec_show.len() - self.tree_view.height);
                }
                return ActType::Draw(DrawParts::SideBar);
            }
            CmdType::MouseUpLeft(_, _) => self.base.scrl_v.is_enable = false,
            _ => return ActType::None,
        };
        return ActType::Draw(DrawParts::SideBar);
    }

    fn ctrl_scrl_h(&mut self, cmd_type: &CmdType) -> ActType {
        match cmd_type {
            CmdType::MouseDownLeft(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) => {
                let view_x = self.tree_view.x;
                let view_width = self.tree_view.width;
                self.base.scrl_h.ctrl_scrollbar_h(cmd_type, view_x, view_width);
                self.base.offset.x = self.base.scrl_h.view.x - self.tree_view.x;
                return ActType::Draw(DrawParts::SideBar);
            }
            CmdType::MouseUpLeft(_, _) => self.base.scrl_h.is_enable = false,
            _ => return ActType::None,
        };
        return ActType::Draw(DrawParts::SideBar);
    }

    fn calc_scrlbar_h(&mut self) {
        self.base.scrl_h.calc_scrlbar_h(self.tree_view.width, &self.base.scrl_h_info, self.base.offset.disp_x);
    }

    fn calc_scrlbar_v(&mut self) {
        self.base.scrl_v.calc_scrlbar_v(&CmdType::Null, self.base.offset, self.tree_view.height, self.vec_show.len());
    }

    fn draw_scrlbar(&self, str_vec: &mut Vec<String>) {
        self.base.scrl_v.draw(str_vec, &self.tree_view, Colors::get_statusbar_bg());

        self.base.scrl_h.draw(str_vec, &self.tree_view);
    }

    fn set_size_scrlbar_v(&mut self) {
        if self.vec_show.len() > self.tree_view.height {
            self.base.scrl_v.is_show = true;
            self.base.scrl_v.view.width = Cfg::get().general.sidebar.scrollbar.vertical.width;
            self.tree_view.width -= self.base.scrl_v.view.width;
            self.base.scrl_v.view.x = ActivityBar::get().get_width() + self.quick_access.base.view.width + ExplorerQuickAccess::SPLIT_LINE_WIDTH + self.tree_view.width;
        }
    }

    fn is_scrl_v_enable(&self) -> bool {
        self.base.scrl_v.is_enable || self.quick_access.base.scrl_v.is_enable
    }

    fn disable_scrl_v(&mut self) {
        self.base.scrl_v.is_enable = false;
        self.quick_access.base.scrl_v.is_enable = false;
    }
    
}
