use crate::{def::*, log::Log, model::*};
use directories::BaseDirs;
use std::{env, fs, path::PathBuf};
use whoami::username;

impl FilePath {
    pub fn get_app_tmp_path() -> PathBuf {
        let mut path = env::temp_dir();
        path.push(format!("{}_{}", &APP_NAME, &username()));

        return path;
    }
    pub fn get_app_clipboard_file_path() -> PathBuf {
        let tmp_path = FilePath::get_app_tmp_path();
        let clipboard_file = &tmp_path.join(CLIPBOARD_FILE);
        return clipboard_file.clone();
    }

    pub fn get_app_config_file_path() -> Option<PathBuf> {
        if let Some(app_dir) = FilePath::get_app_config_path() {
            let config_file = &app_dir.join(SETTING_FILE);
            return Some(config_file.clone());
        } else {
            return None;
        }
    }

    pub fn get_app_config_path() -> Option<PathBuf> {
        if let Some(base_dirs) = BaseDirs::new() {
            let config_dir = base_dirs.config_dir();
            let app_dir = config_dir.join(APP_NAME);
            if let Err(err) = fs::create_dir_all(&app_dir) {
                Log::error("config_app_path create_dir_all err", &err);
                return None;
            };
            return Some(app_dir);
        } else {
            return None;
        }
    }
}
