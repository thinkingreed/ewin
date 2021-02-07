use anyhow::{bail, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{collections::BTreeMap, ffi::OsStr};
use syntect::highlighting::{Theme, ThemeSet};

pub struct ThemeLoader {
    theme_path: String,
    themes: BTreeMap<String, Theme>,
    theme: Option<Theme>,
}

impl ThemeLoader {
    pub fn new(theme_path: &String, themes: &BTreeMap<String, Theme>) -> ThemeLoader {
        ThemeLoader {
            theme_path: theme_path.clone(),
            theme: None,
            themes: themes.clone(),
        }
    }

    /// Consumes the ThemeLoader to Theme.
    pub fn load(mut self) -> anyhow::Result<Theme> {
        self.load_user()?;
        if self.theme.is_none() {
            self.load_defaults()?;
        }
        Ok(self.theme.unwrap())
    }

    fn load_user(&mut self) -> anyhow::Result<()> {
        let theme_path = Path::new(&self.theme_path);
        if theme_path.extension() == Some(OsStr::new("tmTheme")) {
            if let Ok(theme) = File::open(&theme_path) {
                let mut reader = BufReader::new(theme);
                if let Ok(theme) = ThemeSet::load_from_reader(&mut reader) {
                    self.theme = Some(theme);
                } else {
                    bail!("Failed to load theme_path : {:?} ", theme_path.to_str());
                }
            }
        }
        Ok(())
    }

    fn load_defaults(&mut self) -> Result<()> {
        self.theme = Some(self.themes["base16-eighties.dark"].clone());
        Ok(())
    }
}
