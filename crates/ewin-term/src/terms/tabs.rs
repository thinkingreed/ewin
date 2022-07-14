use crate::{
    ewin_com::{_cfg::key::cmd::*, files::file::*, global::*, model::*},
    ewin_editor::model::*,
    global_term::*,
    model::*,
    tab::Tab,
};
use chrono::{DateTime, Local};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::def::*;
use std::usize;

use super::term::*;

impl Terminal {
    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));

        let mut new_tab = Tab::new();

        let dt: DateTime<Local> = Local::now();
        Log::debug("dt", &dt);
        self.add_tab(&mut new_tab, HeaderFile::new(&Lang::get().new_file), FileOpenType::Nomal);
    }

    pub fn switch_tab(&mut self, direction: Direction) -> ActType {
        if self.tabs.len() > 1 {
            let idx = if direction == Direction::Right {
                if self.tabs.len() - 1 == self.tab_idx {
                    0
                } else {
                    self.tab_idx + 1
                }
            } else if self.tab_idx == 0 {
                self.tabs.len() - 1
            } else {
                self.tab_idx - 1
            };
            self.change_tab(idx);
            return ActType::Draw(DParts::All);
        } else {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_tab_can_be_switched.to_string()));
        }
    }

    pub fn add_tab(&mut self, tab: &mut Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        Log::debug_key("add_tab");
        self.tab_idx = self.tabs.len();
        tab.idx = self.tabs.len();
        self.tabs.push(tab.clone());
        self.curt().editor_draw_vec.push(vec![EditorDraw::default()]);

        // FileBar::push_h_file_vec(h_file.clone());
        H_FILE_VEC.get().unwrap().try_lock().map(|mut vec| vec.push(h_file.clone())).unwrap();
        self.fbar.disp_base_idx = USIZE_UNDEFINED;

        self.init_tab(&h_file, file_open_type);
    }
    pub fn change_tab(&mut self, idx: usize) {
        self.tab_idx = idx;
        let h_file = H_FILE_VEC.get().unwrap().try_lock().unwrap()[self.tab_idx].clone();
        self.init_tab(&h_file, FileOpenType::Nomal);
    }

    pub fn swap_tab(&mut self, idx_org: usize, idx_dst: usize) {
        Log::debug_key("swap_tab");

        H_FILE_VEC.get().unwrap().try_lock().unwrap().swap(idx_org, idx_dst);
        self.tabs.swap(idx_org, idx_dst);
        self.change_tab(idx_dst);
    }

    pub fn reopen_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        Log::debug_key("reopen_tab");
        // tab.idx = self.tab_idx;
        let tab_idx = self.tab_idx;
        self.tabs[self.tab_idx] = tab;
        self.curt().editor_draw_vec[tab_idx].clear();
        H_FILE_VEC.get().unwrap().try_lock().unwrap()[self.tab_idx] = h_file.clone();
        self.init_tab(&h_file, file_open_type);
    }

    pub fn init_tab(&mut self, h_file: &HeaderFile, file_open_type: FileOpenType) {
        self.curt().editor.init_editor_scrlbar_h();
        self.set_size();
        self.curt().editor.calc_editor_scrlbar_v();
        self.curt().editor.calc_editor_scrlbar_h();

        self.curt().editor.h_file = h_file.clone();
        if file_open_type != FileOpenType::Reopen && File::is_exist_file(&h_file.fullpath) {
            if let Some(Ok(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                watch_info.fullpath = h_file.fullpath.clone();
                watch_info.mode = h_file.watch_mode;
            }
        }
        Terminal::set_title(&h_file.filenm);
    }

    pub fn del_tab(&mut self, tab_idx: usize) {
        Log::debug_key("del_tab");
        Log::debug("tab_idx", &tab_idx);
        self.tab_idx = if tab_idx == H_FILE_VEC.get().unwrap().try_lock().unwrap().len() - 1 && tab_idx != 0 { tab_idx - 1 } else { self.tab_idx };
        self.tabs.remove(tab_idx);
        // self.curt().editor_draw_vec.remove(tab_idx);
        H_FILE_VEC.get().unwrap().try_lock().unwrap().remove(tab_idx);
        self.fbar.disp_base_idx = USIZE_UNDEFINED;
        self.change_tab(self.tab_idx);

        if let Some(Ok(mut grep_cancel_vec)) = GREP_CANCEL_VEC.get().map(|vec| vec.try_lock()) {
            if grep_cancel_vec.len() > tab_idx {
                grep_cancel_vec.remove(tab_idx);
            }
        }
    }
    pub fn close_tabs(&mut self, leave_tab_idx: usize) -> ActType {
        Log::debug_key("close_tabs");

        self.state.close_other_than_this_tab_idx = leave_tab_idx;
        if leave_tab_idx == USIZE_UNDEFINED {
            self.state.is_all_close_confirm = true;
        }
        let mut idx = self.tabs.len();

        for _ in 0..self.tabs.len() {
            idx -= 1;
            Log::debug("idx", &idx);
            if idx == self.state.close_other_than_this_tab_idx {
                continue;
            }
            self.tab_idx = idx;
            if self.tabs[idx].editor.state.is_changed {
                if self.curt().prom_show_com(&CmdType::CloseFile) == ActType::Exit {
                    return ActType::Exit;
                }
            } else {
                self.del_tab(idx);
                if self.state.close_other_than_this_tab_idx != 0 {
                    self.state.close_other_than_this_tab_idx -= 1;
                }
            }
            if self.tabs.is_empty() {
                break;
            }
        }

        if !self.tabs.is_empty() && self.state.close_other_than_this_tab_idx != USIZE_UNDEFINED {
            self.tab_idx = self.tabs.len() - 1;
            if self.tabs.len() != 1 && self.tab_idx == leave_tab_idx {
                self.tab_idx -= 1;
            }
            if self.tabs.len() == 1 {
                self.state.close_other_than_this_tab_idx = USIZE_UNDEFINED;
            }
        }

        return if self.tabs.is_empty() { ActType::Exit } else { ActType::Draw(DParts::All) };
    }

    pub fn save_all_tab(&mut self) -> ActType {
        Log::debug_key("save_all_tab");
        self.state.is_all_save = true;
        let len = self.tabs.len() - 1;
        for idx in (0..=len).rev() {
            self.tab_idx = idx;
            let act_type = self.curt().save(SaveType::Normal);
            if let ActType::Draw(_) = act_type {
                return act_type;
            } else {
                self.del_tab(idx);
            }
        }
        return ActType::Next;
    }

    pub fn cancel_close_all_tab(&mut self) {
        self.state.is_all_close_confirm = false;
        for tab in self.tabs.iter_mut() {
            if tab.editor.state.is_changed {
                tab.editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                tab.state.clear();
            }
        }
    }
    pub fn cancel_save_all_tab(&mut self) {
        self.state.is_all_save = false;
    }
    pub fn clear_pre_tab_status(&mut self) {
        self.tab_idx -= 1;

        self.curt().prom.clear();
        self.curt().state.clear();
        self.curt().msgbar.clear();
        self.set_size();
        self.curt().editor.win_mgr.curt().draw_range = E_DrawRange::All;
        self.tab_idx += 1;
    }
}
