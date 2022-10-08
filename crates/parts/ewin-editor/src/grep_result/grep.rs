use crate::{ewin_key::model::*, model::*};
use ewin_cfg::{log::*, model::general::default::*};

use ewin_job::job::*;
use ewin_key::{cur::*, key::cmd::*};
use ewin_state::term::*;
use ewin_utils::global::*;
use std::path::PathBuf;

impl Editor {
    pub fn set_grep_result(&mut self, row_str: String) {
        Log::debug_key("set_grep_result");

        self.set_rnw();
        self.win_mgr.curt_mut().cur = Cur { y: self.buf.len_rows() - 1, x: 0, disp_x: 0 };

        if self.grep_result_vec.is_empty() {
            let grep_info = State::get().curt_mut_state().grep.clone();
            self.search = State::get().curt_mut_state().grep.search.clone();
            // for editor scroll
            self.cmd = Cmd::to_cmd(CmdType::GrepingProm(grep_info));
        }
        self.scroll();
        let vec: Vec<&str> = row_str.splitn(3, ':').collect();

        if vec.len() > 1 {
            let result: Result<usize, _> = vec[1].parse();
            let grep_result = match result {
                // text.txt:100:string
                Ok(row_num) => {
                    // For Windows
                    // If the grep search folder contains the current folder,
                    // the relative path is returned in the grep result, otherwise the absolute path is returned.
                    // if is_include_path(&*CURT_DIR, &self.search.dir) {
                    let filenm = if self.search.dir.contains(&*CURT_DIR) { PathBuf::from(&vec[0]).file_name().unwrap().to_string_lossy().to_string() } else { vec[0].to_string() };

                    GrepResult::new(filenm, row_num)
                }
                // Does not occur
                Err(_) => GrepResult::new(vec[1].to_string().as_str().trim().to_string(), 0),
            };

            Log::debug("grep_result", &grep_result);

            // New line code is fixed to LF because it is a non-editable file
            self.buf.insert_end(&format!("{}:{}:{}", grep_result.filenm, grep_result.row_num, vec[2]));

            self.grep_result_vec.push(grep_result.clone());
            // For files without read permission,
            // only log output is performed and screen display is not performed.
            if vec.len() > 2 {
                let regex = Cfg::get().general.editor.search.regex;
                let row = self.buf.len_rows() - 2;

                let ignore_prefix_str = format!("{}:{}:", grep_result.filenm, grep_result.row_num);

                let (start_idx, end_idx, ignore_prefix_len) = match regex {
                    true => (self.buf.row_to_byte(row), self.buf.len_bytes(), ignore_prefix_str.len()),
                    false => (self.buf.row_to_char(row), self.buf.len_chars(), ignore_prefix_str.chars().count()),
                };

                let mut search_vec: Vec<SearchRange> = self.get_search_ranges(&self.search.str, start_idx, end_idx, ignore_prefix_len);
                self.search.ranges.append(&mut search_vec);
            }
        }
    }

    pub fn exit_grep_result(&mut self, is_cancel: bool) {
        Log::debug_key("exit_grep_result");

        let is_empty = self.grep_result_vec.is_empty();
        State::get().curt_mut_state().grep.is_cancel = is_cancel;
        State::get().curt_mut_state().grep.is_empty = is_empty;
        self.set_cur_default();
        self.scroll();
        State::get().curt_mut_state().editor.is_read_only = true;
        self.set_init_scrlbar();

        Job::send_cmd(CmdType::GrepResultProm);
    }
}
