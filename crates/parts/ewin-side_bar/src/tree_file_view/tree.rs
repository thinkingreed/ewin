use crate::{side_bar_trait::side_bar_trait::*, sidebar::*};

use super::tree_file::*;
use directories::BaseDirs;
use ewin_cfg::{log::*, model::general::default::*};
use ewin_const::{def::*, term::*};
use ewin_key::key::cmd::*;
use ewin_utils::files::{dir::*, file::*};
use ewin_view::{scrollbar_v::ScrollbarV, view::*};
use std::path::{Path, PathBuf};

impl TreeFileView {
    pub const HEADER_HEIGHT: usize = 2;

    pub fn create_cont(tgt_file: &str) -> Box<dyn SideBarContTrait> {
        let rows = get_term_size().1;

        let mut all_view = View { x: 0, width: CfgEdit::get().general.sidebar.width, ..View::default() };
        all_view.height = rows - MENUBAR_HEIGHT - MSGBAR_ROW_NUM - STATUSBAR_ROW_NUM;
        all_view.y = MENUBAR_HEIGHT;

        let mut tree_file = TreeFileView::new(tgt_file);
        tree_file.base.view = all_view;
        tree_file.indent = Cfg::get().general.sidebar.treefile.indent;

        let mut tree_view = all_view;
        tree_view.height = all_view.height - TreeFileView::HEADER_HEIGHT;
        tree_view.y += TreeFileView::HEADER_HEIGHT;
        tree_file.view_tree = tree_view;

        tree_file.open_file(tgt_file);

        tree_file.init_scrlbar_v();
        tree_file.calc_scrlbar_v();

        return Box::new(tree_file);
    }

    pub fn init_scrlbar_v(&mut self) {
        let show_vec = self.vec.iter().filter(|node| node.is_show).collect::<Vec<_>>();
        if self.view_tree.height < show_vec.len() {
            self.scrl_v.bar_width = Cfg::get().general.sidebar.scrollbar.vertical.width;
            self.view_tree.width -= self.scrl_v.bar_width;
            self.scrl_v.view.x = self.view_tree.width;
        };
    }

    pub fn calc_scrlbar_v(&mut self) {
        let show_vec = self.vec.iter().filter(|node| node.is_show).collect::<Vec<_>>();
        if self.view_tree.height < show_vec.len() {
            self.scrl_v.is_show = true;
        };
        self.scrl_v.calc_scrlbar_v(&CmdType::Null, self.base.offset, self.view_tree.height, show_vec.len(), true)
    }

    pub fn open_file(&mut self, fullpath: &str) {
        if let Some(node) = self.vec.iter_mut().find(|node| node.is_tgt_file) {
            node.is_tgt_file = false;
        }
        if let Some(node) = self.vec.iter_mut().find(|node| node.fullpath == fullpath) {
            node.is_tgt_file = true;
        }
        self.adjust_offset(fullpath);
    }

    // Adjust the offset so that the target file is displayed in the center
    pub fn adjust_offset(&mut self, tgt_file: &str) {
        Log::debug_key("TreeFileView.adjust_offset");

        let show_vec = self.vec.iter().filter(|node| node.is_show).collect::<Vec<_>>();

        Log::debug_d("show_vec", &show_vec);

        if let Some(open_file_idx) = self.vec.iter().position(|node| node.is_show && node.is_tgt_file) {
            Log::debug("open_node idx", &open_file_idx);
            let open_file_offset_idx = open_file_idx as isize - self.base.offset.y as isize;
            Log::debug("open_file_offset_idx", &open_file_offset_idx);

            // offset setting for files not displayed in tree
            if !(0 <= open_file_offset_idx && open_file_offset_idx < self.view_tree.height as isize) {
                if let Some(idx) = self.vec.iter().position(|node| node.fullpath == tgt_file) {
                    if self.view_tree.height < idx {
                        // +1 is an adjustment to show up in the case of a fractions
                        let diff = idx + 1 - self.view_tree.height;
                        self.base.offset.y = (self.view_tree.height as f64 / 2_f64).ceil() as usize + diff;
                    }
                }
            }
        }
    }

    pub fn new(tgt_file: &str) -> Self {
        Log::debug("TreeFileView::new()", &tgt_file);
        let root_dir = if let Some(base_dirs) = BaseDirs::new() { base_dirs.home_dir().to_string_lossy().to_string() } else { "".to_string() };

        let mut tree_file_view = TreeFileView::default();
        tree_file_view.root_dir = root_dir;

        tree_file_view.vec = tree_file_view.create_tree_vec(tgt_file);
        // root is open by default
        tree_file_view.vec[0].dir.is_open = true;
        return tree_file_view;
    }

    pub fn create_tree_vec(&mut self, tgt_file: &str) -> Vec<TreeFile> {
        Log::debug("TreeFileView.create_tree_vec()", &tgt_file);

        let mut tree_node_vec = vec![];

        let mut dirs = vec![PathBuf::from(self.root_dir.clone())];
        Dir::visit_dir(&self.root_dir, dirs.as_mut(), &VisitDirType::CurrentDirOnlyScan);

        Log::debug_d("dirsdirsdirsdirsdirsdirsdirsdirs 111111111111", &dirs);

        let parent = Path::new(&File::get_absolute_path(tgt_file)).parent().unwrap().to_str().unwrap().to_string();

        Log::debug("self.root_dir", &self.root_dir);
        Log::debug("parent", &parent);

        Dir::visit_dir(&self.root_dir, dirs.as_mut(), &VisitDirType::TargetDirRouteOnlyScan(parent));

        Log::debug_d("dirsdirsdirsdirsdirsdirsdirsdirs 222222222222", &dirs);

        for dir in dirs {
            tree_node_vec.push(TreeFile::new(&self.root_dir, dir));
        }

        for node in tree_node_vec.iter_mut() {
            if tgt_file.contains(&node.fullpath) && node.is_dir {
                node.dir.is_open = true;
            }
        }

        tree_node_vec[0].dir.is_chaild_search = true;

        Log::debug("tree_node_vec", &tree_node_vec);

        tree_node_vec
    }

    pub fn add_tree_vec(&mut self, vec: Vec<PathBuf>) {
        for dir in vec {
            self.vec.push(TreeFile::new(&self.root_dir, dir));
        }
        self.vec.sort();
    }
}

#[derive(Debug, Default, Clone)]
pub struct TreeFileView {
    pub base: SideBarContBase,
    pub view_tree: View,
    pub indent: usize,
    pub root_dir: String,
    pub vec: Vec<TreeFile>,
    pub scrl_v: ScrollbarV,
}
