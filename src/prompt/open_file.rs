use crate::{_cfg::keys::*, colors::*, def::*, global::*, log::*, model::*, prompt::cont::promptcont::*, prompt::prompt::prompt::*, tab::Tab, terminal::*, util::*};
use crossterm::{cursor::MoveTo, terminal::ClearType::*, terminal::*};
use std::{
    cmp::min,
    env,
    path::{self, Path, MAIN_SEPARATOR},
    usize,
};

impl EvtAct {
    pub fn open_file(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.open_file");

        match term.curt().editor.keycmd {
            KeyCmd::Resize => {
                term.set_disp_size();
                PromOpenFile::set_file_list(&mut term.curt().prom);
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::Tab => PromOpenFile::set_file_list(&mut term.curt().prom),
            KeyCmd::CursorUp => PromOpenFile::move_vec(term, CurDirection::Up),
            KeyCmd::CursorDown => PromOpenFile::move_vec(term, CurDirection::Down),
            KeyCmd::CursorLeft | KeyCmd::CursorRight => {
                if term.curt().prom.prom_open_file.vec_y == PromOpenFile::PATH_INPUT_FIELD {
                    PromOpenFile::set_file_list(&mut term.curt().prom);
                } else {
                    let cur_direction = if term.curt().editor.keycmd == KeyCmd::CursorLeft { CurDirection::Left } else { CurDirection::Right };
                    PromOpenFile::move_vec(term, cur_direction);
                }
            }
            KeyCmd::MouseDownLeft(y, x) => {
                if y != term.curt().prom.cont_1.buf_row_posi as usize && !(term.curt().prom.cont_2.buf_row_posi as usize <= y && y <= term.curt().prom.cont_2.buf_row_posi as usize + term.curt().prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM) {
                    return EvtActType::Hold;
                } else {
                    // File path
                    if y == term.curt().prom.cont_1.buf_row_posi as usize {
                        let disp_vec = split_inclusive(&term.curt().prom.cont_1.buf.iter().collect::<String>(), path::MAIN_SEPARATOR);

                        // Identifying the path of the clicked position
                        let (mut all_width, mut path_str) = (0, String::new());
                        for path in disp_vec.iter() {
                            if path == &path::MAIN_SEPARATOR.to_string() {
                                all_width += 1;
                            } else {
                                let width = get_str_width(&path);
                                if all_width <= x && x <= all_width + width {
                                    path_str.push_str(&path);
                                    path_str = path_str.replace(CONTINUE_STR, &term.curt().prom.prom_open_file.omitted_path_str);
                                    if Path::new(&path_str).metadata().unwrap().is_dir() {
                                        path_str.push(path::MAIN_SEPARATOR);
                                        PromOpenFile::set_file_path(&mut term.curt().prom, &path_str);
                                        PromOpenFile::set_file_list(&mut term.curt().prom);
                                    }
                                    break;
                                }
                                all_width += width;
                            }
                            path_str.push_str(&path);
                        }
                        // File list
                    } else if term.curt().prom.cont_2.buf_row_posi as usize <= y && y <= term.curt().prom.cont_2.buf_row_posi as usize + term.curt().prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM {
                        let disp_row_posi = term.curt().prom.cont_2.buf_row_posi as usize;
                        let op_file_vec = term.curt().prom.prom_open_file.vec.clone();
                        let dest = min(term.curt().prom.prom_open_file.vec.len(), term.curt().prom.prom_open_file.offset + term.curt().prom.prom_open_file.disp_row_len);
                        // Identifying the file of the clicked position
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
            KeyCmd::MouseScrollUp => PromOpenFile::move_vec(term, CurDirection::Up),
            KeyCmd::MouseScrollDown => PromOpenFile::move_vec(term, CurDirection::Down),
            KeyCmd::InsertLine => {
                let path_str = term.curt().prom.cont_1.buf.iter().collect::<String>();
                let full_path_str = term.curt().prom.prom_open_file.select_open_file(&path_str);
                let path = Path::new(&full_path_str);

                if path_str.len() == 0 {
                    term.curt().mbar.set_err(&LANG.not_entered_filenm);
                } else if !path.exists() {
                    term.curt().mbar.set_err(&LANG.file_not_found);
                } else if !File::is_readable(&full_path_str) {
                    term.curt().mbar.set_err(&LANG.no_read_permission);
                } else if path.metadata().unwrap().is_dir() {
                    PromOpenFile::set_file_list(&mut term.curt().prom);
                } else {
                    let mut tgt_idx = USIZE_UNDEFINED;
                    // Check if the file is already open
                    for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                        if full_path_str == h_file.fullpath {
                            tgt_idx = idx;
                        }
                    }
                    if tgt_idx == USIZE_UNDEFINED {
                        if term.open(&path.display().to_string(), &mut Tab::new()) {
                            term.clear_pre_tab_status();
                        }
                    } else {
                        term.idx = tgt_idx;
                        term.curt().editor.keys = Keys::Null;
                    }
                    return EvtActType::DrawOnly;
                }
                term.curt().editor.d_range.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
        return EvtActType::Hold;
    }
}

impl Prompt {
    pub const OPEN_FILE_FIXED_PHRASE_ROW_NUM: usize = 5;

    pub fn open_file(term: &mut Terminal) {
        Log::debug_key("open_file");
        term.curt().state.is_open_file = true;
        let is_disp = term.set_disp_size();
        if !is_disp {
            term.clear_curt_tab();
            term.curt().mbar.set_err(&LANG.increase_height_width_terminal);
            return;
        }

        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt(), PromptContPosi::First).get_open_file(&mut term.curt().prom);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt(), PromptContPosi::Second).get_open_file(&mut term.curt().prom);
        PromOpenFile::set_file_list(&mut term.curt().prom);

        term.curt().prom.prom_open_file.base_path = get_dir_path(&term.curt().prom.cont_1.buf.iter().collect::<String>().replace(CONTINUE_STR, &term.curt().prom.prom_open_file.omitted_path_str));
    }

    pub fn draw_open_file(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_key("  draw_open_file");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());

        let num_of_disp = format!("{}/{}", self.prom_open_file.get_disp_file_count(), self.prom_open_file.file_all_count);
        let cont_2_buf_desc = format!("{}{}({}){}", Colors::get_msg_highlight_fg(), &LANG.file_list, num_of_disp, Colors::get_default_fg());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &cont_2_buf_desc);

        // cont_2.buf
        let mut file_disp_str_org = String::new();
        for y in 0..self.prom_open_file.disp_row_len {
            str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), Clear(CurrentLine)));
            let mut row_str = String::new();
            let vec_y = y + self.prom_open_file.offset;
            if let Some(vec) = self.prom_open_file.vec.get(vec_y) {
                for (x, op_file) in vec.iter().enumerate() {
                    let file_disp_str = &self.prom_open_file.get_file_disp_str(&op_file, vec_y, x);

                    if file_disp_str != &file_disp_str_org {
                        row_str.push_str(&format!("{}{}", file_disp_str, op_file.filenm_disp));
                    } else {
                        row_str.push_str(&format!("{}", op_file.filenm_disp));
                    }
                    file_disp_str_org = file_disp_str.clone();
                }
            }
            if !row_str.is_empty() {
                str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), row_str));
            }
            str_vec.push(format!("{}", Colors::get_default_fg_bg()));
        }
    }
}

