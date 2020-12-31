use crossterm::event::{Event, Event::*, KeyCode::*, KeyEvent, KeyModifiers};

pub const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const STR_UNDEFINED: &str = "STR_UNDEFINED";
pub const USIZE_UNDEFINED: usize = usize::MAX;
// コピー複数行の最後が空文字の場合の代替文字　WSLで改行を最後に設定出来ない対応
pub const COPY_END: &str = "COPY_END";
pub const NEW_LINE: char = '\n';
pub const NEW_LINE_CR: char = '\r';
pub const NEW_LINE_CRLF: &str = "\r\n";
pub const NEW_LINE_MARK: char = '↲';
// 暫定のEOFの印
pub const EOF: char = '▚';
pub const EOF_MARK: &str = "EOF";
/// Event
pub const RIGHT: Event = Key(KeyEvent { code: Right, modifiers: KeyModifiers::NONE });
pub const PAGE_DOWN: Event = Key(KeyEvent { code: PageDown, modifiers: KeyModifiers::NONE });
// SHIFT
pub const SHIFT_RIGHT: Event = Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code: Right });
// CTRL
pub const CTRL_V: Event = Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: Char('v') });
