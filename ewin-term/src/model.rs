#[derive(Debug, Clone)]
pub struct Macros {}

#[derive(Debug, Clone)]
pub struct MsgBar {
    pub msg_readonly: String,
    pub msg_keyrecord: String,
    pub msg_keyrecord_org: String,
    pub msg: Msg,
    pub msg_org: Msg,
    pub disp_readonly_row_posi: usize,
    pub disp_keyrecord_row_posi: usize,
    // 0 indexed
    pub disp_row_posi: usize,
    // 0 indexed
    pub disp_readonly_row_num: usize,
    // 0 indexed
    pub disp_keyrecord_row_num: usize,
    pub disp_row_num: usize,
    pub disp_col_num: usize,
}

impl Default for MsgBar {
    fn default() -> Self {
        MsgBar { msg_readonly: String::new(), msg_keyrecord: String::new(), msg_keyrecord_org: String::new(), msg: Msg::default(), msg_org: Msg::default(), disp_readonly_row_posi: 0, disp_keyrecord_row_posi: 0, disp_row_posi: 0, disp_readonly_row_num: 0, disp_keyrecord_row_num: 0, disp_row_num: 0, disp_col_num: 0 }
    }
}

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
