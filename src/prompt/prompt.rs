use super::{grep::PromGrep, open_file::PromOpenFile, search::PromSearch};
use crate::{def::*, log::*, model::*, prompt::promptcont::promptcont::PromptContPosi::*, prompt::promptcont::promptcont::*, tab::TabState, tab::*, util::*};
use crossterm::{cursor::*, event::*, terminal::ClearType::*, terminal::*};
use std::{
    fmt,
    io::{stdout, BufWriter, Write},
    path::{self, Path},
};

impl Prompt {
    pub const CHOICE_ITEM_MARGIN: usize = 2;

    pub fn draw(&self, str_vec: &mut Vec<String>, tab_state: &TabState) {
        Log::info_key("Prompt.draw");

        if self.cont_1.guide.len() > 0 {
            Prompt::set_draw_vec(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
            Prompt::set_draw_vec(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);

            if tab_state.is_save_new_file || tab_state.is_move_line {
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            } else if tab_state.is_search {
                self.draw_search(str_vec);
            } else if tab_state.is_replace || tab_state.grep_info.is_grep {
                Prompt::set_draw_vec(str_vec, self.cont_1.opt_row_posi, &self.get_serach_opt());
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc);
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi, &self.cont_2.get_draw_buf_str());

                if tab_state.grep_info.is_grep {
                    Prompt::set_draw_vec(str_vec, self.cont_3.buf_desc_row_posi, &self.cont_3.buf_desc);
                    Prompt::set_draw_vec(str_vec, self.cont_3.buf_row_posi, &self.cont_3.get_draw_buf_str());
                }
            } else if tab_state.is_open_file {
                self.draw_open_file(str_vec);
            } else if tab_state.is_enc_nl {
                self.draw_enc_nl(str_vec);
            } else if tab_state.is_menu {
                self.draw_menu(str_vec);
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

    pub fn draw_only<T: Write>(out: &mut T, tab: &Tab) {
        Log::debug_key("Prompt.draw_only");
        let mut v: Vec<String> = vec![];
        tab.prom.draw(&mut v, &tab.state);
        tab.prom.draw_cur(&mut v, tab);
        let _ = out.write(&v.concat().as_bytes());
        out.flush().unwrap();
    }

    pub fn draw_cur(&self, str_vec: &mut Vec<String>, tab: &Tab) {
        let mut x = 0;
        let mut y = 0;

        if tab.state.is_exists_input_field() {
            if self.prom_cont_posi == PromptContPosi::First {
                x = self.cont_1.cur.disp_x;
                y = self.cont_1.buf_row_posi;
            } else if self.prom_cont_posi == PromptContPosi::Second {
                x = self.cont_2.cur.disp_x;
                y = self.cont_2.buf_row_posi;
            } else if self.prom_cont_posi == PromptContPosi::Third {
                x = self.cont_3.cur.disp_x;
                y = self.cont_3.buf_row_posi;
            }
            str_vec.push(MoveTo(x as u16, y as u16).to_string());
        } else if tab.state.is_enc_nl {
            tab.prom.draw_cur_enc_nl(str_vec);
        } else if tab.state.is_menu {
            tab.prom.draw_cur_menu(str_vec);
        }
    }

    pub fn cursor_down(&mut self, tab_state: &TabState) {
        Log::debug_s("              cursor_down");
        if tab_state.is_replace {
            if self.prom_cont_posi == PromptContPosi::First {
                self.prom_cont_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            }
        } else if tab_state.grep_info.is_grep {
            if self.prom_cont_posi == PromptContPosi::First {
                self.prom_cont_posi = PromptContPosi::Second;
                Prompt::set_cur(&self.cont_1, &mut self.cont_2)
            } else if self.prom_cont_posi == PromptContPosi::Second {
                self.prom_cont_posi = PromptContPosi::Third;
                Prompt::set_cur(&self.cont_2, &mut self.cont_3)
            }
        }
    }

    pub fn cursor_up(&mut self, tab_state: &TabState) {
        if tab_state.is_replace {
            if self.prom_cont_posi == PromptContPosi::Second {
                self.prom_cont_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            }
        } else if tab_state.grep_info.is_grep {
            if self.prom_cont_posi == PromptContPosi::Second {
                self.prom_cont_posi = PromptContPosi::First;
                Prompt::set_cur(&self.cont_2, &mut self.cont_1)
            } else if self.prom_cont_posi == PromptContPosi::Third {
                self.prom_cont_posi = PromptContPosi::Second;
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

    pub fn ctrl_mouse(&mut self, x: u16, y: u16, state: &TabState, is_mouse_left_down: bool) {
        Log::debug_s("              PromptCont.ctrl_mouse");

        if y == self.cont_1.buf_row_posi {
            self.prom_cont_posi = PromptContPosi::First;
            if !state.is_open_file {
                self.cont_1.ctrl_mouse(x, y, is_mouse_left_down);
            }
        } else if y == self.cont_2.buf_row_posi {
            if !state.is_open_file {
                self.prom_cont_posi = PromptContPosi::Second;
                self.cont_2.ctrl_mouse(x, y, is_mouse_left_down);
            }
        } else if y == self.cont_3.buf_row_posi {
            self.prom_cont_posi = PromptContPosi::Third;
            self.cont_3.ctrl_mouse(x, y, is_mouse_left_down);
        }
    }

    pub fn shift_right(&mut self) {
        match &self.prom_cont_posi {
            First => self.cont_1.shift_right(),
            Second => self.cont_2.shift_right(),
            Third => self.cont_3.shift_right(),
            _ => {}
        }
    }
    pub fn shift_left(&mut self) {
        match &self.prom_cont_posi {
            First => self.cont_1.shift_left(),
            Second => self.cont_2.shift_left(),
            Third => self.cont_3.shift_left(),
            _ => {}
        }
    }
    pub fn shift_home(&mut self) {
        match &self.prom_cont_posi {
            First => self.cont_1.shift_home(),
            Second => self.cont_2.shift_home(),
            Third => self.cont_3.shift_home(),
            _ => {}
        }
    }
    pub fn shift_end(&mut self) {
        match &self.prom_cont_posi {
            First => self.cont_1.shift_end(),
            Second => self.cont_2.shift_end(),
            Third => self.cont_3.shift_end(),
            _ => {}
        }
    }
    pub fn insert_char(&mut self, c: char, rnw: usize, state: &TabState) {
        match self.prom_cont_posi {
            First => self.cont_1.insert_char(c, state.is_move_line, rnw),
            Second => self.cont_2.insert_char(c, state.is_move_line, rnw),
            Third => self.cont_3.insert_char(c, state.is_move_line, rnw),
            _ => {}
        }
    }
    pub fn paste(&mut self, clipboard: &String) {
        match &self.prom_cont_posi {
            First => self.cont_1.paste(clipboard),
            Second => self.cont_2.paste(clipboard),
            Third => self.cont_3.paste(clipboard),
            _ => {}
        }
    }

    pub fn operation(&mut self, code: KeyCode) {
        if self.prom_open_file.vec_y != PromOpenFile::PATH_INPUT_FIELD {
            if code == KeyCode::Left || code == KeyCode::Right {
                return;
            }
        }
        match &self.prom_cont_posi {
            First => self.cont_1.operation(code),
            Second => self.cont_2.operation(code),
            Third => self.cont_3.operation(code),
            _ => {}
        }
    }

    pub fn tab(&mut self, is_asc: bool, tab_state: &TabState) {
        if tab_state.is_replace {
            match self.prom_cont_posi {
                PromptContPosi::First => self.cursor_down(tab_state),
                PromptContPosi::Second => self.cursor_up(tab_state),
                _ => {}
            }
        } else if tab_state.grep_info.is_grep {
            match self.prom_cont_posi {
                PromptContPosi::First => {
                    if is_asc {
                        self.cursor_down(tab_state);
                    } else {
                        self.prom_cont_posi = PromptContPosi::Third;
                        Prompt::set_cur(&self.cont_1, &mut self.cont_3);
                    }
                }
                PromptContPosi::Second => {
                    if is_asc {
                        self.cursor_down(tab_state);
                    } else {
                        self.cursor_up(tab_state);
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
        } else if tab_state.is_open_file {
            let str = self.cont_1.buf[..self.cont_1.cur.x].iter().collect::<String>();

            Log::debug("str", &str);

            self.cont_1.buf = self.prom_open_file.tab_comp.get_tab_candidate(is_asc, str, false).chars().collect();

            let (cur_x, width) = get_row_width(&self.cont_1.buf[..], 0, false);
            self.cont_1.cur.x = cur_x;
            self.cont_1.cur.disp_x = width;
        } else if tab_state.is_enc_nl {
            self.tab_enc_nl(is_asc);
        }
    }

    pub fn new() -> Self {
        Prompt { ..Prompt::default() }
    }
    pub fn clear(&mut self) {
        Log::debug_s("              Prompt.clear");
        self.disp_row_num = 0;
        self.disp_row_posi = 0;
        self.disp_col_num = 0;
        self.cont_1 = PromptCont::default();
        self.cont_2 = PromptCont::default();
        self.cont_3 = PromptCont::default();
        self.cont_4 = PromptCont::default();
        self.prom_cont_posi = PromptContPosi::First;
    }
}

#[derive(Debug, Clone)]
pub struct Prompt {
    pub disp_row_num: usize,
    pub disp_row_posi: usize,
    pub disp_col_num: usize,
    // Prompt Content_Sequence number
    pub cont_1: PromptCont,
    pub cont_2: PromptCont,
    pub cont_3: PromptCont,
    pub cont_4: PromptCont,
    pub prom_cont_posi: PromptContPosi,
    pub prom_open_file: PromOpenFile,
    pub prom_grep: PromGrep,
    pub prom_search: PromSearch,
}

impl Default for Prompt {
    fn default() -> Self {
        Prompt {
            disp_row_num: 0,
            disp_row_posi: 0,
            disp_col_num: 0,
            //  is_grep_result: false,
            //  is_grep_result_cancel: false,
            cont_1: PromptCont::default(),
            cont_2: PromptCont::default(),
            cont_3: PromptCont::default(),
            cont_4: PromptCont::default(),
            prom_cont_posi: PromptContPosi::First,
            prom_open_file: PromOpenFile::default(),
            prom_grep: PromGrep::default(),
            prom_search: PromSearch::default(),
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
