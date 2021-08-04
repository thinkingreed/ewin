pub const SETTING_FILE: &str = "setting.toml";
pub const KEYBINDING_FILE: &str = "keybind.json5";
pub const MACROS_DIR: &str = "macros";

pub const STR_UNDEFINED: &str = "STR_UNDEFINED";
pub const USIZE_UNDEFINED: usize = usize::MAX;
// Corresponding alternative character that cannot set a newline at the end in WSL
pub const COPY_END: &'static str = "COPY_END";
pub const NEW_LINE_LF: char = '\n';
pub const NEW_LINE_LF_STR: &'static str = "LF";
pub const NEW_LINE_CR: char = '\r';
pub const NEW_LINE_CRLF: &str = "\r\n";
pub const NEW_LINE_CRLF_STR: &'static str = "CRLF";
pub const TAB_CHAR: char = '\t';
// mark to treat as char
pub const NEW_LINE_LF_MARK: char = '↓';
pub const NEW_LINE_CRLF_MARK: char = '↵';
// Meaningless mark
pub const EOF_MARK: char = '▚';
pub const EOF_STR: &str = "EOF";
pub const TAB_MARK: char = '^';
// "…" is not adopted because the width is handled differently depending on the terminal.
pub const CONTINUE_STR: &'static str = "..";
pub const PARENT_FOLDER: &'static str = "..";

#[cfg(target_os = "windows")]
pub const MULTI_CLICK_MILLISECONDS: i64 = 1500;
#[cfg(target_os = "linux")]
pub const MULTI_CLICK_MILLISECONDS: i64 = 500;
pub const DELIM_STR: &'static str = r#"!"\#$%&()*+-',./:;<=>?@[]^`{|}~"#;
pub const HALF_SPACE: &'static str = " ";
pub const FULL_SPACE: &'static str = "　";
pub const ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE: u64 = 1048576;

// Key
pub const KEY_SELECT_KEY: &'static str = "+↑↓←→/Mouse";
pub const HELP_DETAIL: &'static str = "Help detail";
