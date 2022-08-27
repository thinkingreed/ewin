use crate::{conts::cont::*, dialog_traits::dialog_trait::*};
use chrono::Local;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_key::key::cmd::*;
use ewin_utils::str_edit::*;

impl DialogContTrait for DialogContAboutApp {
    fn create_cont_vec(&mut self) -> Vec<String> {
        Log::debug_key("DialogContAboutApp.create_cont_vec");

        let mut key_vec = vec![DialogContAboutApp::VERSION.to_string(), Lang::get().simple_help_desc.to_string(), Lang::get().detailed_help_desc.to_string(), DialogContAboutApp::COPYRIGHT.to_string()];

        Log::debug("key_vec", &key_vec);

        let (key_vec, key_max_width) = DialogContKVS::create_key_vec(&mut key_vec);

        Log::debug("self.base.cfg.max_width", &self.base.cfg.max_width);
        Log::debug("key_max_width", &key_max_width);

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

impl DialogContAboutApp {
    pub const VERSION: &'static str = "Version";
    pub const COPYRIGHT: &'static str = "Copyright(C)";

    pub fn get_kvs_vec(&mut self, key_vec: Vec<String>, value_max_width: usize) -> Vec<(String, Vec<String>)> {
        let mut kvs_vec = vec![];
        for key in key_vec {
            if key.contains(&DialogContAboutApp::VERSION) {
                kvs_vec.push((key, vec![env!("CARGO_PKG_VERSION").to_string()]));
            } else if key.contains(&Lang::get().simple_help_desc.to_string()) {
                Cmd::cmd_to_keys(CmdType::Help);
            } else if key.contains(&Lang::get().detailed_help_desc) {
                let mut repository = split_tgt_str_width(env!("CARGO_PKG_REPOSITORY"), &['/'], value_max_width);
                let wiki = split_tgt_str_width(&format!("{}{}", env!("CARGO_PKG_REPOSITORY"), "/wiki"), &['/'], value_max_width);
                repository.extend(wiki);
                // let path_vec = if file.fullpath.is_empty() { vec![unsettled.clone()] } else { split_tgt_width(&file.fullpath, &[MAIN_SEPARATOR], value_max_width) };

                kvs_vec.push((key, repository));
            } else if key.contains(&DialogContAboutApp::COPYRIGHT) {
                kvs_vec.push((key, vec![format!("{}{} {}", "2021-", Local::now().format("%Y"), env!("CARGO_PKG_AUTHORS"))]));
            }
        }

        Log::debug("kvs_vec", &kvs_vec);
        return kvs_vec;
    }
}

#[derive(Default, Debug, Clone)]
pub struct DialogContAboutApp {
    pub base: DialogContBase,
}
