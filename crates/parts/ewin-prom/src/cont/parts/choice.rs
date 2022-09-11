use crate::{model::*, prom_trait::cont_trait::*};
use crossterm::cursor::*;
use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use ewin_const::{def::*, models::view::*};
use ewin_key::key::cmd::CmdType;
use ewin_utils::str_edit::*;
use std::{
    cmp::max,
    collections::{hash_map::Entry::*, *},
    usize,
};

impl PromContChoice {
    pub const ITEM_MARGIN: usize = 1;

    pub fn set_cont_posi_or_is_up_down_cont_posi(&mut self) -> bool {
        let mut is_up_down_cont_posi = false;
        if matches!(self.base.cmd.cmd_type, CmdType::CursorUp) {
            if self.vec_y == 0 {
                is_up_down_cont_posi = true;
            } else if let Some(vec) = self.vec.get(self.vec_y - 1) {
                if vec.get(self.vec_x).is_some() {
                    self.vec_y -= 1;
                } else {
                    self.vec_y -= 1;
                    self.vec_x = vec.len() - 1;
                }
            } else {
                is_up_down_cont_posi = true;
            }
        } else if matches!(self.base.cmd.cmd_type, CmdType::CursorDown) {
            if let Some(vec) = self.vec.get(self.vec_y + 1) {
                if vec.get(self.vec_x).is_some() {
                    self.vec_y += 1;
                } else {
                    is_up_down_cont_posi = true;
                }
            } else {
                is_up_down_cont_posi = true;
            }
        }
        is_up_down_cont_posi
    }

    pub fn move_left_right(&mut self, cmd_type: &CmdType) {
        if let Some(vec) = self.vec.get(self.vec_y) {
            if matches!(cmd_type, CmdType::CursorRight) {
                self.vec_x = if vec.get(self.vec_x + 1).is_some() { self.vec_x + 1 } else { 0 };
            } else if self.vec_x == 0 {
                if vec.last().is_some() {
                    self.vec_x = vec.len() - 1;
                }
            } else if matches!(cmd_type, CmdType::CursorLeft) {
                self.vec_x = if vec.get(self.vec_x - 1).is_some() { self.vec_x - 1 } else { 0 };
            }
        }
    }

    pub fn set_choice_area(buf_row_posi: u16, choices_map: &mut HashMap<((usize, usize), (usize, usize)), PromContChoice>) {
        for (_, choices) in choices_map.iter_mut() {
            for (y_idx, v) in choices.vec.iter_mut().enumerate() {
                let mut row_width = 1;

                for item in v.iter_mut() {
                    let item_len = get_str_width(&item.disp_name);
                    item.area = (buf_row_posi as usize + y_idx, row_width, row_width + item_len - 1);
                    row_width += item_len + PromContChoice::ITEM_MARGIN;
                }
            }
        }
    }

    pub fn set_shaping_choice_list(&mut self) {
        Log::debug_key("set_shaping_choice_list");

        let mut map: HashMap<usize, usize> = HashMap::new();
        for vecs in self.vec.iter() {
            for (column, choice) in vecs.iter().enumerate() {
                match map.entry(column) {
                    Occupied(mut e) => {
                        e.insert(max(*e.get(), get_str_width(&choice.name)));
                    }
                    Vacant(e) => {
                        e.insert(get_str_width(&choice.name));
                    }
                }
            }
        }
        for (column, max_width) in &map {
            for vecs in self.vec.iter_mut() {
                if let Some(choice) = vecs.get_mut(*column) {
                    let rest = max_width - get_str_width(&choice.name);
                    choice.disp_name = format!("{}{}", choice.name, get_space(rest))
                }
            }
        }
    }

