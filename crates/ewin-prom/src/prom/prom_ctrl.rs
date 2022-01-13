use crate::{
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*, util::*},
    model::{PromptContPosi::*, *},
    prom::choice::*,
};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::{
    io::Write,
    path::{self, Path},
    u16,
};

impl Prompt {
    pub fn draw(&mut self, str_vec: &mut Vec<String>, prom_disp_row_posi: u16, tab_state: &TabState, is_exsist_msg: bool, h_file: &HeaderFile) {
        Log::info_key("Prompt.draw");

        if !tab_state.is_nomal_and_not_result() {
            //
            // is_search is for incremental search
            if self.is_first_draw() || !tab_state.is_search {
                self.draw_set_posi(tab_state, prom_disp_row_posi, h_file);
                if tab_state.grep.is_greping() {
                    Prompt::set_draw_vec_for_greping(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
                    Prompt::set_draw_vec_for_greping(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);
                } else {
                    Prompt::set_draw_vec(str_vec, self.cont_1.guide_row_posi, &self.cont_1.guide);
                    Prompt::set_draw_vec(str_vec, self.cont_1.key_desc_row_posi, &self.cont_1.key_desc);
                }
            }
            if tab_state.is_save_new_file || tab_state.is_move_row {
                Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());
            } else if tab_state.is_search {
                self.draw_search(str_vec);
            } else if tab_state.is_replace {
                self.draw_replace(str_vec);
            } else if tab_state.is_save_forced {
                self.draw_save_forced(str_vec);
            } else if tab_state.is_open_file {
                self.draw_open_file(str_vec, is_exsist_msg);
            } else if tab_state.is_menu {
                self.draw_menu(str_vec);
            } else if tab_state.grep.is_grep {
                self.draw_grep(str_vec);
            } else if tab_state.is_enc_nl {
                self.draw_enc_nl(str_vec);
            }
        }
    }

    pub fn get_serach_opt(&self) -> String {
        let o1 = &self.cont_1.opt_1;
        let o2 = &self.cont_1.opt_2;
        return format!("{}{}  {}{}", o1.key, o1.get_check_str(), o2.key, o2.get_check_str());
    }

    pub fn set_draw_vec(str_vec: &mut Vec<String>, posi: u16, str: &str) {
        str_vec.push(format!("{}{}{}", MoveTo(0, posi), Clear(CurrentLine), str));
    }
    pub fn set_draw_vec_for_greping(str_vec: &mut Vec<String>, posi: u16, str: &str) {
        str_vec.push(format!("{}{}", MoveTo(0, posi), str));
    }

    pub fn draw_only<T: Write>(&mut self, out: &mut T, tab_state: &TabState, is_exsist_msg: bool, h_file: &HeaderFile) {
        Log::debug_key("Prompt.draw_only");
        let mut v: Vec<String> = vec![];
        let prom_disp_row_posi = self.disp_row_posi;
        self.draw(&mut v, prom_disp_row_posi, tab_state, is_exsist_msg, h_file);
        self.draw_cur(&mut v, tab_state);
        let _ = out.write(v.concat().as_bytes());
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
        } else if state.grep.is_grep {
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
        } else if state.grep.is_grep {
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
        let (cur_x, width) = get_until_disp_x(&cont.buf, cont.updown_x, false);
        cont.cur.x = cur_x;
        cont.cur.disp_x = width;
    }

    pub fn clear_sels(&mut self) {
        self.cont_1.sel.clear();
        self.cont_2.sel.clear();
        self.cont_3.sel.clear();
    }
    pub fn clear_sels_keycmd(&mut self) {
        match self.p_cmd {
            P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorUp | P_Cmd::CursorDown | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd | P_Cmd::TabNextFocus | P_Cmd::BackTabBackFocus => {
                self.clear_sels();
            }
            _ => {}
        }
    }

