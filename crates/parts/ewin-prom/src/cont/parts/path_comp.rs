use ewin_cfg::log::*;
use ewin_const::def::*;
use ewin_utils::{files::file::File, path::*};
use std::{
    fmt,
    path::{self, Path},
};

impl PathComp {
    pub fn get_path_candidate(&mut self, is_asc: bool, target_path: String, is_dir_only: bool) -> Vec<char> {
        Log::debug_key("get_path_candidate");
        if self.files.is_empty() {
            self.files = get_path_comp_files(target_path.clone(), is_dir_only, true);
        }
        Log::debug("self.files", &self.files);

        let mut rtn_string = target_path;

        for file in &self.files {
            // One candidate

            match self.files.len() {
                0 => {}
                1 => {
                    if !is_dir_only {
                        let path = Path::new(&file.name);
                        //  let path = Path::new(&os_str);
                        rtn_string = if path.metadata().unwrap().is_file() { file.name.to_string() } else { format!("{}{}", file.name, path::MAIN_SEPARATOR) };
                    } else {
                        rtn_string = format!("{}{}", file.name, path::MAIN_SEPARATOR);
                    }
                    self.clear_path_comp();
                    break;
                }
                _ => {
                    Log::debug_s("Multi candidates");
                    Log::debug("self.tab_comp.index", &self.index);
                    if is_asc && self.index >= self.files.len() - 1 || self.index == USIZE_UNDEFINED {
                        self.index = 0;
                    } else if !is_asc && self.index == 0 {
                        self.index = self.files.len() - 1;
                    } else {
                        self.index = if is_asc { self.index + 1 } else { self.index - 1 };
                    }
                    rtn_string = self.files[self.index].name.clone();
                    break;
                }
            }
        }

        return rtn_string.chars().collect();
    }
    pub fn clear_path_comp(&mut self) {
        Log::debug_key("clear_tab_comp ");
        self.index = USIZE_UNDEFINED;
        self.files.clear();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathComp {
    // List of complementary candidates
    pub files: Vec<File>,
    // List of complementary candidates index
    pub index: usize,
}
impl Default for PathComp {
    fn default() -> Self {
        PathComp { index: USIZE_UNDEFINED, files: vec![] }
    }
}
impl fmt::Display for PathComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabComp index:{}, files:{:?},", self.index, self.files,)
    }
}
