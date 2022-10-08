use crate::{lang::lang_cfg::*, model::color::default::*};
use anyhow::Result;
use std::{collections::BTreeMap, fs::File, io::BufReader, path::Path};
use syntect::highlighting::{Theme, ThemeSet};

pub struct ThemeLoader {
    theme_default_type: String,
    highlight_theme_path: Option<String>,
    highlight_themes: BTreeMap<String, Theme>,
    highlight_theme: Option<Theme>,
}

impl ThemeLoader {
    pub fn new(theme_default_type: String, theme_path: &Option<String>, themes: &BTreeMap<String, Theme>) -> ThemeLoader {
        ThemeLoader { theme_default_type, highlight_theme_path: theme_path.clone(), highlight_theme: None, highlight_themes: themes.clone() }
    }

    /// Consumes the ThemeLoader to Theme.
    pub fn load(mut self) -> anyhow::Result<(Theme, String)> {
        let mut err_str = String::new();

        if self.highlight_theme_path.is_none() {
            err_str = self.load_user();
        }

        if self.highlight_theme_path.is_none() || !err_str.is_empty() {
            self.load_defaults()?;
        }
        Ok((self.highlight_theme.unwrap(), err_str))
    }

    fn load_user(&mut self) -> String {
        let mut err_str = "".to_string();

        if let Some(theme_path) = &self.highlight_theme_path {
            let theme_path = Path::new(&theme_path);
            if theme_path.exists() {
                if let Ok(theme) = File::open(&theme_path) {
                    let mut reader = BufReader::new(theme);
                    match ThemeSet::load_from_reader(&mut reader) {
                        Ok(theme) => self.highlight_theme = Some(theme),
                        Err(e) => {
                            err_str = format!("{} {} {}", Lang::get().file_loading_failed, theme_path.to_string_lossy(), e);
                        }
                    }
                }
            } else {
                err_str = format!("{} {}", Lang::get().file_not_found, theme_path.to_string_lossy());
            }
        }
        err_str
    }

    fn load_defaults(&mut self) -> Result<()> {
        self.highlight_theme = match ColorSchemeThemeType::from_str_color_type(&self.theme_default_type.to_lowercase()) {
            ColorSchemeThemeType::Black => Some(self.highlight_themes["base16-eighties.dark"].clone()),
            ColorSchemeThemeType::White => Some(self.highlight_themes["InspiredGitHub"].clone()),
        };
        Ok(())
    }
}
