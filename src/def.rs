use crossterm::event::{Event, Event::*, KeyCode::*, KeyEvent, KeyModifiers};
pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const STR_UNDEFINED: &str = "STR_UNDEFINED";
pub const USIZE_UNDEFINED: usize = usize::MAX;
// コピー複数行の最後が空文字の場合の代替文字　WSLで改行を最後に設定出来ない対応
pub const COPY_END: &str = "COPY_END";
pub const NEW_LINE: char = '\n';
pub const NEW_LINE_CR: char = '\r';
pub const NEW_LINE_CRLF: &str = "\r\n";
// mark to treat as char
pub const NEW_LINE_MARK: char = '↲';
pub const EOF_MARK: char = '▚';
pub const EOF_STR: &str = "EOF";
pub const MULTI_CLICK_MILLISECONDS: i64 = 500;
pub const DELIM_STR: &'static str = r#"!"\#$%&()*+-',./:;<=>?@[]^`{|}~"#;
pub const HALF_SPACE: &'static str = " ";
pub const FULL_SPACE: &'static str = "　";
/// Event
pub const RIGHT: Event = Key(KeyEvent { code: Right, modifiers: KeyModifiers::NONE });
pub const LEFT: Event = Key(KeyEvent { code: Left, modifiers: KeyModifiers::NONE });
pub const DOWN: Event = Key(KeyEvent { code: Down, modifiers: KeyModifiers::NONE });
pub const UP: Event = Key(KeyEvent { code: Up, modifiers: KeyModifiers::NONE });
pub const PAGE_DOWN: Event = Key(KeyEvent { code: PageDown, modifiers: KeyModifiers::NONE });
pub const DEL: Event = Key(KeyEvent { code: Delete, modifiers: KeyModifiers::NONE });
pub const BS: Event = Key(KeyEvent { code: Backspace, modifiers: KeyModifiers::NONE });
// pub const INSERT_CHAR: Event = Key(KeyEvent { code: Char(), modifiers: KeyModifiers::NONE });
pub const ENTER: Event = Key(KeyEvent { code: Enter, modifiers: KeyModifiers::NONE });
// SHIFT
pub const SHIFT_RIGHT: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Right });
pub const SHIFT_LEFT: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Left });
pub const SHIFT_DOWN: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Down });
pub const SHIFT_UP: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Up });
// CTRL
pub const PASTE: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('v') });
pub const CUT: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('x') });
pub const UNDO: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('z') });
pub const REDO: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('y') });
pub const ALL_SELECT: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('a') });
// MOUSE
