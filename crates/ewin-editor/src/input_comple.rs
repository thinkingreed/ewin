use std::{cmp::min, collections::BTreeSet};

use crate::{
    ewin_com::{model::*, util::*},
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::_cfg::key::cmd::{Cmd, CmdType};
use ewin_widget::widget::input_comple::*;

impl Editor {
    pub fn init_input_comple(&mut self, is_first: bool) -> ActType {
        Log::debug_key("Editor.init_input_comple");

        let search_str = self.get_until_delim_str().0;
        if self.win_mgr.curt().cur.x != 0 && search_str.is_empty() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        }
        self.input_comple.search_set = self.input_comple.search(&search_str);
        if self.input_comple.search_set.is_empty() {
            return ActType::Draw(DParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        }

        self.input_comple.widget.clear();

        Log::debug("self.input_comple.search_set", &self.input_comple.search_set);
        let set = if is_first { self.input_comple.search_set.clone() } else { self.input_comple.search(&search_str) };

        if !is_first && search_str.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return ActType::Cancel;
        }
        if set.is_empty() {
            self.state.input_comple_mode = InputCompleMode::None;
            return ActType::Draw(DParts::MsgBar(Lang::get().no_input_comple_candidates.to_string()));
        } else if set.len() == 1 {
            for s in set {
                self.replace_input_comple(s);
                self.clear_input_comple();
            }
        } else {
            self.state.input_comple_mode = InputCompleMode::WordComple;

            self.input_comple.set_disp_name(&search_str);
            let height = min(self.input_comple.widget.cont.cont_vec.len(), InputComple::MAX_HEIGHT);

            self.input_comple.widget.init_menu(self.win_mgr.curt().cur.y + self.get_curt_row_posi() - self.win_mgr.curt().offset.y, self.win_mgr.curt().cur.disp_x + self.get_rnw_and_margin() - 1, height);
            // self.input_comple.widget.set_parent_disp_area(self.cur.y + self.row_posi - self.offset_y, self.cur.disp_x + self.get_rnw_and_margin() - 1, height);
            // self.input_comple.widget.set_init_menu();
        }

        return ActType::Draw(DParts::All);
    }
    pub fn ctrl_input_comple(&mut self) -> ActType {
        Log::debug_key("EvtAct::ctrl_input_comple_before");
        let cmd = self.cmd.clone();
        Log::debug("cmd", &cmd);

        match cmd.cmd_type {
            CmdType::InputComple => return self.init_input_comple(false),
            CmdType::MouseDownLeft(y, x) => {
                if self.input_comple.widget.is_mouse_within_area(y, x) {
                    Log::debug_s("is_mouse_within_range");
                    let evt_act = self.select_input_comple(); // EvtAct::select_input_comple(term);
                    self.clear_input_comple();
                    return evt_act;
                }
                let x = x - self.get_rnw_and_margin();
                let y = y - self.get_curt_row_posi() + self.win_mgr.curt().offset.y;
                self.set_cur_target_by_disp_x(y, x);

                self.clear_input_comple();
                return ActType::Draw(DParts::All);
            }
            CmdType::MouseMove(y, x) => {
                if self.input_comple.widget.is_mouse_within_area(y, x) {
                    self.input_comple.widget.set_offset_y(InputComple::MAX_HEIGHT);
                    self.input_comple.widget.ctrl_mouse_move(y, x);
                    if !self.input_comple.widget.is_menu_change() {
                        return ActType::Cancel;
                    }

                    return ActType::Draw(DParts::InputComple);
                } else if self.input_comple.widget.is_mouse_area_around(y, x) {
                    self.input_comple.widget.clear_select_menu();
                    return ActType::Draw(DParts::InputComple);
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::CursorDown | CmdType::CursorUp | CmdType::CursorRight | CmdType::CursorLeft | CmdType::MouseScrollUp | CmdType::MouseScrollDown => {
                Log::debug("input_comple.window 111", &&self.input_comple.widget);
                match cmd.cmd_type {
                    CmdType::CursorDown | CmdType::MouseScrollDown => self.input_comple.widget.cur_move(Direction::Down),
                    CmdType::CursorUp | CmdType::MouseScrollUp => self.input_comple.widget.cur_move(Direction::Up),
                    CmdType::CursorRight => self.input_comple.widget.cur_move(Direction::Right),
                    CmdType::CursorLeft => self.input_comple.widget.cur_move(Direction::Left),
                    _ => {}
                }
                //  self.draw_range =  self.input_comple.widget.get_draw_range_y();
                self.input_comple.widget.set_offset_y(InputComple::MAX_HEIGHT);

                return ActType::Draw(DParts::InputComple);
            }
            CmdType::InsertRow => {
                let act_type = self.select_input_comple(); //  EvtAct::select_input_comple(term);
                self.clear_input_comple();
                return act_type;
            }
            _ => {}
        };
        if self.cmd.config.is_edit {
            self.edit_proc(self.cmd.clone());
        };
        match cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::DelPrevChar => self.init_input_comple(false),
            _ if self.cmd.config.is_edit => {
                self.state.input_comple_mode = InputCompleMode::None;
                return ActType::Draw(DParts::All);
            }
            _ => return ActType::Next,
        }
    }

    pub fn replace_input_comple(&mut self, replace_str: String) {
        let (search_str, str_sx) = self.get_until_delim_str();
        let s_idx = self.buf.row_to_char(self.win_mgr.curt().cur.y) + str_sx;

        self.edit_proc(Cmd::to_cmd(CmdType::ReplaceExec(search_str, replace_str, BTreeSet::from([s_idx]))));
    }

    pub fn clear_input_comple(&mut self) {
        self.input_comple.clear();
        self.state.input_comple_mode = InputCompleMode::None;
    }

    pub fn get_until_delim_str(&self) -> (String, usize) {
        let y = self.win_mgr.curt_ref().cur.y;
        let sx = get_until_delim_sx(&self.buf.char_vec_row(y)[..self.win_mgr.curt_ref().cur.x]);
        return (self.buf.char_vec_row(y)[sx..self.win_mgr.curt_ref().cur.x].iter().collect::<String>(), sx);
    }

    pub fn is_input_imple_mode(&self, is_curt: bool) -> bool {
        if is_curt {
            self.state.input_comple_mode == InputCompleMode::WordComple
        // org
        } else {
            self.state.input_comple_mode_org == InputCompleMode::WordComple
        }
    }
    pub fn select_input_comple(&mut self) -> ActType {
        Log::debug_key("select_input_comple");
        if let Some((menu, _)) = self.input_comple.widget.get_curt_parent() {
            self.replace_input_comple(menu.name);
            return ActType::Draw(DParts::All);
        }
        return ActType::Cancel;
    }
}
