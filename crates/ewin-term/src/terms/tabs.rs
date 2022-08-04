use crate::{ewin_editor::model::*, model::*, tab::*};
use chrono::{DateTime, Local};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{def::*, model::*};
use ewin_key::{files::file::*, global::*, key::cmd::*};
use ewin_state::{header_file::*, tabs::*};
use std::usize;

use super::term::*;

impl Term {
    pub fn new_tab(&mut self) {
        // Disable the event in case of the next display
        self.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));
        let mut new_tab = Tab::new();

        let dt: DateTime<Local> = Local::now();
        Log::debug("dt", &dt);
        self.add_tab(&mut new_tab, HeaderFile::new(&String::new()), FileOpenType::Nomal);
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

        Tabs::get().idx = self.tab_idx;
        Tabs::get().h_file_vec.insert(self.tab_idx, h_file.clone());
        Tabs::get().state_vec.insert(self.tab_idx, TabsState::default());

        self.fbar.disp_base_idx = USIZE_UNDEFINED;
        self.init_tab(&h_file, file_open_type);
    }

    pub fn change_tab(&mut self, idx: usize) {
        Log::debug_key("change_tab");

        self.tab_idx = idx;

        Log::debug("Tabs::get() 111", &Tabs::get());

        Tabs::get().idx = self.tab_idx;
        //  Tabs::set_idx( self.tab_idx);

        Log::debug("Tabs::get() 222", &Tabs::get());

        let h_file = Tabs::get().curt_h_file().clone();
        self.init_tab(&h_file, FileOpenType::Nomal);
    }

    pub fn swap_tab(&mut self, idx_org: usize, idx_dst: usize) {
        Log::debug_key("swap_tab");

        Tabs::get().h_file_vec.swap(idx_org, idx_dst);
        self.tabs.swap(idx_org, idx_dst);
        self.change_tab(idx_dst);
    }

    pub fn reopen_tab(&mut self, tab: Tab, h_file: HeaderFile, file_open_type: FileOpenType) {
        Log::debug_key("reopen_tab");
        self.tabs[self.tab_idx] = tab;

        for vec in self.curt().editor_draw_vec.iter_mut() {
            for editor_draw in vec {
                editor_draw.clear();
            }
        }
        Tabs::get().h_file_vec[self.tab_idx] = h_file.clone();
        self.init_tab(&h_file, file_open_type);
    }

    pub fn init_tab(&mut self, h_file: &HeaderFile, file_open_type: FileOpenType) {
        Log::debug_key("init_tab");
        self.curt().editor.init_editor_scrlbar_h();
        self.set_size();
        self.curt().editor.calc_editor_scrlbar_v();
        self.curt().editor.calc_editor_scrlbar_h();

        self.curt().editor.h_file = h_file.clone();
        if file_open_type != FileOpenType::Reopen && File::is_exist_file(&h_file.file.fullpath) {
            if let Some(Ok(mut watch_info)) = WATCH_INFO.get().map(|watch_info| watch_info.try_lock()) {
                watch_info.fullpath = h_file.file.fullpath.clone();
                watch_info.mode = h_file.watch_mode;
            }
        }
        Term::set_title(&h_file.file.name);
    }

    pub fn del_tab(&mut self, tab_idx: usize) {
        Log::debug_key("del_tab");
        Log::debug("tab_idx", &tab_idx);
        self.tab_idx = if tab_idx == Tabs::get().h_file_vec.len() - 1 && tab_idx != 0 { tab_idx - 1 } else { self.tab_idx };
        self.tabs.remove(tab_idx);

        Tabs::get().del_file(tab_idx, self.tab_idx);

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

        return if self.tabs.is_empty() {
            ActType::Exit
        } else {
            Tabs::get().idx = self.tab_idx;
            ActType::Draw(DParts::All)
        };
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
        self.curt().editor.draw_range = E_DrawRange::All;
        self.tab_idx += 1;
    }
}
