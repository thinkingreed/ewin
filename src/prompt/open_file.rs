use crate::{colors::*, def::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*, util::*};
use crossterm::{
    cursor::MoveTo,
    event::{Event::*, KeyCode::*, KeyEvent, KeyModifiers, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind},
    terminal::ClearType::*,
    terminal::*,
};
use std::{
    cmp::min,
    env,
    path::{self, Path, MAIN_SEPARATOR},
    usize,
};

impl EvtAct {
    pub fn open_file(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.open_file");
        Log::debug("term.curt().editor.evt ", &term.curt().editor.evt);

        match term.curt().editor.evt {
            Resize(_, _) => {
                PromOpenFile::set_file_list(&mut term.curt().prom);
                return EvtActType::Next;
            }
            Mouse(M_Event { kind: M_Kind::ScrollDown, .. }) => PromOpenFile::move_row_vec(term, CurDirection::Down),
            Mouse(M_Event { kind: M_Kind::ScrollUp, .. }) => PromOpenFile::move_row_vec(term, CurDirection::Up),
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);
                if y != term.curt().prom.cont_1.buf_row_posi as usize && !(term.curt().prom.cont_2.buf_row_posi as usize <= y && y <= term.curt().prom.cont_2.buf_row_posi as usize + term.curt().prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM) {
                    return EvtActType::Hold;
                } else {
                    if y == term.curt().prom.cont_1.buf_row_posi as usize {
                        let vec = split_inclusive(&term.curt().prom.cont_1.buf.iter().collect::<String>(), path::MAIN_SEPARATOR);

                        let (mut all_width, mut path_str) = (0, String::new());
                        for path in vec {
                            if path == path::MAIN_SEPARATOR.to_string() {
                                all_width += 1;
                            } else {
                                let w = get_str_width(&path);
                                if all_width < x && x < all_width + w {
                                    path_str.push_str(&path);
                                    if Path::new(&path_str).metadata().unwrap().is_dir() {
                                        path_str.push(path::MAIN_SEPARATOR);
                                        PromOpenFile::set_file_path(&mut term.curt().prom, &path_str);
                                        PromOpenFile::set_file_list(&mut term.curt().prom);
                                    }
                                    break;
                                }
                                all_width += w;
                            }
                            path_str.push_str(&path);
                        }
                    } else if term.curt().prom.cont_2.buf_row_posi as usize <= y && y <= term.curt().prom.cont_2.buf_row_posi as usize + term.curt().prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM {
                        let disp_row_posi = term.curt().prom.cont_2.buf_row_posi as usize;
                        let op_file_vec = term.curt().prom.prom_open_file.row_vec.clone();
                        let dest = min(term.curt().prom.prom_open_file.row_vec.len(), term.curt().prom.prom_open_file.offset + term.curt().prom.prom_open_file.disp_row_len);
                        for (row_idx, vec) in op_file_vec[term.curt().prom.prom_open_file.offset..dest].iter().enumerate() {
                            for op_file in vec.iter() {
                                if y - disp_row_posi == row_idx && op_file.filenm_area.0 <= x && x <= op_file.filenm_area.1 {
                                    return PromOpenFile::select_file(term, op_file, true);
                                }
                            }
                        }
                    }
                }
            }
            Key(KeyEvent { modifiers: KeyModifiers::SHIFT, code }) => match code {
                Char(_) => PromOpenFile::set_file_list(&mut term.curt().prom),
                _ => return EvtActType::Hold,
            },
            Key(KeyEvent { code, .. }) => match code {
                Up => PromOpenFile::move_row_vec(term, CurDirection::Up),
                Down => PromOpenFile::move_row_vec(term, CurDirection::Down),
                Left => PromOpenFile::move_row_vec(term, CurDirection::Left),
                Right => PromOpenFile::move_row_vec(term, CurDirection::Right),
                Char(_) | Delete | Backspace | Home | End | Tab => PromOpenFile::set_file_list(&mut term.curt().prom),
                Enter => {
                    let path_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    let path = Path::new(&path_str);
                    term.curt().prom.prom_open_file.cache_open_filenm = path_str.clone();
                    if path_str.len() == 0 {
                        term.curt().mbar.set_err(&LANG.not_entered_filenm);
                    } else if !path.exists() {
                        term.curt().mbar.set_err(&LANG.file_not_found);
                    } else if path.metadata().unwrap().is_dir() {
                        PromOpenFile::set_file_list(&mut term.curt().prom);
                        // term.curt().mbar.set_err(&LANG.file_not_found);
                    } else {
                        if term.open(&path.display().to_string(), &mut Tab::new()) {
                            term.clear_pre_tab_status();
                        }
                        return EvtActType::DrawOnly;
                    }
                    term.curt().editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
        return EvtActType::Hold;
    }
}

impl Prompt {
    const OPEN_FILE_FIXED_PHRASE_ROW_NUM: usize = 5;
    pub fn open_file(term: &mut Terminal) {
        term.curt().state.is_open_file = true;

        let rows = size().unwrap().1 as usize;
        // -1 is MsgBar
        let disp_row_num = rows - term.hbar.disp_row_num - term.help.disp_row_num - term.curt().sbar.disp_row_num - 1;
        term.curt().prom.disp_row_num = disp_row_num as usize;
        term.set_disp_size();
        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First).get_open_file(&term.curt().prom);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Second).get_open_file(&term.curt().prom);
        PromOpenFile::set_file_list(&mut term.curt().prom);
    }

    pub fn draw_open_file(&self, str_vec: &mut Vec<String>) {
        Log::debug_s("              　　　　　draw_open_file");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());

        let num_of_disp = format!("{}/{}", self.prom_open_file.get_disp_file_count(), self.prom_open_file.file_all_count);
        let cont_2_buf_desc = format!("{}{}({}){}", Colors::get_msg_highlight_fg(), &LANG.file_list, num_of_disp, Colors::get_default_fg());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &cont_2_buf_desc);

        Log::debug("self.prom_open_file", &self.prom_open_file);

        // cont_2.buf
        for y in 0..self.prom_open_file.disp_row_len {
            str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), Clear(CurrentLine)));
            let mut row_str = String::new();
            let vec_y = y + self.prom_open_file.offset;
            if let Some(vec) = self.prom_open_file.row_vec.get(vec_y) {
                for (x, op_file) in vec.iter().enumerate() {
                    let file_disp_str = &self.prom_open_file.get_file_disp_str(&op_file, vec_y, x);
                    row_str.push_str(&format!("{}{}{}", file_disp_str, op_file.filenm_disp, Colors::get_default_fg_bg()));
                }
            }
            if !row_str.is_empty() {
                str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), row_str));
            }
        }
    }
}

