#[derive(Debug, Clone)]
pub struct Macros {}

#[derive(Debug, Clone, PartialEq)]
pub struct Msg {
    pub str: String,
    pub msg_type: MsgType,
}

impl Default for Msg {
    fn default() -> Self {
        Msg { str: String::new(), msg_type: MsgType::Info }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsgType {
    Info,
    Error,
}

/// Event action
#[derive(Debug, Clone)]
pub struct EvtAct {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOpenType {
    Nomal,
    First,
    Reopen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaveType {
    Normal,
    Forced,
    NewName,
}
