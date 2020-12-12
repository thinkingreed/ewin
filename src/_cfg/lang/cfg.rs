use crate::_cfg::lang::lang::LANG_CONFIG;
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct LangCfg {
    pub row: String,
    pub col: String,
    pub yes: String,
    pub no: String,
    pub cancel: String,
    pub close: String,
    pub fixed: String,
    pub search: String,
    pub search_bottom: String,
    pub search_top: String,
    pub search_str: String,
    pub search_file: String,
    pub search_folder: String,
    pub searching: String,
    pub new_file: String,
    pub all_replace: String,
    pub move_input_field: String,
    pub replace_char: String,
    pub save_confirmation_to_close: String,
    pub terminal_size_small: String,
    pub set_new_filenm: String,
    pub set_search: String,
    pub set_replace: String,
    pub set_grep: String,
    pub unable_to_edit: String,
    pub open_target_file_in_another_terminal: String,
    pub key_recording: String,
    pub complement: String,
    /// Long msg
    pub not_entered_filenm: String,
    pub not_entered_search_str: String,
    pub not_entered_search_file: String,
    pub not_entered_search_folder: String,
    pub not_entered_replace_str: String,
    pub cannot_find_char_search_for: String,
    pub long_time_to_search: String,
    pub show_search_result: String,
    pub show_search_no_result: String,
    pub no_undo_operation: String,
    pub no_operation_re_exec: String,
    // File open
    pub no_read_permission: String,
    pub file_opening_problem: String,
    pub file_not_found: String,
    // Paste
    pub cannot_paste_multi_lines: String,
    pub unsupported_operation: String,
}
#[derive(Debug, Deserialize)]
pub struct LangMulti {
    en: LangCfg,
    ja: LangCfg,
}

impl LangCfg {
    pub fn default() -> Self {
        LangCfg {
            row: String::new(),
            col: String::new(),
            yes: String::new(),
            no: String::new(),
            cancel: String::new(),
            close: String::new(),
            fixed: String::new(),
            search: String::new(),
            search_bottom: String::new(),
            search_top: String::new(),
            search_str: String::new(),
            search_file: String::new(),
            search_folder: String::new(),
            searching: String::new(),
            all_replace: String::new(),
            move_input_field: String::new(),
            new_file: String::new(),
            replace_char: String::new(),
            unable_to_edit: String::new(),
            open_target_file_in_another_terminal: String::new(),
            key_recording: String::new(),
            complement: String::new(),
            // Long msg
            save_confirmation_to_close: String::new(),
            terminal_size_small: String::new(),
            set_new_filenm: String::new(),
            set_search: String::new(),
            set_replace: String::new(),
            set_grep: String::new(),
            not_entered_filenm: String::new(),
            not_entered_search_str: String::new(),
            not_entered_search_file: String::new(),
            not_entered_search_folder: String::new(),
            not_entered_replace_str: String::new(),
            cannot_find_char_search_for: String::new(),
            long_time_to_search: String::new(),
            show_search_result: String::new(),
            show_search_no_result: String::new(),
            no_undo_operation: String::new(),
            no_operation_re_exec: String::new(),
            // File open
            no_read_permission: String::new(),
            file_opening_problem: String::new(),
            file_not_found: String::new(),
            // Paste
            cannot_paste_multi_lines: String::new(),
            unsupported_operation: String::new(),
        }
    }
    pub fn read_lang_cfg() -> LangCfg {
        let lang_multi: LangMulti = toml::from_str(&LANG_CONFIG.to_string()).unwrap();

        let lang = env::var("LANG").unwrap_or("en_US".to_string());
        if lang.starts_with("ja_JP") {
            return lang_multi.ja;
        } else {
            return lang_multi.en;
        }
    }
}
