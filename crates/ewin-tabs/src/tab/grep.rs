use crate::ewin_key::model::*;
use ewin_cfg::log::*;

use ewin_const::{def::*, models::draw::*};
use ewin_editor::{editor_gr::*, model::*};
use ewin_job::job::*;
use ewin_key::{global::*, grep_cancel::*};
use ewin_utils::files::nl::*;
use ewin_view::traits::view::*;
use std::{ffi::OsStr, io::Write, path::PathBuf};

use super::tab::*;

impl Tab {
    pub fn draw_grep_result<T: Write>(&mut self, out: &mut T, job_grep: JobGrep) {
        Log::debug_key("draw_grep_result");

        if GrepCancel::is_canceling() {
            Log::debug_s("EvtAct::is_grep_Canceling()");
            GREP_CANCEL_VEC.get().unwrap().try_lock().map(|mut vec| *vec.last_mut().unwrap() = GrepCancelType::Canceled).unwrap();
            EditorGr::get().curt_mut().exit_grep_result(true);
        } else if GrepCancel::is_canceled() {
            Log::debug_s("EvtAct::is_grep_canceled()");
            return;
        } else if job_grep.is_end {
            Log::debug_s("grep is end");
            EditorGr::get().curt_mut().exit_grep_result(false);
        } else {
            if job_grep.grep_str.trim().is_empty() {
                return;
            }

            let path = PathBuf::from(&EditorGr::get().curt_ref().search.fullpath);
            let filenm = path.file_name().unwrap_or_else(|| OsStr::new("")).to_string_lossy().to_string();
            let replace_folder = EditorGr::get().curt_ref().search.fullpath.replace(&filenm, "");
            let mut row_str = job_grep.grep_str.replace(&replace_folder, "");
            Log::debug("line_str", &row_str);

            NL::del_nl(&mut row_str);
            row_str.push(NEW_LINE_LF);

            EditorGr::get().curt_mut().set_grep_result(row_str);
            EditorGr::get().curt_mut().set_size();

            let len_rows = EditorGr::get().curt_ref().buf.len_rows();
            let row_len = EditorGr::get().curt_ref().get_curt_row_len();
            let rnw = EditorGr::get().curt_ref().rnw;
            let rnw_org = EditorGr::get().curt_ref().rnw_org;

            if len_rows > row_len && rnw_org == rnw {
                let y = EditorGr::get().curt_ref().win_mgr.curt_ref().offset.y + EditorGr::get().curt_ref().get_curt_row_len() - 2;
                //  self.editor.draw(out, if cfg!(target_os = "windows") { &DParts::All } else { &DParts::ScrollUpDown(ScrollUpDownType::Grep) });

                let draw_range = if cfg!(target_os = "windows") { E_DrawRange::All } else { E_DrawRange::ScrollDown(y - 2, y) };
                Editor::draw_only(out, &mut self.draw_cache_vecs, &DrawParts::Editor(draw_range));
            } else {
                Editor::draw_only(out, &mut self.draw_cache_vecs, &DrawParts::Editor(E_DrawRange::All));
            }
        }
    }
}
