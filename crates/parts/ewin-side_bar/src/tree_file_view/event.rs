use super::{tree::*, tree_file::*};
use ewin_cfg::log::*;
use ewin_const::models::{draw::*, event::*};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_utils::files::dir::*;
use std::path::PathBuf;

impl TreeFileView {
    pub fn ctrl_evt(&mut self, cmd_type: &CmdType) -> ActType {
        Log::debug_key("TreeFileView.ctrl_evt");
        Log::debug("cmd_type", &cmd_type);
        match cmd_type {
            CmdType::MouseDownLeft(y, x) => {
                Log::debug("yyy", &y);
                Log::debug("xxx", &x);

                // header
                if self.base.view.y <= *y && *y < self.base.view.y + TreeFileView::HEADER_HEIGHT {
                    return ActType::None;
                } else {
                    let y = y - self.view_tree.y + self.base.offset.y;
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

            CmdType::MouseMove(y, x) => {
                return ActType::Cancel;
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft => {
                return ActType::Cancel;
            }
            CmdType::Confirm => return ActType::Cancel,
            _ => return ActType::None,
        }
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
        self.vec_show = self.vec_all.iter().filter(|node| node.is_show).cloned().collect::<Vec<TreeFile>>();
    }
}
