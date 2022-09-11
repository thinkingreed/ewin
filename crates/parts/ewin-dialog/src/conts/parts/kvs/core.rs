use crate::{conts::cont::*, dialog::*};
use ewin_cfg::log::*;
use ewin_const::models::view::*;
use ewin_utils::str_edit::*;
use std::cmp::max;

impl DialogContKVS {
    pub fn create_key_vec(key_vec: &mut Vec<String>) -> (&mut Vec<String>, usize) {
        let mut key_max_width = get_strs_max_width(key_vec);
        adjust_width_str(key_vec, key_max_width);
        let add_str = ":";
        key_max_width += get_str_width(add_str);

        for s in key_vec.iter_mut() {
            s.push_str(add_str);
        }
        return (key_vec, key_max_width);
    }

    pub fn create_kvs_vec(kvs_vec: &mut Vec<(String, Vec<String>)>, key_max_width: usize, min_width: usize) -> Vec<String> {
        Log::debug_key("create_kvs_vec");
        // value
        let cont_width = DialogContKVS::get_kvs_cont_size(kvs_vec).1;
        let cont_width = max(cont_width, min_width);

        Log::debug("cont_width", &cont_width);

        Log::debug("key_max_width", &key_max_width);

        for (_, vec) in kvs_vec.iter_mut() {
            adjust_width_str(vec, cont_width - key_max_width);
        }
        return DialogContKVS::fmt_cont_vec(key_max_width, kvs_vec);
    }

    pub fn get_kvs_cont_size(value_vec: &Vec<(String, Vec<String>)>) -> (usize, usize) {
        let mut height = 0;
        let mut width = 0;
        let mut max_value_width = 0;
        for (idx, (key, vec)) in value_vec.iter().enumerate() {
            for value in vec {
                let value_width = get_str_width(value);
                max_value_width = if max_value_width > value_width { max_value_width } else { value_width };
            }
            if idx == value_vec.len() - 1 {
                width += get_str_width(key) + max_value_width;
            }
            height += vec.len();
        }
        return (height, width);
    }

    pub fn fmt_cont_vec(key_max_width: usize, kvs_vec: &Vec<(String, Vec<String>)>) -> Vec<String> {
        let mut rtn_vec = vec![];
        let margin_width = Dialog::CONT_MARGIN_WIDTH / 2;

        for (key, vec) in kvs_vec {
            for (j, value) in vec.iter().enumerate() {
                let key = if j == 0 { key.clone() } else { get_space(key_max_width) };
                rtn_vec.push(format!("{}{}{}{}", get_space(margin_width), key, value, get_space(margin_width)));
            }
        }
        return rtn_vec;
    }
}
