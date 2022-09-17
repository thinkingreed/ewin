use crate::{side_bar_trait::side_bar_trait::*, sidebar::*};

use super::tree::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::models::view::*;
use ewin_utils::str_edit::*;
use ewin_view::view::*;

impl SideBarContTrait for TreeFileView {
    fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("TreeFileView.draw");

        str_vec.push(Colors::get_sidebar_bg_header());

        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, self.base.view.y as u16), get_space(self.base.view.width)));
        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, (self.base.view.y + 1) as u16), get_space(self.base.view.width)));
        str_vec.push(Colors::get_sidebar_fg_bg());
        str_vec.push(format!("{}", MoveTo(self.base.view.x as u16, (self.view_tree.y) as u16)));

        let tgt_vec = if self.base.offset.y + self.view_tree.height > self.vec_show.len() { self.vec_show[self.base.offset.y..].to_vec() } else { self.vec_show[self.base.offset.y..self.base.offset.y + self.view_tree.height].to_vec() };

        Log::debug("show tgt_vec", &tgt_vec);

        for (idx, tree_file) in (self.view_tree.y..self.view_tree.y + self.view_tree.height).zip(tgt_vec.iter()) {
            str_vec.push(format!("{}", MoveTo(self.base.view.x as u16, idx as u16)));

            let icon = if tree_file.is_dir {
                if tree_file.dir.is_open {
                    "-📂"
                } else {
                    "+📁"
                }
            } else {
                " 📄"
            };
            str_vec.push(if tree_file.is_tgt_file { Colors::get_sidebar_bg_open_file() } else { Colors::get_sidebar_bg() });
            let filenm = format!("{}{}{}", get_space(tree_file.level), icon, tree_file.dispnm);
            let dispnm = adjust_str_len(&filenm, self.base.view.width, false, true);

            str_vec.push(dispnm);
        }

        // sidebar split line
        str_vec.push(Colors::get_statusbar_bg());
        for y in self.base.view.y..self.base.view.y + self.base.view.height {
            str_vec.push(format!("{}{}", MoveTo((self.base.view.x + self.base.view.width) as u16, y as u16), " "));
        }
    }

    fn resize(&mut self) {
        self.resize();
    }

    fn get_cont_vec_len(&self) -> usize {
        self.vec_show.len()
    }
    fn get_cont_view(&self) -> View {
        self.view_tree
    }

    fn as_base(&self) -> &SideBarContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut SideBarContBase {
        &mut self.base
    }
}
