use crate::def::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOpenType {
    Nomal,
    First,
    Reopen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SaveFileType {
    Normal,
    Forced,
    NewFile,
    Confirm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CloseFileType {
    Normal,
    Forced,
}

pub struct NL {}
impl NL {
    pub fn get_nl(nl_str: &str) -> String {
        if nl_str == NEW_LINE_CRLF_STR {
            NEW_LINE_CRLF.to_string()
        } else {
            NEW_LINE_LF.to_string()
        }
    }
}