    pub fn ctrl_mouse(&mut self, state: &TabState, y: usize, x: usize) {
        Log::debug_key("PromptCont.ctrl_mouse");
        let y = y as u16;

        if y == self.cont_1.buf_row_posi {
            self.cont_posi = PromptContPosi::First;
            if !state.is_open_file {
                self.cont_1.ctrl_mouse(x, y);
            }
        } else if y == self.cont_2.buf_row_posi {
            if !state.is_open_file {
                self.cont_posi = PromptContPosi::Second;
                self.cont_2.ctrl_mouse(x, y);
            }
        } else if y == self.cont_3.buf_row_posi {
            self.cont_posi = PromptContPosi::Third;
            self.cont_3.ctrl_mouse(x, y);
        }
    }

    pub fn shift_move_com(&mut self) {
        Log::debug_key("Prompt.shift_move_com");
        match &self.cont_posi {
            First => self.cont_1.shift_move_com(),
            Second => self.cont_2.shift_move_com(),
            Third => self.cont_3.shift_move_com(),

            _ => {}
        }
    }

    pub fn insert_str(&mut self, str: String) {
        match self.cont_posi {
            First => self.cont_1.edit_proc(P_Cmd::InsertStr(str)),
            Second => self.cont_2.edit_proc(P_Cmd::InsertStr(str)),
            Third => self.cont_3.edit_proc(P_Cmd::InsertStr(str)),

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

        match &cont.p_cmd {
            P_Cmd::InsertStr(_) | P_Cmd::Cut | P_Cmd::DelNextChar | P_Cmd::DelPrevChar => {
                cont.edit_proc(cont.p_cmd.clone());
            }
            P_Cmd::CursorLeft | P_Cmd::CursorRight | P_Cmd::CursorRowHome | P_Cmd::CursorRowEnd => {
                cont.cur_move();
            }
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
        } else if state.grep.is_grep {
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
                    let (cur_x, width) = get_row_cur_x_disp_x(&self.cont_3.buf[..], 0, false);
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

            let (cur_x, width) = get_row_cur_x_disp_x(&self.cont_1.buf[..], 0, false);
            self.cont_1.cur.x = cur_x;
            self.cont_1.cur.disp_x = width;
        } else if state.is_enc_nl {
            self.move_enc_nl(Direction::Right);
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
    pub fn set_cmd(&mut self, keycmd: KeyCmd) {
        Log::debug_key("Prompt::set_keys");
        //  let keycmd = Keybind::keys_to_keycmd(&keys, None, KeyWhen::PromptFocus);
        self.keycmd = keycmd.clone();
        let p_cmd = match &keycmd {
            KeyCmd::Prom(p_cmd) => p_cmd.clone(),
            _ => P_Cmd::Null,
        };
        self.p_cmd = p_cmd.clone();
        let keycmd = keycmd;
        match self.cont_posi {
            PromptContPosi::First => self.cont_1.set_key_info(keycmd, p_cmd),
            PromptContPosi::Second => self.cont_2.set_key_info(keycmd, p_cmd),
            PromptContPosi::Third => self.cont_3.set_key_info(keycmd, p_cmd),
            PromptContPosi::Fourth => self.cont_4.set_key_info(keycmd, p_cmd),
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
    pub fn is_first_draw(&mut self) -> bool {
        self.cont_1.guide_row_posi == 0
    }
}

impl TabComp {
    pub fn get_tab_candidate(&mut self, is_asc: bool, target_path: String, is_dir_only: bool) -> String {
        if self.files.is_empty() {
            self.files = get_tab_comp_files(target_path.clone(), is_dir_only, true);
        }

        let mut rtn_string = target_path;

        for file in &self.files {
            // One candidate

            match self.files.len() {
                0 => {}
                1 => {
                    if !is_dir_only {
                        let path = Path::new(&file.name);
                        //  let path = Path::new(&os_str);
                        rtn_string = if path.metadata().unwrap().is_file() { file.name.to_string() } else { format!("{}{}", file.name.to_string(), path::MAIN_SEPARATOR) };
                    } else {
                        rtn_string = format!("{}{}", file.name.to_string(), path::MAIN_SEPARATOR);
                    }
                    self.clear_tab_comp();
                    break;
                }
                _ => {
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
        }

        rtn_string
    }
    pub fn clear_tab_comp(&mut self) {
        Log::debug_s("clear_tab_comp ");
        self.index = USIZE_UNDEFINED;
        self.files.clear();
    }
}
