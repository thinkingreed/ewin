use crate::_cfg::lang::lang_cfg::*;
use anyhow::Result;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};
use syntect::highlighting::{Theme, ThemeSet};

pub struct ThemeLoader {
    theme_path: Option<String>,
    themes: BTreeMap<String, Theme>,
    theme: Option<Theme>,
}

impl ThemeLoader {
    pub fn new(theme_path: &Option<String>, themes: &BTreeMap<String, Theme>) -> ThemeLoader {
        ThemeLoader { theme_path: theme_path.clone(), theme: None, themes: themes.clone() }
    }

    /// Consumes the ThemeLoader to Theme.
    pub fn load(mut self) -> anyhow::Result<(Theme, String)> {
        let mut err_str = String::new();

        if self.theme_path.is_none() {
            err_str = self.load_user();
        }

        if self.theme_path.is_none() || !err_str.is_empty() {
            self.load_defaults()?;
        }
        Ok((self.theme.unwrap(), err_str))
    }

    fn load_user(&mut self) -> String {
        let mut err_str = "".to_string();

        if let Some(theme_path) = &self.theme_path {
            let theme_path = Path::new(&theme_path);
            if theme_path.exists() {
                if let Ok(theme) = File::open(&theme_path) {
                    let mut reader = BufReader::new(theme);
                    match ThemeSet::load_from_reader(&mut reader) {
                        Ok(theme) => self.theme = Some(theme),
                        Err(e) => {
                            err_str = format!("{} {} {}", Lang::get().file_loading_failed, theme_path.to_string_lossy().to_string(), e);
                        }
                    }
                }
            } else {
                err_str = format!("{} {}", Lang::get().file_not_found, theme_path.to_string_lossy().to_string());
            }
        }
        err_str
    }

    fn load_defaults(&mut self) -> Result<()> {
        self.theme = Some(self.themes["base16-eighties.dark"].clone());
        Ok(())
    }
}
