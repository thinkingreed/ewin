use crate::traits::traits::*;

use super::{explorer::*, file::*};
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_utils::files::dir::*;
use std::path::PathBuf;

impl Explorer {
    pub fn ctrl_evt(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("TreeFileView.ctrl_evt");
        Log::debug("cmd_type", &cmd_type);
        match cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                Log::debug("yyy", &y);
                Log::debug("xxx", &x);

                // header
                if self.base.view.y <= *y && *y < self.base.view.y + Explorer::HEADER_HEIGHT {
                    return ActType::None;
                } else {
                    let y = y - self.tree_view.y + self.base.offset.y;
                    if self.vec_all[y].is_dir {
                        if !self.vec_all[y].dir.is_chaild_search {
                            let mut dirs: Vec<PathBuf> = vec![];
                            Dir::visit_dir(&self.vec_all[y].fullpath, dirs.as_mut(), &VisitDirType::CurrentDirOnlyScan);
                            self.vec_all[y].dir.is_chaild_search = true;
                            self.add_tree_vec(dirs);
                        }
                        self.open_close_dir(y);
                    } else {
                        let fullpath = self.vec_all[y].fullpath.clone();
                        self.open_file(&fullpath);
                        Job::send_cmd(CmdType::OpenTgtFile(fullpath));
                    }
                    return ActType::Draw(DrawParts::SideBar);
                }
            }

            CmdType::ChangeFileSideBar(fullpath) => {
                self.open_file(fullpath);
                return ActType::Draw(DrawParts::SideBar);
            }

            CmdType::MouseMove(_, _) => {
                return ActType::Cancel;
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                return ActType::Cancel;
            }
            CmdType::Confirm => return ActType::Cancel,
            CmdType::MouseScrollUp => {
                if self.base.scrl_v.is_show && self.base.offset.y > 0 {
                    self.base.offset.y -= 1;
                    self.calc_scrlbar_v();
                    return ActType::Draw(DrawParts::SideBar);
                }
            }
            CmdType::MouseScrollDown => {
                if self.base.scrl_v.is_show && self.base.offset.y + 1 + self.tree_view.height <= self.vec_show.len() {
                    self.base.offset.y += 1;

                    self.calc_scrlbar_v();
                    return ActType::Draw(DrawParts::SideBar);
                }
            }
            // mouse
            // scrl_v
            CmdType::MouseDownLeft(_, x) if self.base.scrl_v.view.is_x_range(*x) => return self.ctrl_scrl_v(cmd_type),
            CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseUpLeft(_, _) if self.base.scrl_v.is_enable => return self.ctrl_scrl_v(cmd_type),
            // scrl_h
            CmdType::MouseDownLeft(y, _) if self.base.scrl_h.view.is_y_range(*y) => return self.ctrl_scrl_h(cmd_type),
            CmdType::MouseDragLeftDown(_, _) | CmdType::MouseDragLeftUp(_, _) | CmdType::MouseDragLeftLeft(_, _) | CmdType::MouseDragLeftRight(_, _) | CmdType::MouseUpLeft(_, _) if self.base.scrl_h.is_enable => return self.ctrl_scrl_h(cmd_type),

            _ => return ActType::None,
        }
        return ActType::None;
    }

    pub fn open_close_dir(&mut self, y: usize) {
        Log::debug_key("TreeFileView.open_dir");

        let is_open = self.vec_all[y].dir.is_open;
        self.vec_all[y].dir.is_open = !is_open;

        let tgt_node_fullpath = self.vec_all[y].fullpath.clone();

        Log::debug("tgt_node_fullpath", &tgt_node_fullpath);

        for node in self.vec_all.iter_mut() {
            if node.fullpath != tgt_node_fullpath && node.fullpath.contains(&tgt_node_fullpath) {
                node.is_show = !is_open;
            }
        }
        self.vec_show = self.vec_all.iter().filter(|node| node.is_show).cloned().collect::<Vec<ExplorerFile>>();
    }
}
