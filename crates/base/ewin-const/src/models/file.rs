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
