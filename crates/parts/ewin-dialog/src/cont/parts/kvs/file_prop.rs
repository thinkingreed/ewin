use crate::{cont::cont::*, dialog_trait::dialog_trait::*};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::def::*;
use ewin_key::util::*;
use ewin_state::tabs::*;
use num_format::{Locale, ToFormattedString};
use std::path::MAIN_SEPARATOR;

impl DialogContTrait for DialogContFileProp {
    fn create_cont_vec(&mut self) -> Vec<String> {
        let mut key_vec = vec![Lang::get().place.to_string(), Lang::get().size.to_string(), Lang::get().create_time.to_string(), Lang::get().mod_time.to_string()];

        let (key_vec, key_max_width) = DialogContKVS::create_key_vec(&mut key_vec);
        let mut kvs_vec = self.get_kvs_vec(key_vec.clone(), self.base.cfg.max_width - key_max_width);

        return DialogContKVS::create_kvs_vec(&mut kvs_vec, key_max_width, self.as_base().cfg.min_width);
    }

    fn as_base(&self) -> &DialogContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut DialogContBase {
        &mut self.base
    }
}

impl DialogContFileProp {
    pub fn get_kvs_vec(&mut self, key_vec: Vec<String>, value_max_width: usize) -> Vec<(String, Vec<String>)> {
        let file = &Tabs::get().curt_h_file().file.clone();
        let mut kvs_vec = vec![];
        let unsettled = if file.fullpath.is_empty() { Lang::get().unsettled.to_string() } else { "".to_string() };
        for key in key_vec {
            if key.contains(&Lang::get().place) {
                Log::debug("file.fullpath", &file.fullpath);
                let path_vec = if file.fullpath.is_empty() { vec![unsettled.clone()] } else { split_tgt_width(&file.fullpath, &[MAIN_SEPARATOR], value_max_width) };
                kvs_vec.push((key, path_vec));
            } else if key.contains(&Lang::get().size) {
                let mut byte_str = String::new();
                if file.fullpath.is_empty() {
                    byte_str = unsettled.clone();
                } else {
                    fmt_bytes(file.len);
                    if !byte_str.contains(BYTE_KEY) {
                        byte_str.push_str(&format!("({} bytes)", file.len.to_formatted_string(&Locale::en)));
                    }
                }
                kvs_vec.push((key, vec![byte_str]));
            } else if key.contains(&Lang::get().create_time) {
                kvs_vec.push((key, vec![if file.fullpath.is_empty() { unsettled.clone() } else { time_to_str(file.create_time) }]));
            } else if key.contains(&Lang::get().mod_time) {
                kvs_vec.push((key, vec![if file.fullpath.is_empty() { unsettled.clone() } else { time_to_str(file.mod_time) }]));
            }
        }
        return kvs_vec;
    }
}

#[derive(Default, Debug, Clone)]
pub struct DialogContFileProp {
    pub base: DialogContBase,
}
