use chrono::{DateTime, Local};
use ewin_cfg::{global::*, model::general::default::*};
use ewin_const::def::*;
use number_prefix::NumberPrefix;
use serde_json::Value;
use std::{self, collections::HashMap, time::*, *};

pub fn is_enable_syntax_highlight(ext: &str) -> bool {
    !(ext.is_empty() || Cfg::get().colors.theme.disable_highlight_ext.contains(&ext.to_string()))
}

pub fn ordinal_suffix(number: usize) -> &'static str {
    match (number % 10, number % 100) {
        (_, 11..=13) => "th",
        (1, _) => "st",
        (2, _) => "nd",
        (3, _) => "rd",
        _ => "th",
    }
}

pub fn change_regex(string: String) -> String {
    let cfg_search = &&Cfg::get().general.editor.search;

    if cfg_search.regex {
        let string = string.replace("\\n", &'\n'.to_string());
        let string = string.replace("\\t", &'\t'.to_string());
        let string = string.replace("\\r", &'\r'.to_string());
        let string = string.replace('\\', r"\");
        let string = string.replace("\\'", "\'");
        let string = string.replace("\\\"", "\"");
        return string;
    }
    string
}
pub fn get_tab_str() -> String {
    Cfg::get().general.editor.tab.tab.clone()
}

/// Get version of app as a whole
pub fn get_app_version() -> String {
    let cfg_str = include_str!("../../../../Cargo.toml");
    let map: HashMap<String, Value> = toml::from_str(cfg_str).unwrap();
    let mut s = map["package"]["version"].to_string();
    s.retain(|c| c != '"');
    return s;
}

pub fn get_unixtime_str() -> String {
    let unixtime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    return format!("{}.{:03}", unixtime.as_secs(), unixtime.subsec_millis());
}

pub fn to_unixtime_str(sys_time: SystemTime) -> String {
    let unixtime = sys_time.duration_since(UNIX_EPOCH).unwrap();
    return format!("{}.{:03}", unixtime.as_secs(), unixtime.subsec_millis());
}

pub fn get_cfg_lang_name(name_str: &str) -> &str {
    if let Some(name) = LANG_MAP.get(name_str) {
        return name;
    } else {
        return name_str;
    }
}

pub fn time_to_str(time: SystemTime) -> String {
    let time: DateTime<Local> = time.into();
    return time.format("%Y/%m/%d %H:%M:%S").to_string();
}

pub fn fmt_bytes(amount: u64) -> String {
    let amount = amount as f64;
    let result = match NumberPrefix::decimal(amount) {
        NumberPrefix::Standalone(bytes) => {
            format!("{} {}", bytes, BYTE_KEY)
        }
        NumberPrefix::Prefixed(prefix, n) => {
            format!("{:.2} {}B", n, prefix.to_string().to_ascii_uppercase())
        }
    };
    return result;
}

#[cfg(test)]
mod tests {
    use ewin_cfg::model::{general::default::*, modal::AppArgs};
    use ewin_const::models::env::Env;

    use crate::{global::*, os::*};

    use super::*;

    #[test]
    fn test_get_env_platform() {
        match *ENV {
            Env::WSL => assert_eq!(get_env_platform(), Env::WSL),
            Env::Linux => assert_eq!(get_env_platform(), Env::Linux),
            Env::Windows => assert_eq!(get_env_platform(), Env::Windows),
        };
    }

    #[test]
    fn test_is_enable_syntax_highlight() {
        Cfg::init(&AppArgs { ..AppArgs::default() });

        assert!(!is_enable_syntax_highlight("txt"));
        assert!(is_enable_syntax_highlight("rs"));
        assert!(!is_enable_syntax_highlight(""));
    }
}
