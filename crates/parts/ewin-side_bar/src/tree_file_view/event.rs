use crate::sidebar::SideBar;

use super::tree::*;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_utils::files::dir::*;
use std::path::PathBuf;

impl TreeFileView {
    pub fn ctrl_evt(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("TreeFileView.ctrl_evt");
        match cmd_type {
            CmdType::MouseDownLeft(y, _) => {
                // header
                if self.base.view.y <= *y && *y < self.base.view.y + TreeFileView::HEADER_HEIGHT {
                    return ActType::None;
                    // tree
                } else {
                    let y = y - self.view_tree.y + self.base.offset.y;
                    if self.vec[y].is_dir {
                        if !self.vec[y].dir.is_chaild_search {
                            let mut dirs: Vec<PathBuf> = vec![];
                            Dir::visit_dir(&self.vec[y].fullpath, dirs.as_mut(), &VisitDirType::CurrentDirOnlyScan);
                            Log::debug("dirsdirsdirsdirsdirsdirsdirs", &dirs);
                            self.vec[y].dir.is_chaild_search = true;
                            self.add_tree_vec(dirs);
                        }
                        self.open_dir(y);
                    } else {
                        let fullpath = self.vec[y].fullpath.clone();
                        self.open_file(&fullpath);
                        Job::send_cmd(CmdType::OpenTgtFile(fullpath));
                    }
                    return ActType::Draw(DrawParts::SideBar);
                }
            }
            CmdType::MouseScrollUp => {
                if self.base.offset.y > 0 {
                    self.base.offset.y -= 1;
                    self.calc_scrlbar_v();
                    return ActType::Draw(DrawParts::SideBar);
                }
            }
            CmdType::MouseScrollDown => {
                if self.base.offset.y + 1 + self.view_tree.height <= self.vec.len() {
                    self.base.offset.y += 1;
                    self.calc_scrlbar_v();
                    return ActType::Draw(DrawParts::SideBar);
                }
            }
            CmdType::ChangeFileSideBar(fullpath) => {
                self.open_file(fullpath);
                return ActType::Draw(DrawParts::SideBar);
            }

            // mouse
            CmdType::MouseDownLeft(_, _) | CmdType::MouseUpLeft(_, _) | CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseDownLeftBox(_, _) | CmdType::MouseDragLeftBox(_, _) => self.ctrl_mouse(),

            CmdType::MouseMove(y, x) => {
                return ActType::Cancel;
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                return ActType::Cancel;
            }
            CmdType::Confirm => return ActType::Cancel,
            _ => return ActType::None,
        }
        return ActType::None;
    }

    pub fn open_dir(&mut self, y: usize) {
        let is_open = self.vec[y].dir.is_open;
        self.vec[y].dir.is_open = !is_open;

        let tgt_node_fullpath = self.vec[y].fullpath.clone();

        for node in self.vec.iter_mut() {
            if node.fullpath != tgt_node_fullpath && node.fullpath.contains(&tgt_node_fullpath) {
                node.is_show = !is_open;
            }
        }
    }
}
