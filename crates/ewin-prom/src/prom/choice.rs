use crate::{
    ewin_com::def::*,
    ewin_com::log::*,
    ewin_com::model::Direction,
    ewin_com::util::*,
    model::{Prompt, PromptCont},
};
use std::{cmp::max, collections::HashMap, usize};

impl Choices {
    pub const ITEM_MARGIN: usize = 1;

    pub fn set_vec_posi(&mut self, cur_direction: Direction) -> bool {
        let mut is_updown_contposi = false;
        match cur_direction {
            Direction::Right | Direction::Left => {
                if let Some(vec) = self.vec.get(self.vec_y) {
                    if cur_direction == Direction::Right {
                        self.vec_x = if vec.get(self.vec_x + 1).is_some() { self.vec_x + 1 } else { 0 };
                    } else if self.vec_x == 0 {
                        if vec.get(vec.len() - 1).is_some() {
                            self.vec_x = vec.len() - 1;
                        }
                    } else {
                        self.vec_x = if vec.get(self.vec_x - 1).is_some() { self.vec_x - 1 } else { 0 };
                    }
                }
            }
            Direction::Up => {
                if self.vec_y == 0 {
                    is_updown_contposi = true;
                } else if let Some(vec) = self.vec.get(self.vec_y - 1) {
                    if vec.get(self.vec_x).is_some() {
                        self.vec_y -= 1;
                    } else {
                        is_updown_contposi = true;
                    }
                } else {
                    is_updown_contposi = true;
                }
            }
            Direction::Down => {
                if let Some(vec) = self.vec.get(self.vec_y + 1) {
                    if vec.get(self.vec_x).is_some() {
                        self.vec_y += 1;
                    } else {
                        is_updown_contposi = true;
                    }
                } else {
                    is_updown_contposi = true;
                }
            }
        }
        is_updown_contposi
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
        (dummy_y, dummy_x)
    }

    pub fn set_choice_area(buf_row_posi: u16, choices_map: &mut HashMap<((usize, usize), (usize, usize)), Choices>) {
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
    pub fn change_show_choice(prom: &mut Prompt) {
        let (first_y, first_x) = Choices::get_y_x(&prom.cont_1);
        Choices::set_show_choice(USIZE_UNDEFINED, USIZE_UNDEFINED, first_y, first_x, &mut prom.cont_2.choices_map);
        let (second_y, second_x) = Choices::get_y_x(&prom.cont_2);
        Choices::set_show_choice(first_y, first_x, second_y, second_x, &mut prom.cont_3.choices_map);
    }

    pub fn set_show_choice(grandparents_y: usize, grandparents_x: usize, parent_y: usize, parent_x: usize, choices_map: &mut HashMap<((usize, usize), (usize, usize)), Choices>) {
        for (((gp_y, gp_x), (p_y, p_x)), choices) in choices_map.iter_mut() {
            choices.is_show = grandparents_y == *gp_y && grandparents_x == *gp_x && parent_y == *p_y && parent_x == *p_x;
        }
    }

    pub fn set_shaping_choice_list(choices_map: &mut HashMap<((usize, usize), (usize, usize)), Choices>) {
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
    pub fn new(name: &str) -> Self {
        Choice { name: name.to_string(), ..Choice::default() }
    }
    pub fn is_none(&self) -> bool {
        self.name.is_empty()
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