impl PromptCont {
    pub fn get_open_file(&mut self, prom: &Prompt) -> PromptCont {
        let base_posi = self.disp_row_posi - 1;

        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_open_filenm);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}Esc  {}{}:{}Tab  {}{}:{}Click  {}{}:{}↑↓",
                Colors::get_default_fg(),
                &LANG.open,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.select,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.scroll,
                Colors::get_msg_highlight_fg(),
            );

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.filenm, Colors::get_default_fg());

            if prom.prom_open_file.cache_open_filenm.len() > 0 {
                self.buf = prom.prom_open_file.cache_open_filenm.chars().collect();
            } else {
                if let Ok(path) = env::current_dir() {
                    self.buf = format!("{}{}", path.to_string_lossy().to_string(), path::MAIN_SEPARATOR).chars().collect();
                }
            };

            self.cur.x = self.buf.len();
            self.cur.disp_x = get_str_width(&self.buf.iter().collect::<String>());

            self.guide_row_posi = base_posi;
            self.key_desc_row_posi = base_posi + 1;
            self.buf_desc_row_posi = base_posi + 2;
            self.buf_row_posi = base_posi + 3;
        } else if self.posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.file_list, Colors::get_default_fg());
            self.buf_desc_row_posi = base_posi + 4;
            self.buf_row_posi = base_posi + 5;
        }
        return self.clone();
    }
}

impl PromOpenFile {
    pub const PATH_INPUT_FIELD: usize = usize::MAX;

