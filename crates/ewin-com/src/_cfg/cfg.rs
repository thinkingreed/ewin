use crate::{
    _cfg::model::{default::*, user::*},
    _cfg::{setting_file_loader::*, theme_loader::*},
    colors::*,
    def::*,
    file::File,
    global::*,
    log::*,
    model::*,
};
use std::{path::Path, sync::Mutex};

impl Cfg {
    pub fn init(args: &AppArgs) -> String {
        let mut cfg: Cfg = toml::from_str(include_str!("../../../../setting/setting.toml")).unwrap();

        let (cfg_user, err_str) = SettingFileLoader::new(FileType::TOML, args, FilePath::get_app_config_dir(), SETTING_FILE).load::<CfgUser>();
        if !err_str.is_empty() {
            return err_str;
        }

        cfg.set_user_setting(cfg_user);
        Cfg::write_setting_file(args, SETTING_FILE, &toml::to_string_pretty(&cfg).unwrap());
        cfg.set_setting();

        Log::set_logger(&cfg.general.log);

        let mut cfg_syntax = CfgSyntax::default();

        if let Ok((theme, err_string)) = ThemeLoader::new(cfg.general.colors.theme.default_color_theme.clone(), &cfg.general.colors.theme.highlight_theme_path, &cfg_syntax.syntax.theme_set.themes).load() {
            Log::debug("err_string", &err_string);

            if !err_string.is_empty() {
                return err_str;
            }
            cfg_syntax.syntax.theme = theme;
            if let Some(c) = cfg_syntax.syntax.theme.settings.background {
                if let Some(theme_bg_enable) = cfg.general.colors.theme.highlight_theme_background_enable {
                    cfg.colors.editor.bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                    cfg.colors.editor.line_number.passive_bg = Color { rgb: Rgb { r: c.r, g: c.g, b: c.b } };
                    cfg.general.colors.theme.theme_bg_enable = theme_bg_enable;
                } else {
                    cfg.general.colors.theme.theme_bg_enable = false;
                }
            }
        }
        let default_color_theme = &cfg.general.colors.theme.default_color_theme;
        Log::debug("theme.default_color_theme", &default_color_theme);

        let default_theme_str = match ThemeSystemColorType::from_str_color_type(&default_color_theme.to_lowercase()) {
            ThemeSystemColorType::Black => include_str!("../../../../setting/theme/default_black.toml"),
            ThemeSystemColorType::White => include_str!("../../../../setting/theme/default_white.toml"),
            ThemeSystemColorType::Blue => include_str!("../../../../setting/theme/default_blue.toml"),
        };
        Log::debug("default_theme_str", &default_theme_str);

        let default_color_theme_file_path = &format!("theme/default_{}.toml", default_color_theme);
        let (user_colors, err_str) = SettingFileLoader::new(FileType::TOML, args, FilePath::get_app_config_dir(), default_color_theme_file_path).load::<CfgUserColors>();
        if !err_str.is_empty() {
            Log::error("err_str", &err_str);
            return err_str;
        }
        cfg.colors = toml::from_str(default_theme_str).unwrap();
        cfg.colors.set_user_setting(user_colors);
        Cfg::write_setting_file(args, default_color_theme_file_path, &toml::to_string_pretty(&cfg.colors).unwrap());
        cfg.convert_color_setting();

        Log::info_s("Setting to apply");
        let cfg_str = toml::to_string_pretty(&cfg).unwrap().replace(NEW_LINE_LF_STR, &NEW_LINE_LF.to_string());
        Log::info_s(&cfg_str);

        Log::info_s("Colors to apply");
        let colors_string = toml::to_string_pretty(&cfg.colors).unwrap().replace(NEW_LINE_LF_STR, &NEW_LINE_LF.to_string());
        Log::info_s(&colors_string);

        let _ = CFG.set(cfg.clone());
        let _ = CFG_EDIT.set(Mutex::new(cfg));
        let _ = CFG_SYNTAX.set(cfg_syntax);

        return err_str;
    }

    pub fn write_setting_file(args: &AppArgs, filenm: &str, write_str: &str) -> String {
        let mut err_str = String::new();
        if args.out_config_flg {
            if FilePath::get_app_config_dir().is_some() {
                let setting_file_fullpath = &FilePath::get_app_config_file_path(filenm).unwrap();
                Log::debug("write_setting_file_path", &setting_file_fullpath);
                if !Path::new(&setting_file_fullpath).exists() {
                    File::create_write_file(setting_file_fullpath, write_str).unwrap_or_else(|why| {
                        Log::error_s(&format!("{} create err {}", setting_file_fullpath.display(), why));
                    });
                }
            } else {
                err_str = "Config dir not found".to_string();
            }
        }
        return err_str;
    }
}
