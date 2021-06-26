extern crate ropey;
use crate::{def::*, log::Log, model::CurDirection, prompt::cont::promptcont::*, util::get_str_width};
use std::{cmp::max, collections::HashMap, usize};

impl Choices {
    pub const ITEM_MARGIN: usize = 1;

    pub fn set_vec_posi(&mut self, cur_direction: CurDirection) -> bool {
        let mut is_move_cont = false;
        match cur_direction {
            CurDirection::Right | CurDirection::Left => {
                if let Some(vec) = self.vec.get(self.vec_y) {
                    if cur_direction == CurDirection::Right {
                        self.vec_x = if vec.get(self.vec_x + 1).is_some() { self.vec_x + 1 } else { 0 };
                    } else {
                        if self.vec_x == 0 {
                            if let Some(_) = vec.get(vec.len() - 1) {
                                self.vec_x = vec.len() - 1;
                            };
                        } else {
                            self.vec_x = if vec.get(self.vec_x - 1).is_some() { self.vec_x - 1 } else { 0 };
                        }
                    }
                }
            }
            CurDirection::Up => {
                if self.vec_y == 0 {
                    is_move_cont = true;
                } else {
                    if let Some(vec) = self.vec.get(self.vec_y - 1) {
                        if let Some(_) = vec.get(self.vec_x) {
                            self.vec_y -= 1;
                        } else {
                            is_move_cont = true;
                        }
                    } else {
                        is_move_cont = true;
                    }
                }
            }
            CurDirection::Down => {
                if let Some(vec) = self.vec.get(self.vec_y + 1) {
                    if let Some(_) = vec.get(self.vec_x) {
                        self.vec_y += 1;
                    } else {
                        is_move_cont = true;
                    }
                } else {
                    is_move_cont = true;
                }
            }
        }
        return is_move_cont;
    }

    pub fn get_y_x(prompt_cont: &PromptCont) -> (usize, usize) {
        let (dummy_y, dummy_x) = (USIZE_UNDEFINED, USIZE_UNDEFINED);
        for (_, choices) in prompt_cont.choices_map.iter() {
            if !choices.is_show {
                continue;
            }
            for (y_idx, v) in choices.vec.iter().enumerate() {
                for (x_idx, _) in v.iter().enumerate() {
                    if choices.is_show && choices.vec_y == y_idx && choices.vec_x == x_idx {
                        return (y_idx, x_idx);
                    }
                }
            }
        }
        return (dummy_y, dummy_x);
    }
    pub fn set_choice_area(buf_row_posi: u16, choices_map: &mut HashMap<(usize, usize), Choices>) {
        for (_, choices) in choices_map.iter_mut() {
            for (y_idx, v) in choices.vec.iter_mut().enumerate() {
                let mut row_width = 1;

                for item in v.iter_mut() {
                    let item_len = get_str_width(&item.disp_name);
                    item.area = (buf_row_posi as usize + y_idx, row_width, row_width + item_len - 1);
                    row_width += item_len + Choices::ITEM_MARGIN;
                }
            }
        }
    }
    pub fn set_show_choice(p_y: usize, p_x: usize, choices_map: &mut HashMap<(usize, usize), Choices>) {
        for ((y, x), choices) in choices_map.iter_mut() {
            choices.is_show = if p_y == *y && p_x == *x { true } else { false };
        }
    }

    pub fn set_shaping_choice_list(choices_map: &mut HashMap<(usize, usize), Choices>) {
        Log::debug_key("set_shaping_choice_list");

        for (_, choices) in choices_map.iter_mut() {
            let mut map: HashMap<usize, usize> = HashMap::new();
            for vecs in choices.vec.iter() {
                for (column, choice) in vecs.iter().enumerate() {
                    if map.contains_key(&column) {
                        let max_width = max(*map.get(&column).unwrap(), get_str_width(&choice.name));
                        map.insert(column, max_width);
                    } else {
                        map.insert(column, get_str_width(&choice.name));
                    }
                }
            }
            for (column, max_width) in &map {
                for vecs in choices.vec.iter_mut() {
                    if let Some(choice) = vecs.get_mut(*column) {
                        let rest = max_width - get_str_width(&choice.name);
                        choice.disp_name = format!("{}{}", choice.name, " ".repeat(rest))
                    }
                }
            }
        }
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
    pub fn new(name: &String) -> Self {
        return Choice { name: name.clone(), ..Choice::default() };
    }
}

#[derive(Debug, Clone)]
pub struct Choices {
    pub is_show: bool,
    pub vec: Vec<Vec<Choice>>,
    // pub idx: usize,
    pub vec_y: usize,
    pub vec_x: usize,
}

impl Default for Choices {
    fn default() -> Self {
        Choices { is_show: false, vec: vec![], vec_y: 0, vec_x: 0 }
    }
}