    pub fn set_file_path(prom: &mut Prompt, path: &String) {
        let width = get_str_width(path);
        prom.cont_1.cur.disp_x = width;
        prom.cont_1.cur.x = path.chars().count();
        prom.cont_1.buf = path.chars().collect();
    }
    pub fn set_file_path_parent(prom: &mut Prompt, path: &String) {
        if path == &MAIN_SEPARATOR.to_string() {
            return;
        }
        let mut parent_str = Path::new(path).parent().unwrap().display().to_string();
        if &parent_str != &MAIN_SEPARATOR.to_string() {
            parent_str.push_str(&MAIN_SEPARATOR.to_string());
        }
        PromOpenFile::set_file_path(prom, &parent_str);
    }

    pub fn select_file(term: &mut Terminal, op_file: &OpenFile, is_click: bool) -> EvtActType {
        let path_str = &term.curt().prom.cont_1.buf.iter().collect::<String>();
        if op_file.file.is_dir {
            if op_file.file.name == PARENT_FOLDER {
                if is_click {
                    PromOpenFile::set_file_path_parent(&mut term.curt().prom, path_str);
                } else {
                    let base_path = term.curt().prom.prom_open_file.base_path.clone();
                    PromOpenFile::set_file_path_parent(&mut term.curt().prom, &base_path);
                }
            } else {
                PromOpenFile::chenge_file_path(&mut term.curt().prom, op_file);
            }
        } else {
            if is_click {
                if term.open(&format!("{}{}", path_str, op_file.file.name), &mut Tab::new()) {
                    term.clear_pre_tab_status();
                }
            } else {
                PromOpenFile::chenge_file_path(&mut term.curt().prom, op_file);
            }
            return EvtActType::DrawOnly;
        }
        if is_click {
            PromOpenFile::set_file_list(&mut term.curt().prom);
        }
        return EvtActType::Hold;
    }

    pub fn chenge_file_path(prom: &mut Prompt, op_file: &OpenFile) {
        let mut path = prom.prom_open_file.base_path.clone();
        path.push_str(&op_file.file.name);
        if op_file.file.is_dir {
            path.push_str(&MAIN_SEPARATOR.to_string());
        }
        PromOpenFile::set_file_path(prom, &path);
    }

    pub fn set_file_list(prom: &mut Prompt) {
        // Initialize
        prom.prom_open_file.offset = 0;
        prom.prom_open_file.tgt_x = 0;
        prom.prom_open_file.tgt_y = PromOpenFile::PATH_INPUT_FIELD;

        let path = prom.cont_1.buf[..prom.cont_1.cur.x].iter().collect::<String>();
        let mut vec = get_tab_comp_files(path, false, false);
        vec.insert(0, File { name: PARENT_FOLDER.to_string(), is_dir: true });
        let (op_file_row_vec, file_count) = get_shaping_file_list(&mut vec, prom.disp_col_num);
        prom.prom_open_file.row_vec = op_file_row_vec;
        prom.prom_open_file.file_all_count = file_count;
        prom.prom_open_file.disp_row_len = prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM;
    }

    pub fn get_disp_file_count(&self) -> usize {
        let mut count = 0;
        let dest = min(self.row_vec.len(), self.offset + self.disp_row_len);
        for vec in self.row_vec[0..dest].iter() {
            count += vec.len();
        }
        return count;
    }

    pub fn down_disp_file_list(&mut self) {
        if self.row_vec.len() - 1 - self.offset >= self.disp_row_len {
            self.offset += 1;
        }
    }

