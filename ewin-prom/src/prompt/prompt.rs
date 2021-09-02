use crate::{
    cont::{promptcont::PromptContPosi::*, promptcont::*},
    ewin_core::{_cfg::keys::*, def::*, file::*, log::Log, model::*, util::*},
    grep::*,
    menu::*,
    open_file::*,
    prompt::choice::*,
    save_new_file::*,
};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::{
    fmt,
    io::{stdout, BufWriter, Write},
    path::{self, Path},
    u16,
};

impl Prompt {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, prompt_disp_row_posi: u16, tab_state: &TabState, is_exsist_msg: bool) {
        Log::info_key("Prompt.draw");

        if !tab_state.is_nomal() {
            self.draw_set_posi(tab_state, prompt_disp_row_posi);

            Log::debug("self.cont_1", &self.cont_1);
            Log::debug("self.cont_2", &self.cont_2);
            Log::debug("tab_state", &tab_state);

            Prompt::set_draw_vec(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
            Prompt::set_draw_vec(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);

            if tab_state.is_save_new_file || tab_state.is_move_row {
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            } else if tab_state.is_search {
                self.draw_search(str_vec);
            } else if tab_state.is_replace {
                self.draw_replace(str_vec);
            } else if tab_state.is_open_file {
                self.draw_open_file(str_vec, is_exsist_msg);
            } else if tab_state.is_menu {
                self.draw_menu(str_vec);
            } else if tab_state.grep_state.is_grep {
                self.draw_grep(str_vec);
            } else if tab_state.is_enc_nl {
                self.draw_enc_nl(str_vec);
            }
            let out = stdout();
            let mut out = BufWriter::new(out.lock());
            let _ = out.write(&str_vec.concat().as_bytes());
            out.flush().unwrap();
            str_vec.clear();
        }
    }

    pub fn get_serach_opt(&self) -> String {
        let o1 = &self.cont_1.opt_1;
        let o2 = &self.cont_1.opt_2;
        return format!("{}{}  {}{}", o1.key, o1.get_check_str(), o2.key, o2.get_check_str());
    }

