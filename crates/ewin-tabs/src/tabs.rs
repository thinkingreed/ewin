use crate::tab::tab::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*, file::*, model::*},
};
use ewin_editor::{editor_gr::*, model::*};
use ewin_file_bar::filebar::*;
use ewin_job::job::*;
use ewin_key::{
    global::*,
    key::{cmd::*, keys::*},
    model::*,
};
use ewin_state::term::*;
use ewin_utils::files::file::*;
use ewin_view::view::*;
use std::usize;

impl Tabs {
    pub fn new_grep_tgt_tab(&mut self, search: &Search) -> ActType {
        let act_type = self.new_tab(&search.fullpath);
        if let ActType::Draw(DrawParts::MsgBar(_)) = act_type {
            return act_type;
        };
        EditorGr::get().curt_mut().search = search.clone();
        EditorGr::get().curt_mut().set_cmd_keys(Cmd::to_cmd(CmdType::FindNext), Keys::Null);
        EditorGr::get().curt_mut().search_str(true, false);
        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn new_tab(&mut self, fullpath: &str) -> ActType {
        Log::debug_key("Tabs.new_tab");
        if fullpath.is_empty() {
            self.add_tab(Editor::default(), File::new(fullpath), FileOpenType::Nomal);
        } else {
            let idx_opt = State::get().is_opened_file_idx(fullpath);
            // If there is already an open file
            if let Some(idx) = idx_opt {
                self.change_file(idx);
            } else {
                let act_type = self.open_file(fullpath, FileOpenType::Nomal, None);
                if let ActType::Draw(DrawParts::MsgBar(_)) = act_type {
                    return act_type;
                };
            }
        }
        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn init_tab(&mut self, file: &File, file_open_type: FileOpenType) {
        Log::debug_key("init_tab");
        self.set_size();

        EditorGr::get().curt_mut().set_init_scrlbar();

        if file_open_type != FileOpenType::Reopen && File::is_exist_file(&file.fullpath) {
            if let Some(Some(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                watch_info.fullpath = file.fullpath.clone();
                watch_info.mode = file.watch_mode;
            }
        }
        View::set_title(&file.name);
        Job::send_cmd(CmdType::ChangeFileSideBar(file.fullpath.clone()));
    }

    pub fn reopen_tab(&mut self, editor: Editor, file: File, file_open_type: FileOpenType) {
        Log::debug_key("reopen_tab");

        for cache_vec in self.curt().draw_cache_vecs.iter_mut() {
            for cache in cache_vec.iter_mut() {
                cache.clear();
            }
        }

        // self.curt().draw_cache_vecs.clear();

        State::get().tabs.vec[self.idx].file = file;
        EditorGr::get().vec[self.idx] = editor;

        let file = State::get().tabs.vec[self.idx].file.clone();
        self.init_tab(&file, file_open_type);
    }

    pub fn add_tab(&mut self, editor: Editor, file: File, file_open_type: FileOpenType) {
        Log::debug_key("add_tab");
        self.idx = self.vec.len();
        self.vec.push(Tab::new());

        self.curt().draw_cache_vecs.push(vec![EditorDrawCache::default()]);

        State::get().add_tab(file.clone());

        FileBar::get().add_tab();
        EditorGr::get().add_tab(editor);

        self.init_tab(&file, file_open_type);
    }

    pub fn change_file(&mut self, idx: usize) -> ActType {
        Log::debug_key("change_tab");

        Log::debug("idx", &idx);

        self.idx = idx;

        Log::debug("State::get().tabs 111", &State::get().tabs);

        State::get().tabs.idx = self.idx;

        Log::debug("State::get().tabs 222", &State::get().tabs);

        let file = &State::get().curt_ref_state().file.clone();

        Log::debug("file", &file);

        self.init_tab(file, FileOpenType::Nomal);

        return ActType::Draw(DrawParts::TabsAll);
    }

    pub fn switch_file(&mut self, direction: Direction) -> ActType {
        if self.vec.len() > 1 {
            let idx = if direction == Direction::Right {
                if self.vec.len() - 1 == self.idx {
                    0
                } else {
                    self.idx + 1
                }
            } else if self.idx == 0 {
                self.vec.len() - 1
            } else {
                self.idx - 1
            };
            self.change_file(idx);

            return ActType::Draw(DrawParts::TabsAll);
        } else {
            return ActType::Draw(DrawParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
        }
    }

    pub fn swap_file(&mut self, idx_org: usize, idx_dst: usize) -> ActType {
        Log::debug_key("swap_tab");
        Log::debug("idx_org", &idx_org);
        Log::debug("idx_dst", &idx_dst);

        FileBar::get().file_vec.swap(idx_org, idx_dst);
        State::get().tabs.vec.swap(idx_org, idx_dst);

        self.vec.swap(idx_org, idx_dst);
        self.change_file(idx_dst);

        ActType::Draw(DrawParts::FileBar)
    }

    pub fn del_tab(&mut self, del_idx: usize) {
        Log::debug_key("del_tab");
        Log::debug("del_idx", &del_idx);
        Log::debug("self.idx 111", &self.idx);

        self.idx = if del_idx == FileBar::get().file_vec.len() - 1 && del_idx != 0 {
            del_idx - 1
        } else if self.idx > del_idx {
            self.idx - 1
        } else {
            self.idx
        };

        Log::debug("self.idx 222", &self.idx);

        self.vec.remove(del_idx);

        State::get().del_file(del_idx, self.idx);
        FileBar::get().del_file(del_idx);

        self.change_file(self.idx);

        if let Some(Some(mut grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
            if grep_cancel_vec.len() > del_idx {
                grep_cancel_vec.remove(del_idx);
            }
        }
    }

    pub fn close_other_than_this_tab(&mut self, leave_tab_idx: usize) -> ActType {
        Log::debug_key("close_other_than_this_tab");
        Log::debug("leave_tab_idx", &leave_tab_idx);
        State::get().tabs.all.close_other_than_this_tab_idx = leave_tab_idx;
        Log::debug("State::get().tabs.all.close_other_than_this_tab_idx", &State::get().tabs.all.close_other_than_this_tab_idx);

        let mut idx = self.vec.len();

        for _ in 0..self.vec.len() {
            idx -= 1;
            Log::debug("idx", &idx);

            if idx == leave_tab_idx {
                continue;
            }
            self.idx = idx;
            if State::get().tabs.vec[idx].editor.is_changed {
                return Job::send_cmd(CmdType::CloseFileTgt(idx));
            } else {
                Log::debug("self.del_tab", &idx);
                self.del_tab(idx);
            }
            if self.vec.is_empty() {
                break;
            }
        }

        if !self.vec.is_empty() && leave_tab_idx != USIZE_UNDEFINED {
            self.idx = self.vec.len() - 1;
            if self.vec.len() != 1 && self.idx == leave_tab_idx {
                self.idx -= 1;
            }
            if self.vec.len() == 1 {
                State::get().tabs.all.close_other_than_this_tab_idx = USIZE_UNDEFINED;
            }
        }

        return if self.vec.is_empty() {
            ActType::Exit
        } else {
            State::get().tabs.idx = self.idx;
            ActType::Draw(DrawParts::TabsAll)
        };
    }

    pub fn save_all_tab(&mut self) -> ActType {
        Log::debug_key("save_all_tab");
        State::get().tabs_mut_all().is_all_save = true;
        let len = self.vec.len() - 1;
        for idx in (0..=len).rev() {
            self.idx = idx;
            if State::get().tabs.vec[idx].editor.is_changed {
                let act_type = EditorGr::get().curt_mut().save(&SaveFileType::Normal);
                if let ActType::Draw(_) = act_type {
                    return act_type;
                }
            }
        }
        return ActType::Next;
    }

    pub fn clear_pre_tab_status(&mut self) {
        self.idx -= 1;

        self.curt().clear_curt_tab(true);

        /*
        self.curt().prom.clear();
        State::get().curt_mut_state().clear();
        self.curt().msgbar.clear();
        self.set_size();
        self.curt().editor.draw_range = E_DrawRange::All;
         */
        self.idx += 1;
    }

    /*

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;

        for tab in self.vec.iter_mut() {
            if State::get().curt_state().editor.is_changed {
                tab.editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                State::get().curt_mut_state().clear();
            }
        }
    }
     */

    #[track_caller]
    pub fn curt(&mut self) -> &mut Tab {
        return self.vec.get_mut(self.idx).unwrap();
    }
}

#[derive(Debug, Default, Clone)]
// Terminal
pub struct Tabs {
    pub vec: Vec<Tab>,
    // tab index
    pub idx: usize,
}
