use super::cmd::*;
use crate::{
    global::*,
    key::{keys::*, keywhen::*},
    util::*,
};
use ewin_cfg::{
    lang::lang_cfg::*,
    log::*,
    model::{default::*, modal::*},
    setting_file_loader::*,
};
use ewin_const::def::*;
use json5;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};

impl Keybind {
    pub fn init(args: &AppArgs) -> String {
        #[cfg(target_family = "unix")]
        let mut keybind_vec: Vec<Keybind> = json5::from_str(include_str!("../../../../../setting/keybind/keybind_unix_family.json5")).unwrap();
        #[cfg(target_family = "windows")]
        let mut keybind_vec: Vec<Keybind> = json5::from_str(include_str!("../../../../../setting/keybind/keybind_windows.json5")).unwrap();

        let (mut keybind_vec_user, err_str) = SettingFileLoader::new(FileType::JSON5, args, CFgFilePath::get_app_config_dir(), KEYBINDING_FILE).load::<Vec<Keybind>>();

        if !err_str.is_empty() {
            return err_str;
        }
        Cfg::write_setting_file(args, KEYBINDING_FILE, &json5::to_string(&keybind_vec).unwrap());

        for (i, keybind) in keybind_vec_user.iter().enumerate() {
            let err_str = Keybind::check_keybind_file(keybind, i);
            if !err_str.is_empty() {
                return err_str;
            }
        }
        keybind_vec.append(&mut keybind_vec_user);

        let mut key_when_cmd_map: HashMap<(Keys, KeyWhen), Cmd> = HashMap::new();
        let mut cmd_type_key_map: HashMap<CmdType, Keys> = HashMap::new();

        for keybind in keybind_vec {
            Log::debug("", &keybind);
            Log::debug("Keys::from_str", &Keys::from_str(&keybind.key).unwrap());

            cmd_type_key_map.insert(Cmd::str_to_cmd_type(&keybind.cmd), Keys::from_str(&keybind.key).unwrap());
            let cmd = Cmd::str_to_cmd(&keybind.cmd);

            for when in cmd.when_vec {
                key_when_cmd_map.insert((Keys::from_str(&keybind.key).unwrap(), when), Cmd::str_to_cmd(&keybind.cmd));
            }
        }

        Log::debug("cmd_map", &key_when_cmd_map);
        Log::debug("cmd_type_map", &cmd_type_key_map);
        let _ = CMD_MAP.set(key_when_cmd_map);
        let _ = CMD_TYPE_MAP.set(cmd_type_key_map);

        return err_str;
    }

    pub fn get_menu_str(menunm: &str, cmd_type: CmdType) -> String {
        let str = Keys::get_key_str(cmd_type);
        let key_str = if str.is_empty() { "".to_string() } else { format!("({})", str) };
        return format!("{}{}", menunm, key_str);
    }

    pub fn check_keybind_file(keybind: &Keybind, i: usize) -> String {
        let mut msg = &String::new();
        let mut err_key = &String::new();
        let mut err_str = String::new();

        if Keys::from_str(&keybind.key).is_err() {
            msg = &Lang::get().specification_err_key;
            err_key = &keybind.key;
        } else if Cmd::str_to_cmd_type(&keybind.cmd) == CmdType::Unsupported {
            msg = &Lang::get().specification_err_keycmd;
            err_key = &keybind.cmd;
        }

        if !msg.is_empty() {
            err_str = format!("{}{} {} {} setting {}{} {}", Lang::get().file, Lang::get().parsing_failed, KEYBINDING_FILE, msg, (i + 1), ordinal_suffix(i + 1), err_key);
        }
        err_str
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Keybind {
    pub key: String,
    pub cmd: String,
}