impl PromptCont {
    pub fn get_open_file(&mut self, prom: &mut Prompt) -> PromptCont {
        let base_posi = self.disp_row_posi;

        if self.posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_open_filenm);
            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}{}  {}{}:{}Tab  {}{}:{}Click  {}{}:{}↑↓←→",
                Colors::get_default_fg(),
                &LANG.open,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::ConfirmPrompt),
                Colors::get_default_fg(),
                &LANG.close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::EscPrompt),
                Colors::get_default_fg(),
                &LANG.complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.select,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &LANG.movement,
                Colors::get_msg_highlight_fg(),
            );

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.filenm, Colors::get_default_fg());

            if prom.prom_open_file.cache_disp_filenm.len() > 0 {
                self.buf = prom.prom_open_file.cache_disp_filenm.chars().collect();
                prom.prom_open_file.base_path = prom.prom_open_file.cache_full_filenm.clone();
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
    pub const PATH_INPUT_FIELD: usize = USIZE_UNDEFINED;

    pub fn set_file_path(prom: &mut Prompt, path: &String) {
        let path = &path.replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str);
        // -2 is margin
        let disp_path = cut_str(path.clone(), prom.disp_col_num - 2, true, true);

        let tmp = disp_path.replace(CONTINUE_STR, "");
        prom.prom_open_file.omitted_path_str = path.replace(&tmp, "");

        Log::debug("File path omitted_path_str", &prom.prom_open_file.omitted_path_str);

        let width = get_str_width(&disp_path);
        prom.cont_1.cur.disp_x = width;
        prom.cont_1.cur.x = disp_path.chars().count();
        prom.cont_1.buf = disp_path.chars().collect();
    }

    pub fn set_file_path_parent(prom: &mut Prompt, path: &String) {
        if File::is_root_dir(path) {
            PromOpenFile::set_file_path(prom, &path);
            return;
        }
        let path = &path.replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str);

        let mut parent_str = Path::new(path).parent().unwrap().display().to_string();
        if !File::is_root_dir(&parent_str) {
            parent_str.push_str(&MAIN_SEPARATOR.to_string());
        }
        PromOpenFile::set_file_path(prom, &parent_str);
    }

    pub fn select_file(term: &mut Terminal, op_file: &OpenFile, is_click: bool) -> EvtActType {
        // let path = &term.curt().prom.cont_1.buf.iter().collect::<String>();
        if op_file.file.is_dir {
            if op_file.file.name == PARENT_FOLDER {
                let base_path = term.curt().prom.prom_open_file.base_path.clone();
                PromOpenFile::set_file_path_parent(&mut term.curt().prom, &base_path);
            } else {
                let mut path = term.curt().prom.prom_open_file.base_path.clone();
                path.push_str(&op_file.file.name);
                if is_click && !File::is_readable(&path) {
                    term.curt().mbar.set_err(&LANG.no_read_permission);
                    return EvtActType::Hold;
                } else {
                    PromOpenFile::chenge_file_path(&mut term.curt().prom, op_file);
                }
            }
        } else {
            if is_click {
                let base_path = term.curt().prom.prom_open_file.base_path.clone();
                let base_path = term.curt().prom.prom_open_file.select_open_file(&base_path);
                if term.open(&format!("{}{}", &base_path, op_file.file.name), &mut Tab::new()) {
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

    pub fn select_open_file(&mut self, path: &String) -> String {
        self.cache_disp_filenm = get_dir_path(&path.clone());
        let path_str = path.replace(CONTINUE_STR, &self.omitted_path_str);
        self.cache_full_filenm = path_str.clone();
        return path_str;
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
        Log::debug_s("set_file_list");

        // Initialize
        prom.prom_open_file.offset = 0;
        prom.prom_open_file.vec_x = 0;
        prom.prom_open_file.vec_y = PromOpenFile::PATH_INPUT_FIELD;

        let path = prom.cont_1.buf[..prom.cont_1.cur.x].iter().collect::<String>();
        let path = path.replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str);

        let mut vec = get_tab_comp_files(path, false, false);
        vec.insert(0, File { name: PARENT_FOLDER.to_string(), is_dir: true });

        let (op_file_row_vec, file_count) = get_shaping_file_list(&mut vec, prom.disp_col_num);
        prom.prom_open_file.vec = op_file_row_vec;
        prom.prom_open_file.file_all_count = file_count;
        prom.prom_open_file.disp_row_len = prom.disp_row_num - Prompt::OPEN_FILE_FIXED_PHRASE_ROW_NUM;
        //  prom.prom_open_file.base_path = get_dir_path(prom.cont_1.buf.iter().collect());
        prom.prom_open_file.base_path = get_dir_path(&prom.cont_1.buf.iter().collect::<String>().replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str));
    }

    pub fn get_disp_file_count(&self) -> usize {
        let mut count = 0;
        let dest = min(self.vec.len(), self.offset + self.disp_row_len);
        for vec in self.vec[0..dest].iter() {
            count += vec.len();
        }
        return count;
    }

    pub fn down_disp_file_list(&mut self) {
        if self.vec.len() - 1 - self.offset >= self.disp_row_len {
            self.offset += 1;
        }
    }

    pub fn up_disp_file_list(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
    }

    pub fn move_vec(term: &mut Terminal, cur_direction: CurDirection) {
        if term.curt().prom.prom_open_file.vec_y == PromOpenFile::PATH_INPUT_FIELD {
            match cur_direction {
                CurDirection::Up => return,
                CurDirection::Down => term.curt().prom.prom_open_file.base_path = get_dir_path(&term.curt().prom.cont_1.buf.iter().collect::<String>().replace(CONTINUE_STR, &term.curt().prom.prom_open_file.omitted_path_str)),
                _ => {}
            };
        }
        term.curt().prom.prom_open_file.set_vec_posi(cur_direction);
        if term.curt().prom.prom_open_file.vec_y == PromOpenFile::PATH_INPUT_FIELD {
            let base_path = term.curt().prom.prom_open_file.base_path.clone();
            PromOpenFile::set_file_path(&mut term.curt().prom, &base_path);
            return;
        }
        let (y, x) = (term.curt().prom.prom_open_file.vec_y, term.curt().prom.prom_open_file.vec_x);
        let op_file = &term.curt().prom.prom_open_file.vec.get(y).unwrap().get(x).unwrap().clone();

        let _ = PromOpenFile::select_file(term, &op_file, false);
    }

    pub fn set_vec_posi(&mut self, cur_direction: CurDirection) {
        match cur_direction {
            CurDirection::Right | CurDirection::Left => {
                if let Some(vec) = self.vec.get(self.vec_y) {
                    if cur_direction == CurDirection::Right {
                        if let Some(_) = vec.get(self.vec_x + 1) {
                            self.vec_x += 1;
                        };
                    } else {
                        if self.vec_x != 0 {
                            if let Some(_) = vec.get(self.vec_x - 1) {
                                self.vec_x -= 1;
                            };
                        }
                    }
                }
            }
            CurDirection::Up => {
                if self.vec_y == 0 {
                    self.vec_y = PromOpenFile::PATH_INPUT_FIELD;
                } else {
                    if let Some(vec) = self.vec.get(self.vec_y - 1) {
                        if let Some(_) = vec.get(self.vec_x) {
                            self.vec_y -= 1;
                        };
                        if self.vec_y < self.offset {
                            self.offset -= 1;
                        }
                    }
                }
            }
            CurDirection::Down => {
                if self.vec_y == PromOpenFile::PATH_INPUT_FIELD {
                    if self.vec_x == 0 {
                        if self.vec.len() == 1 {
                            self.vec_x = if self.vec.get(0).unwrap().get(1).is_some() { 1 } else { 0 };
                            self.vec_y = 0;
                        } else {
                            // If the file exists other than "..", specify it.
                            self.vec_y = 1
                        }
                    } else {
                        self.vec_y = 0;
                    }
                } else {
                    if let Some(vec) = self.vec.get(self.vec_y + 1) {
                        if let Some(_) = vec.get(self.vec_x) {
                            if self.vec_y >= self.offset + self.disp_row_len - 1 {
                                self.offset += 1;
                            }
                            self.vec_y += 1;
                        };
                    }
                }
            }
        }
    }
    pub fn get_file_disp_str(&self, op_file: &OpenFile, y: usize, x: usize) -> String {
        // Select
        return if y == self.vec_y && x == self.vec_x {
            if op_file.file.is_dir {
                Colors::get_file_dir_inversion_fg_bg()
            } else {
                if File::is_executable(&op_file.file.name) {
                    Colors::get_file_executable_inversion_fg_bg()
                } else {
                    Colors::get_file_normal_inversion_fg_bg()
                }
            }
        } else {
            if op_file.file.is_dir {
                Colors::get_file_dir_fg_bg()
            } else {
                if File::is_executable(&op_file.file.name) {
                    Colors::get_file_executable_fg_bg()
                } else {
                    Colors::get_file_normal_fg_bg()
                }
            }
        };
    }
}
pub fn get_shaping_file_list(file_vec: &mut Vec<File>, cols: usize) -> (Vec<Vec<OpenFile>>, usize) {
    const FILE_MAX_LEN: usize = 60;
    const FILE_MERGIN: usize = 2;

    let file_vec_len = &file_vec.len();

    let mut all_vec: Vec<Vec<OpenFile>> = vec![];
    let mut column_len_file_vec: Vec<(usize, Vec<OpenFile>)> = vec![];

    // From the order of the number of columns,
    // try to see if the total display width of each column fits in the width of the terminal,
    // and if it does not fit, subtract the number of columns.

    let mut max_len = FILE_MAX_LEN;
    for split_idx in (1..=13).rev() {
        let mut row_num = file_vec_len / split_idx;
        if row_num == 0 {
            continue;
        }
        if split_idx == 1 {
            max_len = size().unwrap().0 as usize;
        }

        let rest_num = file_vec_len % split_idx;
        if rest_num != 0 {
            row_num += 1;
        }

        let mut row_vec: Vec<OpenFile> = vec![];
        for (idx, file) in file_vec.iter_mut().enumerate() {
            row_vec.push(OpenFile { file: file.clone(), ..OpenFile::default() });
            if &row_vec.len() == &row_num || idx == file_vec_len - 1 {
                all_vec.push(row_vec.clone());
                row_vec = vec![];
            }
        }

        // Setting the display file name and calculating the maximum width for each column
        let all_vec_len = all_vec.len();
        let mut column_total_width = 0;
        for (idx, vec) in all_vec.iter_mut().enumerate() {
            let mut column_max_len = 0;
            for op_file in vec.iter_mut() {
                let mut filenm_len = get_str_width(&op_file.file.name);
                if filenm_len > max_len {
                    let cut_str = cut_str(op_file.file.name.clone(), max_len, false, true);
                    filenm_len = get_str_width(&cut_str);
                    op_file.filenm_disp = cut_str;
                } else {
                    op_file.filenm_disp = op_file.file.name.clone();
                }

                column_max_len = if filenm_len > column_max_len { filenm_len } else { column_max_len };
            }
            column_total_width += if all_vec_len - 1 == idx { column_max_len } else { column_max_len + FILE_MERGIN };
            column_max_len += if all_vec_len - 1 == idx { 0 } else { FILE_MERGIN };

            column_len_file_vec.push((column_max_len, vec.clone()));
        }
        if column_total_width <= cols {
            break;
        }
        all_vec.clear();
        column_len_file_vec.clear();
    }

    // Set the display file name for each column to the maximum width
    let mut all_row_vec: Vec<Vec<OpenFile>> = vec![];
    let mut all_count = 0;

    if column_len_file_vec.len() > 0 {
        let row_len = column_len_file_vec.first().unwrap().1.len();
        let colum_len = column_len_file_vec.len();

        Log::debug("row_len", &row_len);
        Log::debug("colum_len", &colum_len);

        for y in 0..row_len {
            let mut row_width = 0;
            let mut row_vec: Vec<OpenFile> = vec![];
            for x in 0..colum_len {
                if let Some((max_len, vec)) = column_len_file_vec.get_mut(x) {
                    if let Some(op_file) = vec.get_mut(y) {
                        let rest = *max_len - get_str_width(&op_file.filenm_disp);
                        op_file.filenm_disp = format!("{}{}", op_file.filenm_disp, " ".repeat(rest));
                        op_file.filenm_area = (row_width, row_width + *max_len - 1);
                        row_width += *max_len;
                        row_vec.push(op_file.clone());
                        all_count += 1;
                    }
                }
            }
            all_row_vec.push(row_vec);
        }
    }
    return (all_row_vec, all_count);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromOpenFile {
    pub vec: Vec<Vec<OpenFile>>,
    pub file_all_count: usize,
    pub offset: usize,
    pub disp_row_len: usize,
    pub cache_disp_filenm: String,
    pub cache_full_filenm: String,
    pub tab_comp: TabComp,
    pub vec_y: usize,
    pub vec_x: usize,
    pub base_path: String,
    pub omitted_path_str: String,
}

impl Default for PromOpenFile {
    fn default() -> Self {
        PromOpenFile { vec: vec![], file_all_count: 0, offset: 0, disp_row_len: 0, cache_disp_filenm: String::new(), cache_full_filenm: String::new(), tab_comp: TabComp::default(), vec_y: PromOpenFile::PATH_INPUT_FIELD, vec_x: 0, base_path: String::new(), omitted_path_str: String::new() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenFile {
    pub file: File,
    pub filenm_disp: String,
    pub filenm_area: (usize, usize),
}

impl Default for OpenFile {
    fn default() -> Self {
        OpenFile { file: File::default(), filenm_disp: String::new(), filenm_area: (USIZE_UNDEFINED, USIZE_UNDEFINED) }
    }
}
