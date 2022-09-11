use ewin_utils::files::dir::Dir;
use std::{
    fs,
    path::{PathBuf, MAIN_SEPARATOR},
};

impl TreeFile {
    pub fn new(root: &str, fullpath_buf: PathBuf) -> Self {
        let dispnm = if root == fullpath_buf.to_string_lossy() { root.to_string() } else { fullpath_buf.file_name().unwrap().to_str().unwrap().to_string() };

        let mut _self = Self { fullpath: fullpath_buf.to_string_lossy().to_string(), dispnm, level: fullpath_buf.to_string_lossy().to_string().replace(&root, "").split(MAIN_SEPARATOR).count() - 1, ..TreeFile::default() };
        let meta_opt = if let Ok(meta) = fs::metadata(&fullpath_buf) { Some(meta) } else { None };
        if let Some(meta) = meta_opt {
            _self.is_dir = meta.is_dir();
        }
        // _self.is_show = _self.level == 0;
        _self.is_show = true;
        return _self;
    }
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct TreeFile {
    pub fullpath: String,
    pub dispnm: String,
    pub is_show: bool,
    pub is_tgt_file: bool,
    pub is_dir: bool,
    pub dir: Dir,
    pub level: usize,
}
