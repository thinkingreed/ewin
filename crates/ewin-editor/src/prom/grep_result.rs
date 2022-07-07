use crate::{
    ewin_com::{global::*, model::*, util::*},
    model::*,
};
use ewin_cfg::{log::*, model::default::*};
use std::path::PathBuf;

impl Editor {
    pub fn set_grep_result(&mut self, row_str: String) {
        Log::debug_key("set_grep_result");

        self.set_rnw();
        self.win_mgr.curt().cur = Cur { y: self.buf.len_rows() - 1, x: 0, disp_x: 0 };

        self.scroll();

        // For files without read permission,
        // only log output is performed and screen display is not performed.
        let vec: Vec<&str> = row_str.splitn(3, ':').collect();

        if vec.len() > 2 {
            let ignore_prefix_str = format!("{}:{}:", vec[0], vec[1]);

            let regex = Cfg::get().general.editor.search.regex;
            let row = self.buf.len_rows() - 2;

            let (start_idx, end_idx, ignore_prefix_len) = match regex {
                true => (self.buf.row_to_byte(row), self.buf.len_bytes(), ignore_prefix_str.len()),
                false => (self.buf.row_to_char(row), self.buf.len_chars(), ignore_prefix_str.chars().count()),
            };

            let mut search_vec: Vec<SearchRange> = self.get_search_ranges(&self.search.str, start_idx, end_idx, ignore_prefix_len);
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
                        if is_include_path(&*CURT_DIR, &self.search.dir) {
                            let path = PathBuf::from(&*CURT_DIR).join(&vec[0]);
                            filenm = path.to_string_lossy().to_string().replace(&self.search.dir, "");
                        } else {
                            filenm = vec[0].to_string();
                        }
                        Log::debug("setting_filenm", &filenm);
                    }
                    GrepResult::new(filenm, row_num)
                }
                // Does not occur
                Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), 0),
            };
            self.grep_result_vec.push(grep_result);
        }
    }
}
