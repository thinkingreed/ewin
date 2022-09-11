use std::{
    fs,
    path::{Path, PathBuf},
};

impl Dir {
    pub fn visit_dir<P: AsRef<Path>>(path: &P, paths: &mut Vec<PathBuf>, visit_dir_type: &VisitDirType) {
        Dir::visit_dir_exec(path, paths, visit_dir_type);
        paths.sort();
    }

    fn visit_dir_exec<P: AsRef<Path>>(path: P, paths: &mut Vec<PathBuf>, visit_dir_type: &VisitDirType) {
        if let Ok(mut read_dir) = fs::read_dir(&path) {
            while let Some(Ok(entry)) = read_dir.next() {
                if let Ok(metadata) = entry.metadata() {
                    match visit_dir_type {
                        VisitDirType::FullScan | VisitDirType::CurrentDirOnlyScan => {
                            if !paths.contains(&entry.path()) {
                                paths.push(entry.path());
                            }
                            if metadata.is_dir() && visit_dir_type == &VisitDirType::FullScan {
                                Dir::visit_dir(&entry.path(), paths, visit_dir_type)
                            }
                        }
                        VisitDirType::TargetDirRouteOnlyScan(path) => {
                            let path_str = entry.path().to_string_lossy().to_string();
                            // target dir before and after target dir
                            if path.contains(&path_str) || path_str.contains(path) {
                                if !paths.contains(&entry.path()) {
                                    paths.push(entry.path());
                                }
                                if metadata.is_dir() {
                                    // Do not search deeper than the target dir
                                    if path.len() < path_str.len() {
                                        continue;
                                    }
                                    Dir::visit_dir(&entry.path(), paths, visit_dir_type);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum VisitDirType {
    FullScan,
    CurrentDirOnlyScan,
    TargetDirRouteOnlyScan(String),
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Dir {
    pub is_open: bool,
    pub is_chaild_search: bool,
}