    pub fn up_disp_file_list(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn move_row_vec(term: &mut Terminal, cur_direction: CurDirection) {
        // Initialize
        if cur_direction == CurDirection::Down && term.curt().prom.prom_open_file.tgt_y == PromOpenFile::PATH_INPUT_FIELD {
            let mut path = term.curt().prom.cont_1.buf.iter().collect::<String>();
            path = get_dir_path(path);
            term.curt().prom.prom_open_file.base_path = path;
        }

        term.curt().prom.prom_open_file.move_vec(cur_direction);
        if term.curt().prom.prom_open_file.tgt_y == PromOpenFile::PATH_INPUT_FIELD {
            let base_path = &term.curt().prom.prom_open_file.base_path.clone();
            PromOpenFile::set_file_path(&mut term.curt().prom, base_path);
            return;
        }
        let (y, x) = (term.curt().prom.prom_open_file.tgt_y, term.curt().prom.prom_open_file.tgt_x);
        let op_file = &term.curt().prom.prom_open_file.row_vec.get(y).unwrap().get(x).unwrap().clone();

        let _ = PromOpenFile::select_file(term, &op_file, false);
    }
    pub fn move_vec(&mut self, cur_direction: CurDirection) {
        match cur_direction {
            CurDirection::Right | CurDirection::Left => {
                if let Some(vec) = self.row_vec.get(self.tgt_y) {
                    if cur_direction == CurDirection::Right {
                        if let Some(_) = vec.get(self.tgt_x + 1) {
                            self.tgt_x += 1;
                        };
                    } else {
                        if self.tgt_x != 0 {
                            if let Some(_) = vec.get(self.tgt_x - 1) {
                                self.tgt_x -= 1;
                            };
                        }
                    }
                }
            }
            CurDirection::Up => {
                if self.tgt_y == 0 {
                    self.tgt_y = PromOpenFile::PATH_INPUT_FIELD;
                } else {
                    if let Some(vec) = self.row_vec.get(self.tgt_y - 1) {
                        if let Some(_) = vec.get(self.tgt_x) {
                            self.tgt_y -= 1;
                        };
                        if self.tgt_y < self.offset {
                            self.offset -= 1;
                        }
                    }
                }
            }
            CurDirection::Down => {
                if self.tgt_y == PromOpenFile::PATH_INPUT_FIELD {
                    // If the file exists other than "..", specify it.
                    if self.tgt_x == 0 {
                        self.tgt_y = if self.row_vec.get(1).is_some() { 1 } else { 0 };
                    } else {
                        self.tgt_y = 0;
                    }
                } else {
                    if let Some(vec) = self.row_vec.get(self.tgt_y + 1) {
                        if let Some(_) = vec.get(self.tgt_x) {
                            if self.tgt_y >= self.offset + self.disp_row_len - 1 {
                                self.offset += 1;
                            }
                            self.tgt_y += 1;
                        };
                    }
                }
            }
        }
    }
    pub fn get_file_disp_str(&self, op_file: &OpenFile, y: usize, x: usize) -> String {
        if y == self.tgt_y && x == self.tgt_x {
            return if op_file.file.is_dir {
                Colors::get_file_dir_inversion_fg_bg()
            } else {
                if File::is_executable(&op_file.file.name) {
                    Colors::get_file_executable_inversion_fg_bg()
                } else {
                    Colors::get_file_normal_inversion_fg_bg()
                }
            };
        } else {
            return if op_file.file.is_dir {
                Colors::get_file_dir_fg_bg()
            } else {
                if File::is_executable(&op_file.file.name) {
                    Colors::get_file_executable_fg_bg()
                } else {
                    Colors::get_file_normal_fg_bg()
                }
            };
        }
    }
}

// Cursor direction
#[derive(Debug, PartialEq)]
pub enum CurDirection {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromOpenFile {
    pub row_vec: Vec<Vec<OpenFile>>,
    pub file_all_count: usize,
    pub offset: usize,
    pub disp_row_len: usize,
    pub cache_open_filenm: String,
    pub tab_comp: TabComp,
    pub tgt_y: usize,
    pub tgt_x: usize,
    pub base_path: String,
}

impl Default for PromOpenFile {
    fn default() -> Self {
        PromOpenFile {
            row_vec: vec![],
            file_all_count: 0,
            offset: 0,
            disp_row_len: 0,
            cache_open_filenm: String::new(),
            tab_comp: TabComp::default(),
            tgt_y: PromOpenFile::PATH_INPUT_FIELD,
            tgt_x: 0,
            base_path: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFile {
    pub file: File,
    pub filenm_disp: String,
    pub filenm_area: (usize, usize),
    // pub is_select: bool,
}

impl Default for OpenFile {
    fn default() -> Self {
        OpenFile {
            file: File::default(),
            filenm_disp: String::new(),
            filenm_area: (USIZE_UNDEFINED, USIZE_UNDEFINED),
            // is_select: false,
        }
    }
}
