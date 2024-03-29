pub const APP_NAME: &str = "ewin";

pub const TERM_MINIMUM_WIDTH: usize = 40;
pub const TERM_MINIMUM_HEIGHT: usize = 12;

pub const MENUBAR_HEIGHT: usize = 1;
pub const FILEBAR_HEIGHT: usize = 1;
pub const SCALE_HEIGHT: usize = 1;
pub const MSGBAR_HEIGHT: usize = 1;
pub const STATUSBAR_HEIGHT: usize = 1;

pub const SIDEBAR_SPLIT_LINE_WIDTH: usize = 1;

// Winodw split line width
pub const WINDOW_SPLIT_LINE_WIDTH: usize = 1;

pub const SETTING_FILE: &str = "setting.toml";
pub const KEYBINDING_FILE: &str = "keybind.json5";
pub const MACROS_DIR: &str = "macros";
pub const CLIPBOARD_FILE: &str = "clipboard.txt";

pub const USIZE_UNDEFINED: usize = usize::MAX;
// Corresponding alternative character that cannot set a newline at the end in WSL
pub const COPY_END: &str = "COPY_END";
pub const NEW_LINE_LF: char = '\n';
pub const NEW_LINE_LF_STR: &str = "LF";
pub const NEW_LINE_CR: char = '\r';
pub const NEW_LINE_CRLF: &str = "\r\n";
pub const NEW_LINE_CRLF_STR: &str = "CRLF";
pub const TAB_CHAR: char = '\t';
// mark to treat as char
pub const NEW_LINE_LF_MARK: char = '↓';
pub const NEW_LINE_CRLF_MARK: char = '↵';
pub const EOF_STR: &str = "EOF";
pub const TAB_MARK: char = '^';
// "…" is not adopted because the width is handled differently depending on the terminal.
pub const CONTINUE_STR: &str = "..";
pub const PARENT_FOLDER: &str = "..";

#[cfg(target_os = "windows")]
pub const MULTI_CLICK_MILLISECONDS: i64 = 1000;
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const MULTI_CLICK_MILLISECONDS: i64 = 500;
// pub const DELIM_STR: &str = r#" 　!"\#$%&()*+-',./:;<=>?@[]^`{|}~"#;
pub const HALF_SPACE: &str = " ";
pub const FULL_SPACE: char = '　';
pub const ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE: u64 = 1048576;

// Key
pub const KEY_SELECT_KEY: &str = "+↑↓←→/Mouse";
pub const HELP_DETAIL: &str = "Help detail";

pub const CHAR_Y: &str = "Y";
pub const CHAR_N: &str = "N";
pub const CHAR_R: &str = "R";
pub const CHAR_L: &str = "L";
pub const STR_ESC: &str = "Esc";

pub const BYTE_KEY: &str = "bytes";
