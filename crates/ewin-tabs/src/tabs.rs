use crate::tab::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*, model::*},
};
use ewin_editor::model::*;
use ewin_file_bar::filebar::*;
use ewin_job::job::*;
use ewin_key::{global::*, key::cmd::*};
use ewin_state::term::*;
use ewin_utils::files::file::*;
use ewin_view::view::*;
use std::usize;

impl Tabs {
    pub fn new_tab(&mut self) -> ActType {
        // Disable the event in case of the next display
        let mut new_tab = Tab::new();

        self.add_tab(&mut new_tab, File::new(""), FileOpenType::Nomal);
        return ActType::Draw(DParts::All);
    }

    pub fn init_tab(&mut self, file: &File, file_open_type: FileOpenType) {
        Log::debug_key("init_tab");
        self.curt().editor.init_editor_scrlbar_h();
        // self.set_size();

        self.set_size();
        self.curt().editor.calc_editor_scrlbar_v();
        self.curt().editor.calc_editor_scrlbar_h();

        if file_open_type != FileOpenType::Reopen && File::is_exist_file(&file.fullpath) {
            if let Some(Ok(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                watch_info.fullpath = file.fullpath.clone();
                watch_info.mode = file.watch_mode;
            }
        }
        View::set_title(&file.name);
    }

    pub fn reopen_tab(&mut self, tab: Tab, file: File, file_open_type: FileOpenType) {
        Log::debug_key("reopen_tab");
        self.vec[self.idx] = tab;

        for vec in self.curt().editor.draw_cache.iter_mut() {
            for editor_draw in vec {
                editor_draw.clear();
            }
        }
        // FileBar::get().file_vec[self.idx] = h_file.clone();

        State::get().tabs.vec[self.idx].file = file;

        self.init_tab(&State::get().tabs.vec[self.idx].file, file_open_type);
    }
    pub fn add_tab(&mut self, tab: &mut Tab, file: File, file_open_type: FileOpenType) {
        Log::debug_key("add_tab");
        self.idx = self.vec.len();
        tab.idx = self.vec.len();
        self.vec.push(tab.clone());

        self.curt().editor.draw_cache.push(vec![EditorDraw::default()]);

        State::get().add_tab(file.clone());

        FileBar::get().add_tab();

        self.init_tab(&file, file_open_type);
    }

    pub fn change_file(&mut self, idx: usize) -> ActType {
        Log::debug_key("change_tab");

        Log::debug("idx", &idx);

        self.idx = idx;

        Log::debug("State::get().tabs 111", &State::get().tabs);

        State::get().tabs.idx = self.idx;

        Log::debug("State::get().tabs 222", &State::get().tabs);

        let file = &State::get().curt_state().file.clone();

        Log::debug("file", &file);

        self.init_tab(file, FileOpenType::Nomal);

        return ActType::Draw(DParts::All);
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

            return ActType::Draw(DParts::All);
        } else {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
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

        ActType::Draw(DParts::FileBar)
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

        if let Some(Ok(mut grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
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
                Job::send_cmd(CmdType::CloseFileTgt(idx));
                return ActType::None;
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
            ActType::Draw(DParts::All)
        };
    }

    pub fn save_all_tab(&mut self) -> ActType {
        Log::debug_key("save_all_tab");
        self.state.is_all_save = true;
        let len = self.vec.len() - 1;
        for idx in (0..=len).rev() {
            self.idx = idx;
            if State::get().tabs.vec[idx].editor.is_changed {
                let act_type = self.curt().save(&SaveFileType::Normal);
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

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;
        for tab in self.vec.iter_mut() {
            if State::get().curt_state().editor.is_changed {
                tab.editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                State::get().curt_mut_state().clear();
            }
        }
    }
    pub fn cancel_save_all_tab(&mut self) {
        self.state.is_all_save = false;
    }

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
    pub state: TabsState,
}

#[derive(Debug, Default, Clone)]
pub struct TabsState {
    pub is_all_close_confirm: bool,
    pub is_all_save: bool,
    // pub close_other_than_this_tab_idx: usize,
}
