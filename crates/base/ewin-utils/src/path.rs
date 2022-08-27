use std::{fs, path::MAIN_SEPARATOR};

use ewin_cfg::log::*;

use crate::{char_edit::*, files::file::*};

pub fn get_path_comp_files(target_path: String, is_dir_only: bool, is_full_path_filenm: bool) -> Vec<File> {
    Log::debug_key("get_tab_comp_files");

    // Search target dir
    let mut base_dir = ".".to_string();
    let vec: Vec<(usize, &str)> = target_path.match_indices(MAIN_SEPARATOR).collect();
    // "/" exist
    if !vec.is_empty() {
        let (base, _) = target_path.split_at(vec[vec.len() - 1].0 + 1);
        base_dir = base.to_string();
    }

    let mut rtn_vec: Vec<File> = vec![];

    if let Ok(mut read_dir) = fs::read_dir(&base_dir) {
        while let Some(Ok(path)) = read_dir.next() {
            if !is_dir_only || path.path().is_dir() {
                let mut filenm = path.path().display().to_string();

                if filenm.match_indices(target_path.as_str()).next().is_some() {
                    // Replace "./" for display
                    if &base_dir == "." {
                        filenm = filenm.replace("./", "");
                    }
                    if !is_full_path_filenm {
                        filenm = path.path().file_name().unwrap().to_string_lossy().to_string();
                    }
                    // let is_dir = if path.metadata().is_ok() { path.metadata().unwrap().is_dir() } else { true };
                    rtn_vec.push(File::new(&filenm.to_string()));
                }
            }
        }
    }
    rtn_vec.sort_by_key(|file| file.name.clone());
    rtn_vec
}

pub fn get_dir_path(path_str: &str) -> String {
    let mut vec = split_chars(path_str, true, true, &[MAIN_SEPARATOR]);
    // Deleted when characters were entered

    if !vec.is_empty() && vec.last().unwrap() != &MAIN_SEPARATOR.to_string() {
        vec.pop();
    }
    vec.join("")
}

pub fn is_include_path(src: &str, dst: &str) -> bool {
    let src_vec: Vec<&str> = src.split(MAIN_SEPARATOR).collect();
    let dst_vec: Vec<&str> = dst.split(MAIN_SEPARATOR).collect();

    let mut is_include = false;
    for (i, src) in src_vec.iter().enumerate() {
        if let Some(dst) = dst_vec.get(i) {
            is_include = src == dst;
        } else {
            is_include = false;
        }
    }
    is_include
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::path::get_dir_path;

    #[test]
    fn test_get_dir_path() {
        assert_eq!(get_dir_path("/home/"), "/home/".to_string());
        assert_eq!(get_dir_path("/home/ewin"), "/home/".to_string());
        assert_eq!(get_dir_path(""), "".to_string());
    }
    #[test]
    fn test_is_include_path() {
        assert!(is_include_path("/home", "/home/ewin"));
        assert!(!is_include_path("/hoge", "/home/ewin"));
    }
}
