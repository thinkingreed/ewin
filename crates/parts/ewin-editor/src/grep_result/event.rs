use crate::{ewin_key::model::*, model::*};
use ewin_cfg::log::*;
use ewin_const::{def::*, models::event::*};
use ewin_job::job::*;
use ewin_key::key::cmd::*;
use ewin_state::term::*;
use std::path::*;

impl Editor {
    pub fn grep_result(&mut self) -> ActType {
        Log::debug_key("Editor.grep_result");

        match self.cmd.cmd_type {
            CmdType::Confirm => {
                let y = self.win_mgr.curt_mut().cur.y;
                let grep_result = self.grep_result_vec[y].clone();

                if grep_result.row_num != USIZE_UNDEFINED {
                    let grep = State::get().curt_ref_state().grep.clone();
                    let mut search = Search { str: grep.search.str, filenm: grep_result.filenm, dir: grep.search.dir, row_num: grep_result.row_num - 1, ..Search::default() };
                    search.fullpath = format!("{}{}{}", &search.dir, MAIN_SEPARATOR, &search.filenm);

                    Log::debug("search", &search);
                    return Job::send_cmd(CmdType::OpenGrepTgtFile(search));
                }
                return ActType::Cancel;
            }
            _ => return ActType::Cancel,
        }
    }
}