    pub fn set_draw_vec(str_vec: &mut Vec<String>, posi: u16, str: &String) {
        str_vec.push(format!("{}{}{}", MoveTo(0, posi), Clear(CurrentLine), str));
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T, tab_state: &TabState, is_exsist_msg: bool) {
        Log::debug_key("Prompt.draw_only");
        let mut v: Vec<String> = vec![];
        let prom_disp_row_posi = self.disp_row_posi;
        self.draw(&mut v, prom_disp_row_posi, tab_state, is_exsist_msg);
        self.draw_cur(&mut v, tab_state);
        let _ = out.write(&v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur(&self, str_vec: &mut Vec<String>, tab_state: &TabState) {
        let mut x = 0;
        let mut y = 0;

        if tab_state.is_exists_input_field() {
            if self.cont_posi == PromptContPosi::First {
                x = self.cont_1.cur.disp_x;
                y = self.cont_1.buf_row_posi;
            } else if self.cont_posi == PromptContPosi::Second {
                x = self.cont_2.cur.disp_x;
                y = self.cont_2.buf_row_posi;
            } else if self.cont_posi == PromptContPosi::Third {
                x = self.cont_3.cur.disp_x;
                y = self.cont_3.buf_row_posi;
            }
            str_vec.push(MoveTo(x as u16, y as u16).to_string());
        } else if tab_state.is_menu {
            self.draw_cur_menu(str_vec);
        } else if tab_state.is_enc_nl {
            self.draw_cur_enc_nl(str_vec);
        }
    }

    pub fn cursor_down(&mut self, state: &TabState) {
        Log::debug_key("cursor_down");
        if state.is_replace {
            if self.cont_posi == PromptContPosi::First {
                self.cont_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            }
        } else if state.grep_state.is_grep {
            if self.cont_posi == PromptContPosi::First {
                self.cont_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            } else if self.cont_posi == PromptContPosi::Second {
                self.cont_posi = PromptContPosi::Third;
                Prompt::set_cur(&self.cont_2, &mut self.cont_3)
            }
        }
    }

    pub fn cursor_up(&mut self, state: &TabState) {
        if state.is_replace {
            if self.cont_posi == PromptContPosi::Second {
                self.cont_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            }
        } else if state.grep_state.is_grep {
            if self.cont_posi == PromptContPosi::Second {
                self.cont_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            } else if self.cont_posi == PromptContPosi::Third {
                self.cont_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_3, &mut self.cont_2)
            }
        }
    }

    pub fn set_cur(cont_org: &PromptCont, cont: &mut PromptCont) {
        cont.updown_x = cont_org.cur.disp_x;
        let (cur_x, width) = get_until_x(&cont.buf, cont.updown_x);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }

    pub fn ctrl_mouse(&mut self, state: &TabState, y: usize, x: usize, is_left_down: bool) {
        Log::debug_key("PromptCont.ctrl_mouse");

        if y as u16 == self.cont_1.buf_row_posi {
            self.cont_posi = PromptContPosi::First;
            if !state.is_open_file {
                self.cont_1.ctrl_mouse(x, y, is_left_down);
            }
        } else if y as u16 == self.cont_2.buf_row_posi {
            if !state.is_open_file {
                self.cont_posi = PromptContPosi::Second;
                self.cont_2.ctrl_mouse(x, y, is_left_down);
            }
        } else if y as u16 == self.cont_3.buf_row_posi {
            self.cont_posi = PromptContPosi::Third;
            self.cont_3.ctrl_mouse(x, y, is_left_down);
        }
    }

    pub fn shift_move_com(&mut self) {
        match &self.cont_posi {
            First => self.cont_1.shift_move_com(),
            Second => self.cont_2.shift_move_com(),
            Third => self.cont_3.shift_move_com(),

            _ => {}
        }
    }

    pub fn insert_str(&mut self, str: String) {
        match self.cont_posi {
            First => self.cont_1.edit_proc(KeyCmd::InsertStr(str)),
            Second => self.cont_2.edit_proc(KeyCmd::InsertStr(str)),
            Third => self.cont_3.edit_proc(KeyCmd::InsertStr(str)),

            _ => {}
        }
    }
    pub fn copy(&mut self) {
        match self.cont_posi {
            First => self.cont_1.copy(),
            Second => self.cont_2.copy(),
            Third => self.cont_3.copy(),

            _ => {}
        }
    }

    pub fn undo(&mut self) {
        match self.cont_posi {
            First => self.cont_1.undo(),
            Second => self.cont_2.undo(),
            Third => self.cont_3.undo(),

            _ => {}
        }
    }
    pub fn redo(&mut self) {
        match self.cont_posi {
            First => self.cont_1.redo(),
            Second => self.cont_2.redo(),
            Third => self.cont_3.redo(),
            _ => {}
        }
    }

    pub fn operation(&mut self) {
        Log::debug_s("PromptCont.operation");

        let cont = match &self.cont_posi {
            First => &mut self.cont_1,
            Second => &mut self.cont_2,
            Third => &mut self.cont_3,
            Fourth => &mut self.cont_4,
        };

        if self.prom_open_file.vec_y != PromOpenFile::PATH_INPUT_FIELD && cont.keycmd == KeyCmd::CursorLeft || cont.keycmd == KeyCmd::CursorRight {
            return;
        }

        match &cont.keycmd {
            KeyCmd::InsertStr(_) | KeyCmd::Cut | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar => cont.edit_proc(cont.keycmd.clone()),
            KeyCmd::CursorLeft | KeyCmd::CursorRight | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd => cont.cur_move(),
            _ => {}
        }
    }

    pub fn tab(&mut self, is_asc: bool, state: &TabState) {
        if state.is_replace {
            match self.cont_posi {
                PromptContPosi::First => self.cursor_down(state),
                PromptContPosi::Second => self.cursor_up(state),
                _ => {}
            }
        } else if state.grep_state.is_grep {
            match self.cont_posi {
                PromptContPosi::First => {
                    if is_asc {
                        self.cursor_down(state);
                    } else {
                        self.cont_posi = PromptContPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                }
                PromptContPosi::Second => {
                    if is_asc {
                        self.cursor_down(state);
                    } else {
                        self.cursor_up(state);
                    }
                }
                PromptContPosi::Third => {
                    let str = self.cont_3.buf[..self.cont_3.cur.x].iter().collect::<String>();

                    self.cont_3.buf = self.prom_grep.tab_comp.get_tab_candidate(is_asc, str, true).chars().collect();
                    let (cur_x, width) = get_row_width(&self.cont_3.buf[..], 0, false);
                    self.cont_3.cur.x = cur_x;
                    self.cont_3.cur.disp_x = width;
                }
                _ => {}
            }
        } else if state.is_save_new_file {
            let str = self.cont_1.buf[..self.cont_1.cur.x].iter().collect::<String>();
            self.cont_1.buf = self.prom_save_new_file.tab_comp.get_tab_candidate(is_asc, str, false).chars().collect();

            self.cont_1.set_cur_target(self.cont_1.buf.len());
        } else if state.is_open_file {
            let str = self.cont_1.buf[..self.cont_1.cur.x].iter().collect::<String>();

            self.cont_1.buf = self.prom_open_file.tab_comp.get_tab_candidate(is_asc, str, false).chars().collect();

            let (cur_x, width) = get_row_width(&self.cont_1.buf[..], 0, false);
            self.cont_1.cur.x = cur_x;
            self.cont_1.cur.disp_x = width;
        } else if state.is_enc_nl {
            self.move_enc_nl(CurDirection::Right);
        } else if state.is_menu {
            if is_asc {
                match self.cont_posi {
                    PromptContPosi::First => self.cont_posi = PromptContPosi::Second,
                    PromptContPosi::Second => {
                        let (first_y, _) = Choices::get_y_x(&self.cont_3);
                        self.cont_posi = if first_y == USIZE_UNDEFINED { PromptContPosi::First } else { PromptContPosi::Third }
                    }
                    PromptContPosi::Third => self.cont_posi = PromptContPosi::First,
                    _ => {}
                }
            } else {
                match self.cont_posi {
                    PromptContPosi::First => {
                        let (first_y, _) = Choices::get_y_x(&self.cont_3);
                        self.cont_posi = if first_y == USIZE_UNDEFINED { PromptContPosi::Second } else { PromptContPosi::Third }
                    }
                    PromptContPosi::Second => self.cont_posi = PromptContPosi::First,
                    PromptContPosi::Third => self.cont_posi = PromptContPosi::Second,
                    _ => {}
                }
            }
        }
    }
    pub fn set_keys(&mut self, keys: Keys) {
        let keycmd = Keybind::keys_to_keycmd(&keys, KeyWhen::PromptFocus);
        self.keycmd = keycmd.clone();
        match self.cont_posi {
            PromptContPosi::First => self.cont_1.keycmd = keycmd,
            PromptContPosi::Second => self.cont_2.keycmd = keycmd,
            PromptContPosi::Third => self.cont_3.keycmd = keycmd,
            PromptContPosi::Fourth => self.cont_4.keycmd = keycmd,
        }
    }

    pub fn new() -> Self {
        Prompt { ..Prompt::default() }
    }
    pub fn clear(&mut self) {
        Log::debug_key("Prompt.clear");
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.cont_1 = PromptCont::default();
        self.cont_2 = PromptCont::default();
        self.cont_3 = PromptCont::default();
        self.cont_4 = PromptCont::default();
        self.cont_posi = PromptContPosi::First;
    }
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub keycmd: KeyCmd,
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
            keycmd: KeyCmd::Null,
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
impl TabComp {
    pub fn get_tab_candidate(&mut self, is_asc: bool, target_path: String, is_dir_only: bool) -> String {
        if self.files.len() == 0 {
            self.files = get_tab_comp_files(target_path.clone(), is_dir_only, true);
        }
        Log::debug_s("　　One candidate");

        let mut rtn_string = target_path;

        for file in &self.files {
            // One candidate
            if self.files.len() == 1 {
                Log::debug_s("　　One candidate");

                if !is_dir_only {
                    let path = Path::new(&file.name);
                    //  let path = Path::new(&os_str);
                    rtn_string = if path.metadata().unwrap().is_file() { file.name.to_string() } else { format!("{}{}", file.name.to_string(), path::MAIN_SEPARATOR) };
                } else {
                    rtn_string = format!("{}{}", file.name.to_string(), path::MAIN_SEPARATOR);
                }
                self.clear_tab_comp();
                break;

            // Multiple candidates
            } else if self.files.len() > 1 {
                Log::debug_s("Multi candidates");
                Log::debug("self.tab_comp.index", &self.index);
                if is_asc && self.index >= self.files.len() - 1 || self.index == USIZE_UNDEFINED {
                    self.index = 0;
                } else if !is_asc && self.index == 0 {
                    self.index = self.files.len() - 1;
                } else {
                    self.index = if is_asc { self.index + 1 } else { self.index - 1 };
                }
                rtn_string = self.files[self.index].name.clone();
                break;
            }
        }

        return rtn_string;
    }
    pub fn clear_tab_comp(&mut self) {
        Log::debug_s("clear_tab_comp ");
        self.index = USIZE_UNDEFINED;
        self.files.clear();
    }
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
