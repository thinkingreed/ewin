use crate::{grep::*, menu::*, open_file::*, prom::choice::*, save_new_file::*};
use ewin_com::{_cfg::key::keycmd::*, def::*, file::*, model::*};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Prompt {
    pub keycmd: KeyCmd,
    pub p_cmd: P_Cmd,
    pub disp_row_num: usize,
    // 0 index
    pub disp_row_posi: u16,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub cont_4: PromptCont,
    pub cont_posi: PromptContPosi,
    pub prom_open_file: PromOpenFile,
    pub prom_save_new_file: PromSaveNewFile,
    pub prom_menu: PromMenu,
    pub prom_grep: PromGrep,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            keycmd: KeyCmd::Prom(P_Cmd::Null),
            p_cmd: P_Cmd::Null,
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            //  is_grep_result: false,
            //  is_grep_result_cancel: false,
            cont_1: PromptCont::default(),
            cont_2: PromptCont::default(),
            cont_3: PromptCont::default(),
            cont_4: PromptCont::default(),
            cont_posi: PromptContPosi::First,
            prom_open_file: PromOpenFile::default(),
            prom_save_new_file: PromSaveNewFile::default(),
            prom_menu: PromMenu::default(),
            prom_grep: PromGrep::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabComp {
    // List of complementary candidates
    pub files: Vec<File>,
    // List of complementary candidates index
    pub index: usize,
}
impl Default for TabComp {
    fn default() -> Self {
        TabComp { index: USIZE_UNDEFINED, files: vec![] }
    }
}
impl fmt::Display for TabComp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TabComp index:{}, files:{:?},", self.index, self.files,)
    }
}

#[derive(Debug, Clone)]
pub struct PromptCont {
    pub keycmd: KeyCmd,
    pub open_file_type: OpenFileType,
    pub p_cmd: P_Cmd,
    pub disp_row_posi: u16,
    pub buf_row_len: u16,
    pub posi: PromptContPosi,
    pub guide_row_posi: u16,
    pub key_desc_row_posi: u16,
    pub opt_row_posi: u16,
    pub buf_desc_row_posi: u16,
    pub buf_row_posi: u16,
    pub cur: Cur,
    pub sel: SelRange,
    pub guide_vec: Vec<String>,
    pub opt_1: PromptContOpt,
    pub opt_2: PromptContOpt,
    // pub key_desc: String,
    pub key_desc_vec: Vec<String>,
    pub buf_desc_vec: Vec<String>,
    // For 1-line input
    pub buf: Vec<char>,
    pub updown_x: usize,
    pub history: History,
    // For list display
    pub file_list_vec: Vec<File>,
    // <((Grandparents choices posi y, Grandparents choices posi x)(Parent choices posi y, Parent choices posi x)), Self Choices>
    pub choices_map: HashMap<((usize, usize), (usize, usize)), Choices>,
}

impl Default for PromptCont {
    fn default() -> Self {
        PromptCont {
            keycmd: KeyCmd::Prom(P_Cmd::Null),
            open_file_type: OpenFileType::Normal,
            p_cmd: P_Cmd::Null,
            disp_row_posi: 0,
            buf_row_len: 0,
            posi: PromptContPosi::First,
            guide_row_posi: 0,
            key_desc_row_posi: 0,
            opt_row_posi: 0,
            buf_desc_row_posi: 0,
            buf_row_posi: 0,
            guide_vec: vec![],
            // key_desc: String::new(),
            key_desc_vec: vec![],
            opt_1: PromptContOpt::default(),
            opt_2: PromptContOpt::default(),
            buf_desc_vec: vec![],
            buf: vec![],
            cur: Cur::default(),
            updown_x: 0,
            history: History::default(),
            sel: SelRange::default(),
            file_list_vec: vec![],
            choices_map: HashMap::new(),
        }
    }
}
#[derive(Debug, Default, Clone)]
pub struct PromptContOpt {
    pub key: String,
    pub is_check: bool,
    pub mouse_area: (u16, u16),
}

impl PromptContOpt {
    pub fn get_check_str(&self) -> String {
        let str = if self.is_check { "[*]" } else { "[ ]" };
        str.to_string()
    }
    pub fn toggle_check(&mut self) {
        match self.is_check {
            true => self.is_check = false,
            false => self.is_check = true,
        }
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Debug, Clone)]
pub enum PromptContPosi {
    First,
    Second,
    Third,
    Fourth,
}
