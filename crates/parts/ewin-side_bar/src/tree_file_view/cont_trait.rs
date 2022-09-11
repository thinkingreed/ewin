use crate::{side_bar_trait::side_bar_trait::*, sidebar::*};

use super::tree::*;
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_const::models::view::*;
use ewin_utils::str_edit::*;

impl SideBarContTrait for TreeFileView {
    fn draw(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("TreeFileView.draw");

        str_vec.push(Colors::get_sidebar_bg_header());

        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, self.base.view.y as u16), get_space(self.base.view.width)));
        str_vec.push(format!("{}{}", MoveTo(self.base.view.x as u16, (self.base.view.y + 1) as u16), get_space(self.base.view.width)));
        str_vec.push(Colors::get_sidebar_fg_bg());
        str_vec.push(format!("{}", MoveTo(self.base.view.x as u16, (self.view_tree.y) as u16)));

        Log::debug(" self.view_tree", &self.view_tree);

        for (idx, tree_idx) in (self.view_tree.y..self.view_tree.y + self.view_tree.height).zip(self.base.offset.y..self.base.offset.y + self.view_tree.height) {
            Log::debug(" idx", &idx);
            Log::debug(" tree_idx", &idx);

            if tree_idx > self.vec.len() - 1 {
                break;
            }

            let tree_file = &self.vec[tree_idx];

            if tree_file.is_show {
                Log::debug_d("tree_file", &tree_file);
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
        }

        // sidebar split line
        str_vec.push(Colors::get_statusbar_bg());
        for y in self.base.view.y..self.base.view.y + self.base.view.height {
            str_vec.push(format!("{}{}", MoveTo((self.base.view.x + self.base.view.width) as u16, y as u16), " "));
        }
    }

    fn draw_scrlbar_v(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("TreeFileView.draw_scrlbar_v");
        if self.scrl_v.is_show {
            for i in self.view_tree.y..self.view_tree.y + self.view_tree.height {
                Log::debug_key("111111111111111111111111111111111111");
                str_vec.push(MoveTo(self.scrl_v.view.x as u16, i as u16).to_string());
                Log::debug_key("222222222222222222222222222222222222");
                str_vec.push(if self.view_tree.y + self.scrl_v.view.y <= i && i < self.view_tree.y + self.scrl_v.view.y + self.scrl_v.bar_len { Colors::get_scrollbar_v_bg() } else { Colors::get_default_bg() });
                Log::debug_key("3333333333333333333333333333333333333");
                str_vec.push(" ".to_string().repeat(self.scrl_v.bar_width));
            }
        }
        str_vec.push(Colors::get_default_bg());
    }

    fn as_base(&self) -> &SideBarContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut SideBarContBase {
        &mut self.base
    }
}
