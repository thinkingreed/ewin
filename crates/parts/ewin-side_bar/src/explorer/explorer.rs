use super::{file::*, quick_access::*};
use crate::{sidebar::*, traits::traits::*};
use ewin_cfg::{log::*, model::general::default::*};
use ewin_utils::files::{dir::*, file::*};
use ewin_view::{
    scrollbar::{
        horizontal::{ScrlHInfo, ScrollbarH},
        vertical::ScrollbarV,
    },
    view::*,
};
use std::path::{Path, PathBuf};

impl Explorer {
    pub const HEADER_HEIGHT: usize = 2;

    pub fn create_cont(tgt_file: &str) -> Box<dyn SideBarContTrait> {
        let mut tree_file = Explorer::new(tgt_file);
        tree_file.indent = Cfg::get().general.sidebar.explorer.tree.indent;

        return Box::new(tree_file);
    }

    pub fn open_file(&mut self, fullpath: &str) {
        if let Some(node) = self.vec_all.iter_mut().find(|node| node.is_tgt_file) {
            node.is_tgt_file = false;
        }
        if let Some(node) = self.vec_all.iter_mut().find(|node| node.fullpath == fullpath) {
            node.is_tgt_file = true;
        }
        self.adjust_offset(fullpath);
    }

    // Adjust the offset so that the target file is displayed in the center
    pub fn adjust_offset(&mut self, tgt_file: &str) {
        Log::debug_key("TreeFileView.adjust_offset");

        self.vec_show = self.vec_all.iter().filter(|node| node.is_show).cloned().collect::<Vec<ExplorerFile>>();

        // Log::debug_d("self.vec_show", &self.vec_show);

        if let Some(open_file_idx) = self.vec_show.iter().position(|node| node.is_show && node.is_tgt_file) {
            Log::debug("open_node idx", &open_file_idx);
            let open_file_offset_idx = open_file_idx as isize - self.base.offset.y as isize;
            Log::debug("open_file_offset_idx", &open_file_offset_idx);

            // offset setting for files not displayed in tree
            if !(0 <= open_file_offset_idx && open_file_offset_idx < self.tree_view.height as isize) {
                if let Some(idx) = self.vec_show.iter().position(|node| node.fullpath == tgt_file) {
                    if self.tree_view.height < idx {
                        self.base.offset.y = idx + 1 - (self.tree_view.height as f64 / 2_f64).ceil() as usize;
                    }
                }
            }
        }
    }

    pub fn new(tgt_file: &str) -> Self {
        Log::debug("TreeFileView::new()", &tgt_file);

        let root_dir = Dir::get_home_dir();
        let mut explorer = Explorer::default();
        explorer.quick_access = explorer.quick_access.get_quick_access();
        explorer.root_dir = root_dir;

        explorer.vec_all = explorer.create_tree_vec(tgt_file);
        // root is open by default
        explorer.vec_all[0].dir.is_open = true;
        return explorer;
    }

    pub fn create_tree_vec(&mut self, tgt_file: &str) -> Vec<ExplorerFile> {
        Log::debug("TreeFileView.create_tree_vec()", &tgt_file);

        let mut explorer_file_vec = vec![];

        let mut dirs = vec![PathBuf::from(self.root_dir.clone())];
        Dir::visit_dir(&self.root_dir, dirs.as_mut(), &VisitDirType::CurrentDirOnlyScan);

        let parent = Path::new(&File::get_absolute_path(tgt_file)).parent().unwrap().to_str().unwrap().to_string();

        Log::debug("self.root_dir", &self.root_dir);
        Log::debug("parent", &parent);

        Dir::visit_dir(&self.root_dir, dirs.as_mut(), &VisitDirType::TargetDirRouteOnlyScan(parent));

        for dir in dirs {
            explorer_file_vec.push(ExplorerFile::new(&self.root_dir, dir));
        }

        for node in explorer_file_vec.iter_mut() {
            if tgt_file.contains(&node.fullpath) && node.is_dir {
                node.dir.is_open = true;
            }
        }
        explorer_file_vec[0].dir.is_chaild_search = true;
        Log::debug("tree_node_vec", &explorer_file_vec);

        explorer_file_vec
    }

    pub fn add_tree_vec(&mut self, vec: Vec<PathBuf>) {
        for dir in vec {
            self.vec_all.push(ExplorerFile::new(&self.root_dir, dir));
        }
        self.vec_all.sort();
    }
}

#[derive(Debug, Default, Clone)]
pub struct Explorer {
    pub base: SideBarContBase,
    pub tree_view: View,
    pub indent: usize,
    pub root_dir: String,
    pub vec_all: Vec<ExplorerFile>,
    pub vec_show: Vec<ExplorerFile>,
    pub quick_access: ExplorerQuickAccess,
}
