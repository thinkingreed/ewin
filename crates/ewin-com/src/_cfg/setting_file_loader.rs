use crate::{log::*, model::*};
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    fs::{self},
    path::PathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingFileLoader<'a> {
    file_type: FileType,
    args: &'a AppArgs,
    filedir_opt: Option<PathBuf>,
    filenm: &'a str,
}

impl<'a> SettingFileLoader<'a> {
    pub fn new(file_type: FileType, args: &'a AppArgs, filedir_opt: Option<PathBuf>, filenm: &'a str) -> SettingFileLoader<'a> {
        SettingFileLoader { file_type, args, filedir_opt, filenm }
    }

    pub fn load<T>(&mut self) -> (T, String)
    where
        T: DeserializeOwned + Default + Debug + Clone,
    {
        let mut read_str = String::new();
        let mut err_str = String::new();
        let mut setting: T = T::default();

        if let Some(mut file_path) = self.filedir_opt.clone() {
            file_path.push(self.filenm);
            Log::debug("app_path", &file_path);

            match fs::read_to_string(&file_path) {
                Ok(str) => read_str = str,
                Err(e) => match e.kind() {
                    std::io::ErrorKind::NotFound => return (setting, err_str),
                    _ => err_str = format!("{} {} {}", "file loading failed", file_path.display(), e),
                },
            }
            if err_str.is_empty() {
                let result = self.parse_setting_file(&read_str);
                match result {
                    Ok(s) => setting = s,
                    Err(e) => err_str = format!("{}{} {}", "file parsing failed", file_path.display(), e),
                };
            }
            Log::debug("err_str", &err_str);
        } else {
            err_str = "Config dir not found".to_string();
        }
        return (setting, err_str);
    }

    pub fn parse_setting_file<T>(&mut self, read_str: &str) -> Result<T, anyhow::Error>
    where
        T: DeserializeOwned,
    {
        let result = match self.file_type {
            FileType::JSON5 => json5::from_str::<T>(read_str)?,
            _ => toml::from_str::<T>(read_str)?,
        };
        Ok(result)
    }
}
