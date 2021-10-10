use crate::{ewin_core::log::*, model::*, prompt::choice::*};
use crossterm::cursor::MoveTo;
use std::usize;

impl PromptCont {
    pub fn left_down_choice(&mut self, y: u16, x: u16) -> bool {
        Log::debug_key("left_down_choice_menu");

        let (y, x) = (y as usize, x as usize);
        for (_, choices) in self.choices_map.iter_mut() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in vec.iter().enumerate() {
                        Log::debug("item", &item);
                        if item.area.0 == y && item.area.1 <= x && x <= item.area.2 {
                            Log::debug_key("item.area.0");
                            choices.vec_y = y_idx;
                            choices.vec_x = x_idx;
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
    pub fn draw_choice_cur(&self, str_vec: &mut Vec<String>) {
        Log::debug_key("draw_choice_cur");

        let (mut y, mut x) = (0, 0);
        'outer: for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                for (y_idx, vec) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in vec.iter().enumerate() {
                        if choices.vec_y == y_idx && choices.vec_x == x_idx {
                            y = self.buf_row_posi + y_idx as u16;
                            x = item.area.1;
                            break 'outer;
                        }
                    }
                }
            }
        }
        Log::debug("x", &x);
        Log::debug("y", &y);

        str_vec.push(MoveTo(x as u16, y as u16).to_string());
    }

    pub fn get_choices(&mut self) -> Option<&mut Choices> {
        for (_, choices) in self.choices_map.iter_mut() {
            if choices.is_show {
                return Some(choices);
            }
        }
        // dummy
        return None;
    }

    pub fn get_choice(&self) -> Choice {
        let dummy = Choice::new(&"".to_string());
        for (_, choices) in self.choices_map.iter() {
            if choices.is_show {
                for (y_idx, v) in choices.vec.iter().enumerate() {
                    for (x_idx, item) in v.iter().enumerate() {
                        if choices.vec_y == y_idx && choices.vec_x == x_idx {
                            return item.clone();
                        }
                    }
                }
            }
        }
        return dummy;
    }
}
