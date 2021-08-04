use crate::global::LANG;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(target_os = "linux")]
use std::env;
#[cfg(target_os = "windows")]
use std::process::Command;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LangCfg {
    pub row: String,
    pub col: String,
    pub yes: String,
    pub no: String,
    pub with: String,
    pub without: String,
    pub cancel: String,
    pub close: String,
    pub fixed: String,
    pub end: String,
    pub copy: String,
    pub paste: String,
    pub save: String,
    pub undo: String,
    pub redo: String,
    pub cut: String,
    pub changed: String,
    pub detail: String,
    pub grep: String,
    pub open: String,
    pub movement: String,
    pub file: String,
    pub edit: String,
    pub convert: String,
    pub filenm: String,
    pub file_list: String,
    pub presence_or_absence: String,
    pub method_of_applying: String,
    pub file_reload: String,
    pub keep_and_apply_string: String,
    pub range_select: String,
    pub mouse_switch: String,
    // pub all_select: String,
    pub move_row: String,
    pub format: String,
    pub search: String,
    pub search_bottom: String,
    pub search_top: String,
    pub search_str: String,
    pub search_file: String,
    pub search_folder: String,
    pub searching: String,
    pub case_sens: String,
    pub regex: String,
    pub new_file: String,
    pub replace: String,
    pub all_replace: String,
    pub select: String,
    pub move_setting_location: String,
    pub replace_char: String,
    pub save_confirmation_to_close: String,
    pub terminal_size_small: String,
    pub set_new_filenm: String,
    pub set_open_filenm: String,
    pub set_search: String,
    pub set_replace: String,
    pub set_grep: String,
    pub set_move_row: String,
    pub set_enc_nl: String,
    pub selectable_only_for_utf8: String,
    pub move_to_specified_row: String,
    pub unable_to_edit: String,
    pub complement: String,
    pub open_target_file: String,
    pub key_record_start_stop: String,
    pub key_record_exec: String,
    pub key_recording: String,
    pub help: String,
    pub candidate_change: String,
    pub encoding: String,
    pub new_line_code: String,
    // menu
    pub menu: String,
    pub contents: String,
    pub create_new: String,
    pub save_as: String,
    pub macros: String,

    pub encode: String,
    pub end_of_all_save: String,

    pub to_uppercase: String,
    pub to_lowercase: String,
    pub to_half_width: String,
    pub to_full_width: String,
    pub to_space: String,
    pub to_tab: String,
    pub html: String,
    pub xml: String,
    pub json: String,

    pub box_select: String,
    pub box_insert: String,
    pub box_select_mode: String,

    /// Long msg
    pub not_entered_filenm: String,
    pub not_entered_search_str: String,
    pub not_entered_search_file: String,
    pub not_entered_search_folder: String,
    pub not_entered_replace_str: String,
    pub not_entered_row_number_to_move: String,
    pub cannot_find_char_search_for: String,
    pub long_time_to_search: String,
    pub show_search_result: String,
    pub show_search_no_result: String,
    pub no_undo_operation: String,
    pub no_redo_operation: String,
    pub number_within_current_number_of_rows: String,
    pub cannot_convert_encoding: String,
    pub select_menu: String,
    pub processing_canceled: String,
    pub parsing_failed: String,
    // Keybind
    pub specification_err_key: String,
    pub specification_err_keycmd: String,
    pub specification_err_keywhen: String,
    // File
    pub no_read_permission: String,
    pub no_write_permission: String,
    pub file_opening_problem: String,
    pub file_saving_problem: String,
    pub file_not_found: String,
    pub file_loading_failed: String,
    pub file_already_exists: String,
    pub log_file_create_failed: String,
    // Not sel range
    pub no_sel_range: String,
    // Paste
    pub no_value_in_clipboard: String,
    pub cannot_paste_multi_rows: String,
    // key record
    pub no_key_record_exec: String,
    // editor info
    pub simple_help_desc: String,
    pub detailed_help_desc: String,
    // macro
    pub script_run_error: String,
    pub script_compile_error: String,
    pub specify_file_and_exec_macro: String,
    // other
    pub unsupported_operation: String,
    pub increase_height_width_terminal: String,
}

impl LangCfg {
    pub fn get_lang_map() -> HashMap<String, String> {
        let lang_map: HashMap<String, String> = serde_json::from_str(&serde_json::to_string(&*LANG).unwrap()).unwrap();

        return lang_map;
    }
}
#[cfg(target_os = "linux")]
impl LangCfg {
    pub fn read_lang_cfg() -> LangCfg {
        let env_lang = env::var("LANG").unwrap_or("en_US".to_string());

        // TODO File read dynamic generation
        let lang_str = if env_lang.starts_with("ja_JP") { include_str!("ja_JP.toml") } else { include_str!("en_US.toml") };
        let lang_cfg: LangCfg = toml::from_str(&lang_str).unwrap();

        return lang_cfg;
    }
}

#[cfg(target_os = "windows")]
impl LangCfg {
    pub fn read_lang_cfg() -> LangCfg {
        let env_lang = match Command::new("powershell.exe").args(&["(Get-WinUserLanguageList)[0].LanguageTag"]).output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
            Err(_) => "en_US".to_string(),
        };
        let lang_str = if env_lang.starts_with("ja_JP") { include_str!("ja_JP.toml") } else { include_str!("en_US.toml") };
        let lang_cfg: LangCfg = toml::from_str(&lang_str).unwrap();

        return lang_cfg;
    }
}
