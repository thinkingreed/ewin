use crossterm::event::{Event, Event::*, KeyCode::*, KeyEvent, KeyModifiers};
pub const SETTING_FILE: &str = "setting.toml";
pub const STR_UNDEFINED: &str = "STR_UNDEFINED";
pub const USIZE_UNDEFINED: usize = usize::MAX;
// Corresponding alternative character that cannot set a newline at the end in WSL
pub const COPY_END: &str = "COPY_END";
pub const NEW_LINE: char = '\n';
pub const NEW_LINE_CR: char = '\r';
pub const NEW_LINE_CRLF: &str = "\r\n";
// mark to treat as char
pub const NEW_LINE_MARK: char = '↓';
pub const EOF_MARK: char = '▚';
pub const EOF_STR: &str = "EOF";
pub const MULTI_CLICK_MILLISECONDS: i64 = 500;
pub const DELIM_STR: &'static str = r#"!"\#$%&()*+-',./:;<=>?@[]^`{|}~"#;
pub const HALF_SPACE: &'static str = " ";
pub const FULL_SPACE: &'static str = "　";
pub const ENABLE_SYNTAX_HIGHLIGHT_FILE_SIZE: u64 = 1048576;
/// Event
pub const RIGHT: Event = Key(KeyEvent { code: Right, modifiers: KeyModifiers::NONE });
pub const LEFT: Event = Key(KeyEvent { code: Left, modifiers: KeyModifiers::NONE });
pub const DOWN: Event = Key(KeyEvent { code: Down, modifiers: KeyModifiers::NONE });
pub const UP: Event = Key(KeyEvent { code: Up, modifiers: KeyModifiers::NONE });
pub const PAGE_DOWN: Event = Key(KeyEvent { code: PageDown, modifiers: KeyModifiers::NONE });
pub const PAGE_UP: Event = Key(KeyEvent { code: PageUp, modifiers: KeyModifiers::NONE });
pub const DEL: Event = Key(KeyEvent { code: Delete, modifiers: KeyModifiers::NONE });
pub const BS: Event = Key(KeyEvent { code: Backspace, modifiers: KeyModifiers::NONE });
pub const HOME: Event = Key(KeyEvent { code: Home, modifiers: KeyModifiers::NONE });
pub const END: Event = Key(KeyEvent { code: End, modifiers: KeyModifiers::NONE });
pub const ENTER: Event = Key(KeyEvent { code: Enter, modifiers: KeyModifiers::NONE });
pub const SEARCH: Event = Key(KeyEvent { code: F(3), modifiers: KeyModifiers::NONE });
pub const KEY_NULL: Event = Key(KeyEvent { code: Null, modifiers: KeyModifiers::NONE });
pub const HELP: Event = Key(KeyEvent { code: F(1), modifiers: KeyModifiers::NONE });
// SHIFT
pub const SHIFT_RIGHT: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Right });
pub const SHIFT_LEFT: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Left });
pub const SHIFT_DOWN: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Down });
pub const SHIFT_UP: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Up });
pub const SEARCH_DESC: Event = Key(KeyEvent { code: F(4), modifiers: KeyModifiers::SHIFT });
// CTRL
pub const CLOSE: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('w') });
pub const PASTE: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('v') });
pub const CUT: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('x') });
pub const UNDO: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('z') });
pub const REDO: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('y') });
pub const ALL_SELECT: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('a') });
pub const CTRL_HOME: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Home });
// Key
pub const KEY_CLOSE: &'static str = "Ctrl + w";
pub const KEY_COPY: &'static str = "Ctrl + c";
pub const KEY_SAVE: &'static str = "Ctrl + s";
pub const KEY_PASTE: &'static str = "Ctrl + v";
pub const KEY_UNDO: &'static str = "Ctrl + z";
pub const KEY_REDO: &'static str = "Ctrl + y";
pub const KEY_SEARCH: &'static str = "Ctrl + f";
pub const KEY_REPLACE: &'static str = "Ctrl + r";
pub const KEY_CUT: &'static str = "Ctrl + x";
pub const KEY_GREP: &'static str = "Ctrl + g";
// pub const KEY_ALL_SELECT: &'static str = "Ctrl + a";
pub const KEY_ALL_SELECT: &'static str = "Ctrl + a";
pub const KEY_MOVE_ROW: &'static str = "Ctrl + l";
pub const KEY_SELECT: &'static str = "Shift + ↑↓←→ / Mouse";
pub const KEY_RECORD_START: &'static str = "Shift + F1";
pub const KEY_RECORD_STOP: &'static str = "Shift + F2";
pub const KEY_HELP: &'static str = "F1";
pub const KEY_HELP_DETAIL: &'static str = "F1 * 2";
pub const HELP_DETAIL: &'static str = "detail";
