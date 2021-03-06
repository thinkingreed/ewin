use super::lang::LANG_CONFIG;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LangCfg {
    pub row: String,
    pub col: String,
    pub yes: String,
    pub no: String,
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
    pub grep: String,
    pub range_select: String,
    pub all_select: String,
    pub move_row: String,
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
    pub move_input_field: String,
    pub replace_char: String,
    pub save_confirmation_to_close: String,
    pub terminal_size_small: String,
    pub set_new_filenm: String,
    pub set_search: String,
    pub set_replace: String,
    pub set_grep: String,
    pub set_move_row: String,
    pub move_to_specified_row: String,
    pub unable_to_edit: String,
    pub complement: String,
    pub open_target_file_in_another_terminal: String,
    pub key_record_start: String,
    pub key_record_stop: String,
    pub key_recording: String,
    pub help: String,
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
    pub no_operation_re_exec: String,
    pub number_within_current_number_of_rows: String,
    // File
    pub no_read_permission: String,
    pub no_write_permission: String,
    pub file_opening_problem: String,
    pub file_not_found: String,
    pub file_loading_failed: String,
    pub file_parsing_failed: String,
    // Save
    pub file_already_exists: String,
    // Not sel range
    pub no_sel_range: String,
    // Paste
    pub no_value_in_clipboard: String,
    pub cannot_paste_multi_rows: String,
    // key record
    pub no_key_record_exec: String,
    // other
    pub unsupported_operation: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LangMulti {
    en: LangCfg,
    ja: LangCfg,
}

impl LangCfg {
    pub fn read_lang_cfg() -> LangCfg {
        let lang_multi: LangMulti = serde_yaml::from_str(&LANG_CONFIG.to_string()).unwrap();
        let lang = env::var("LANG").unwrap_or("en_US".to_string());

        if lang.starts_with("ja_JP") {
            return lang_multi.ja;
        } else {
            return lang_multi.en;
        }
    }
}