    pub fn get_choice(&self) -> Choice {
        let dummy = Choice::new("");
        for (y_idx, v) in self.vec.iter().enumerate() {
            for (x_idx, item) in v.iter().enumerate() {
                if self.vec_y == y_idx && self.vec_x == x_idx {
                    return item.clone();
                }
            }
        }
        dummy
    }
    pub fn set_bom(&mut self, is_check: bool) {
        Log::debug_key("PromContChoice.set_bom");

        for (y_idx, vec) in self.vec.iter().enumerate() {
            for (x_idx, choice) in vec.iter().enumerate() {
                if is_check && choice.name == Lang::get().with || !is_check && choice.name == Lang::get().without {
                    self.vec_y = y_idx;
                    self.vec_x = x_idx;
                }
            }
        }
    }
    pub fn click_choice(&mut self, y: u16, x: u16) -> bool {
        Log::debug_key("left_down_choice_menu");
        Log::debug("yyy", &y);
        Log::debug("xxx", &x);

        let (y, x) = (y as usize, x as usize);
        for (y_idx, vec) in self.vec.iter().enumerate() {
            for (x_idx, item) in vec.iter().enumerate() {
                Log::debug("item.name", &item.name);
                Log::debug("item.area", &item.area);
                if item.area.0 == y && item.area.1 <= x && x <= item.area.2 {
                    Log::debug_key("click!!!!!");
                    self.vec_y = y_idx;
                    self.vec_x = x_idx;
                    return true;
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
// DrawRange
pub struct Choice {
    pub name: String,
    pub disp_name: String,
    pub area: (usize, usize, usize),
}

impl Default for Choice {
    fn default() -> Self {
        Choice { disp_name: String::new(), name: String::new(), area: (USIZE_UNDEFINED, USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}

impl Choice {
    pub fn new(name: &str) -> Self {
        Choice { name: name.to_string(), ..Choice::default() }
    }
    pub fn is_none(&self) -> bool {
        self.name.is_empty()
    }
}

impl PromContTrait for PromContChoice {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, is_curt: bool) {
        Log::debug_key("PromContChoice.draw");
        Log::debug("self.base.row_posi_range.start", &self.base.row_posi_range.start);

        if self.is_disp {
            for disp_str in &self.desc_str_vec {
                str_vec.push(format!("{}{}{}{}", MoveTo(0, self.base.row_posi_range.start as u16), if is_curt { Colors::get_msg_highlight_inversion_fg_bg() } else { Colors::get_msg_highlight_fg() }, &disp_str, Colors::get_default_fg_bg()));
            }
            let start_idx = self.base.row_posi_range.start + self.desc_str_vec.len();

            for (y_idx, choice_vec) in self.vec.iter().enumerate() {
                let mut row_width = PromContChoice::ITEM_MARGIN;
                for (x_idx, choice) in choice_vec.iter().enumerate() {
                    let enable_choice = self.vec_y == y_idx && self.vec_x == x_idx;

                    let choice_str = if enable_choice { format!("{}{}{}", Colors::get_msg_warning_inversion_fg_bg(), choice.disp_name, Colors::get_default_fg_bg()) } else { format!("{}{}", Colors::get_default_fg_bg(), choice.disp_name) };
                    str_vec.push(format!("{}{}", MoveTo(row_width as u16, (start_idx + y_idx) as u16), &choice_str));

                    row_width += get_str_width(&choice.disp_name) + PromContChoice::ITEM_MARGIN;
                }
            }
        }
    }

    fn check_allow_p_cmd(&self) -> bool {
        return match self.as_base().cmd.cmd_type {
            CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorDown | CmdType::CursorUp | CmdType::NextContent | CmdType::BackContent => true,
            CmdType::MouseDownLeft(y, _) if self.base.row_posi_range.start <= y && y <= self.base.row_posi_range.end => true,
            _ => false,
        };
    }
}

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContChoice {
    pub base: PromptContBase,
    pub config: PromContChoiceConfig,
    pub desc_str_vec: Vec<String>,
    pub vec: Vec<Vec<Choice>>,
    pub is_disp: bool,
    pub vec_y: usize,
    pub vec_x: usize,
}
