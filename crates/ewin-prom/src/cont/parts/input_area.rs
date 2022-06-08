use crate::{model::*, prom_trait::cont_trait::*, util::*};
use crossterm::cursor::MoveTo;
use ewin_cfg::{colors::*, log::*};
use ewin_com::{_cfg::key::keycmd::*, model::*, util::*};
use std::cmp::min;

use super::path_comp::PathComp;

impl PromContInputArea {
    pub fn proc_input_area(&mut self) -> ActType {
        Log::debug("self.base.p_cmd", &self.base.p_cmd);

        let act_type = match self.base.p_cmd {
            P_Cmd::InsertStr(_) => {
                if self.config.is_edit_proc_orig {
                    return ActType::Next;
                }
                self.edit_proc(self.base.p_cmd.clone())
            }
            P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut => self.edit_proc(self.base.p_cmd.clone()),
            P_Cmd::Copy => self.copy(),
            P_Cmd::Undo => self.undo(),
            P_Cmd::Redo => self.redo(),
            P_Cmd::NextContent | P_Cmd::BackContent => {
                if self.config.is_path {
                    self.buf = self.path_comp.get_path_candidate(self.base.p_cmd == P_Cmd::NextContent, self.buf[..self.cur.x].iter().collect::<String>(), self.config.is_path_dir_only);
                    self.set_cur_target(self.buf.len());
                    return ActType::Draw(DParts::Prompt);
                }
                return ActType::Next;
            }
            P_Cmd::Confirm => return ActType::Next,
            _ => ActType::Next,
        };
        if act_type != ActType::Next {
            return act_type;
        }
        match self.base.p_cmd {
            P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd => self.cur_move(),
            P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect => self.shift_move_com(),
            P_Cmd::MouseDownLeft(y, x) | P_Cmd::MouseDragLeft(y, x) => {
                if y == self.base.row_posi_range.end {
                    self.ctrl_mouse(y, x);
                } else {
                    return ActType::Next;
                }
            }
            _ => {}
        }

        if is_edit_proc(&self.base.p_cmd) {
            self.path_comp.clear_path_comp();
        }
        match &self.base.p_cmd {
            P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect => {
                self.path_comp.clear_path_comp();
            }
            _ => {}
        };

        if self.config.is_edit_proc_later {
            return ActType::Next;
        }
        return ActType::Draw(DParts::Prompt);
    }

    pub fn set_cur_target(&mut self, x: usize) {
        let (cur_x, width) = get_until_disp_x(&self.buf, x, false);
        self.cur.x = cur_x;
        self.cur.disp_x = width;
    }

    pub fn del_sel_range(&mut self) {
        let sel = self.sel.get_range();
        self.buf.drain(sel.sx..sel.ex);
        self.cur.disp_x = min(sel.s_disp_x, sel.e_disp_x);
        self.cur.x = min(sel.sx, sel.ex);
    }

    pub fn get_draw_buf_str(&self) -> Vec<String> {
        Log::debug_key("PromptCont.get_draw_buf_str");
        let ranges = self.sel.get_range();
        Log::debug("ranges", &ranges);

        let mut str_vec: Vec<String> = vec![];
        for (i, c) in self.buf.iter().enumerate() {
            if ranges.sx <= i && i < ranges.ex {
                str_vec.push(Colors::get_select_fg_bg());
            } else {
                str_vec.push(Colors::get_default_fg_bg());
            }
            str_vec.push(c.to_string())
        }
        str_vec.push(Colors::get_default_fg_bg());
        return vec![str_vec.join("")];
    }
}

impl PromContPluginTrait for PromContInputArea {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, is_curt: bool) {
        for disp_str in &self.desc_str_vec {
            str_vec.push(format!("{}{}", if is_curt { Colors::get_msg_highlight_inversion_fg_bg() } else { Colors::get_msg_highlight_fg() }, &disp_str));
        }
        str_vec.push(MoveTo(0, (self.as_base().row_posi_range.start + self.desc_str_vec.len()) as u16).to_string());
        str_vec.append(&mut self.get_draw_buf_str());
    }

    fn check_allow_p_cmd(&self) -> bool {
        return match self.as_base().p_cmd {
            P_Cmd::InsertStr(_) | P_Cmd::DelNextChar | P_Cmd::DelPrevChar | P_Cmd::Cut | P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::CursorLeftSelect | P_Cmd::CursorRightSelect | P_Cmd::CursorRowHomeSelect | P_Cmd::CursorRowEndSelect | P_Cmd::Copy | P_Cmd::Undo | P_Cmd::Redo => true,
            P_Cmd::MouseDownLeft(y, _) | P_Cmd::MouseDragLeft(y, _) if self.base.row_posi_range.start <= y && y <= self.base.row_posi_range.end => true,
            _ => false,
        };
    }
}

impl PromPluginBase {
    pub fn get_curt_input_area(&mut self) -> Option<&mut PromContInputArea> {
        Log::debug_key("PromptPluginBase.get_curt_input_area");
        Log::debug("self.curt_cont_idx", &self.curt_cont_idx);

        if let Some(item) = self.get_curt_cont_mut() {
            if let Ok(input_area) = item.downcast_mut::<PromContInputArea>() {
                return Some(input_area);
            }
        }
        return None;
    }
    pub fn get_curt_input_area_str(&mut self) -> String {
        Log::debug_key("PromptPluginBase.get_curt_input_area_str");
        return String::from_iter(self.get_curt_input_area().unwrap().buf.clone());
    }

    pub fn get_tgt_input_area(&mut self, tgt_idx: usize) -> Option<&mut PromContInputArea> {
        Log::debug_key("PromptPluginBase.get_tgt_input_area");
        let mut idx = 0;
        for cont in self.cont_vec.iter_mut() {
            if let Ok(input_area) = cont.downcast_mut::<PromContInputArea>() {
                if tgt_idx == idx {
                    return Some(input_area);
                }
                idx += 1;
            }
        }
        return None;
    }

    pub fn get_tgt_input_area_str(&mut self, tgt_idx: usize) -> String {
        Log::debug_key("PromptPluginBase.get_tgt_input_area_str");
        let mut rtn_str = "".to_string();

        if let Some(input_area) = self.get_tgt_input_area(tgt_idx) {
            rtn_str = String::from_iter(input_area.buf.clone());
        }
        return rtn_str;
    }

    pub fn proc_for_input_area(&mut self, f: fn(&mut PromContInputArea)) {
        for item in self.cont_vec.iter_mut() {
            if let Ok(input_area) = item.downcast_mut::<PromContInputArea>() {
                f(input_area);
            }
        }
    }
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContInputArea {
    pub base: PromptContBase,
    pub desc_str_vec: Vec<String>,
    pub config: PromInputAreaConfig,
    pub path_comp: PathComp,
    pub buf: Vec<char>,
    pub cur: Cur,
    pub sel: SelRange,
    pub history: History,
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromInputAreaConfig {
    pub is_path: bool,
    pub is_path_dir_only: bool,
    pub is_edit_proc_later: bool,
    pub is_edit_proc_orig: bool,
}
