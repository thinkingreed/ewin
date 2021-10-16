use crate::{
    ewin_com::{def::*, global::*, log::*, model::*, util::*},
    model::*,
};
use std::path::PathBuf;

impl Editor {
    pub fn set_grep_result(&mut self, line_str: String) {
        self.rnw = if self.state.mouse_mode == MouseMode::Normal { self.buf.len_lines().to_string().len() } else { 0 };
        self.cur = Cur { y: self.buf.len_lines() - 1, x: 0, disp_x: 0 };

        self.scroll();

        // Pattern
        //   text.txt:100:string
        //   grep:text.txt:No permission
        let vec: Vec<&str> = line_str.splitn(3, ":").collect();

        if vec.len() > 2 && vec[0] != "grep" {
            let ignore_prefix_str = format!("{}:{}:", vec[0], vec[1]);

            let regex = CFG.get().unwrap().try_lock().unwrap().general.editor.search.regex;
            let row = self.buf.len_lines() - 2;

            let (start_idx, end_idx, ignore_prefix_len) = match regex {
                true => (self.buf.line_to_byte(row), self.buf.len_bytes(), ignore_prefix_str.len()),
                false => (self.buf.line_to_char(row), self.buf.len_chars(), ignore_prefix_str.chars().count()),
            };

            let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
            let mut search_vec: Vec<SearchRange> = self.get_search_ranges(&self.search.str, start_idx, end_idx, ignore_prefix_len, cfg_search);
            self.search.ranges.append(&mut search_vec);
        }

        if vec.len() > 1 {
            let result: Result<usize, _> = vec[1].parse();

            let grep_result = match result {
                // text.txt:100:string
                Ok(row_num) => {
                    let filenm;
                    if cfg!(target_os = "linux") {
                        filenm = vec[0].to_string();
                    } else {
                        // For Windows
                        // If the grep search folder contains the current folder,
                        // the relative path is returned in the grep result, otherwise the absolute path is returned.
                        if is_include_path(&*CURT_DIR, &self.search.folder) {
                            let path = PathBuf::from(&*CURT_DIR).join(&vec[0].to_string());
                            filenm = path.to_string_lossy().to_string().replace(&self.search.folder, "");
                        } else {
                            filenm = vec[0].to_string();
                        }
                        Log::debug("setting_filenm", &filenm);
                    }
                    GrepResult::new(filenm, row_num)
                }
                // grep:text.txt:No permission
                Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), USIZE_UNDEFINED),
            };
            self.grep_result_vec.push(grep_result);
        }
    }
}
