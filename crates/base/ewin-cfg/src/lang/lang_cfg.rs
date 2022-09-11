use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{global::*, model::general::default::*};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lang {
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
    pub save_forced: String,
    pub undo: String,
    pub redo: String,
    pub cut: String,
    pub changed: String,
    pub detail: String,
    pub grep: String,
    pub open_file: String,
    pub reopen: String,
    pub movement: String,
    pub file: String,
    pub edit: String,
    pub unable_to_edit: String,
    pub edit_discard: String,
    pub convert: String,
    pub filenm: String,
    pub file_list: String,
    pub presence_or_absence: String,
    pub method_of_apply: String,
    pub file_reload: String,
    pub keep_and_apply_string: String,
    pub range_select: String,
    pub mouse_switch: String,
    pub mouse_disable: String,
    pub all_select: String,
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
    pub replace_str: String,
    pub save_confirm_to_close: String,
    pub file_has_been_modified_by_other_app: String,
    pub open_modified_time: String,
    pub last_modified_time: String,
    pub terminal_size_small: String,
    pub set_new_filenm: String,
    pub set_open_filenm: String,
    pub set_exec_mocro_filenm: String,
    pub set_search: String,
    pub set_replace: String,
    pub set_grep: String,
    pub set_move_row: String,
    pub set_enc_nl: String,
    pub selectable_only_for_utf8: String,
    pub move_to_specified_row: String,
    pub complement: String,
    pub open_target_file: String,
    pub key_record_start_stop: String,
    pub key_record_exec: String,
    pub key_recording: String,
    pub help: String,
    pub help_init_display_switch: String,
    pub candidate_change: String,
    pub encoding: String,
    pub new_line_code: String,
    pub none: String,
    pub mouse_down_left: String,
    pub unsettled: String,

    // menu
    pub menu: String,
    pub contents: String,
    // file
    pub open_new_file: String,
    pub save_as: String,
    pub macros: String,

    pub all_save_finish: String,

    pub to_uppercase: String,
    pub to_lowercase: String,
    pub to_half_width: String,
    pub to_full_width: String,
    pub to_space: String,
    pub to_tab: String,
    pub html: String,
    pub xml: String,
    pub json: String,
    pub json5: String,
    pub toml: String,
    pub tool: String,

    pub box_select: String,
    pub box_insert: String,
    pub box_select_mode: String,
    // display
    pub display: String,
    pub row_no: String,
    pub scale: String,
    pub appearance: String,
    pub sidebar: String,
    // Window
    pub window: String,
    pub left_and_right_split: String,
    pub top_and_bottom_split: String,
    // other
    pub other: String,
    pub about_app: String,
    // Dialog
    // file info
    pub file_property: String,
    pub place: String,
    pub size: String,
    pub create_time: String,
    pub mod_time: String,

    /// Long msg
    pub not_entered_filenm: String,
    pub not_set_search_str: String,
    pub not_entered_search_file: String,
    pub not_entered_search_folder: String,
    pub not_entered_replace_str: String,
    pub not_entered_row_number_to_move: String,
    pub cannot_find_search_char: String,
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
    pub file_opening_problem: String,
    pub file_saving_problem: String,
    pub file_not_found: String,
    pub file_loading_failed: String,
    pub file_already_exists: String,
    pub log_file_create_failed: String,
    pub check_log_file: String,
    pub close_other_than_this_tab: String,
    pub no_tab_can_be_switched: String,
    pub not_edited_will_reloaded_auto: String,
    pub no_further_monitoring: String,
    pub extension: String,
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
    // input comple
    pub no_input_comple_candidates: String,
    // other
    pub unsupported_operation: String,
    pub increase_height_width_terminal: String,
}

impl Lang {
    pub fn get_lang_map() -> HashMap<String, String> {
        return serde_json::from_str(&serde_json::to_string(&Lang::get()).unwrap()).unwrap();
    }
}
impl Lang {
    pub fn get() -> &'static Lang {
        LANG.get().unwrap()
    }

    pub fn read_lang_cfg() -> Lang {
        let lang = &Cfg::get().general.lang;

        let lang_str = if lang.starts_with("ja_JP") { include_str!("ja_JP.toml") } else { include_str!("en_US.toml") };
        let lang_cfg: Lang = toml::from_str(lang_str).unwrap();

        lang_cfg
    }
}
