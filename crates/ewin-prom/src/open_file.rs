use crate::{
    ewin_com::{_cfg::key::keycmd::*, _cfg::lang::lang_cfg::*, colors::*, def::*, log::Log, model::*, util::*},
    model::*,
};
use crossterm::{cursor::MoveTo, terminal::ClearType::*, terminal::*};
use directories::BaseDirs;
use ewin_com::file::*;
use std::{
    cmp::min,
    env,
    path::{self, Path, *},
    usize,
};

impl Prompt {
    pub const OPEN_FILE_FIXED_PHRASE_ROW_NUM: usize = 5;

    pub fn open_file(&mut self, open_file_type: OpenFileType) {
        Log::debug_key("open_file");

        self.prom_open_file.file_type = open_file_type;
        self.cont_1 = PromptCont::new(Some(PromptContPosi::First)).get_open_file(self);
        self.cont_2 = PromptCont::new(Some(PromptContPosi::Second)).get_open_file(self);
        PromOpenFile::set_file_list(self);

        self.prom_open_file.base_path = get_dir_path(&self.cont_1.buf.iter().collect::<String>().replace(CONTINUE_STR, &self.prom_open_file.omitted_path_str));
    }

    pub fn draw_open_file(&mut self, str_vec: &mut Vec<String>, is_exsist_msg: bool) {
        Log::debug_key("draw_open_file");
        if !is_exsist_msg {
            str_vec.push(format!("{}{}", MoveTo(0, self.cont_1.disp_row_posi - 1), Clear(CurrentLine)));
        }

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_1.buf_row_posi, &self.cont_1.get_draw_buf_str());

        let num_of_disp = format!("{}/{}", self.prom_open_file.get_disp_file_count(), self.prom_open_file.file_all_count);
        let cont_2_buf_desc = format!("{}{}({}){}", Colors::get_msg_highlight_fg(), &Lang::get().file_list, num_of_disp, Colors::get_default_fg());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &cont_2_buf_desc);

        // cont_2.buf
        for y in 0..self.prom_open_file.disp_row_len {
            str_vec.push(format!("{}{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), Colors::get_default_fg_bg(), Clear(CurrentLine),));

            let mut row_str = String::new();
            let mut file_disp_str_org = String::new();
            let vec_y = y + self.prom_open_file.offset;
            if let Some(vec) = self.prom_open_file.vec.get(vec_y) {
                for (x, op_file) in vec.iter().enumerate() {
                    let file_disp_str = self.prom_open_file.get_file_disp_str(op_file, vec_y, x);
                    if file_disp_str != file_disp_str_org {
                        row_str.push_str(&format!("{}{}", file_disp_str, op_file.filenm_disp));
                    } else {
                        row_str.push_str(&op_file.filenm_disp);
                    }
                    file_disp_str_org = file_disp_str.clone();
                }
            }
            if !row_str.is_empty() {
                str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + y as u16), row_str));
            }
        }
        str_vec.push(Colors::get_default_fg_bg());
    }
}

impl PromptCont {
    pub fn get_open_file(&mut self, prom: &mut Prompt) -> PromptCont {
        Log::debug_s("PromptCont.get_open_file");

        if self.posi == PromptContPosi::First {
            let guide_str = match prom.prom_open_file.file_type {
                OpenFileType::Normal => &Lang::get().set_open_filenm,
                OpenFileType::JsMacro => &Lang::get().set_exec_mocro_filenm,
            };
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), guide_str);

            self.key_desc = format!(
                "{}{}:{}{}  {}{}:{}{}  {}{}:{}Tab  {}{}:{}Click  {}{}:{}↑↓←→",
                Colors::get_default_fg(),
                &Lang::get().open,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::ConfirmPrompt)),
                Colors::get_default_fg(),
                &Lang::get().close,
                Colors::get_msg_highlight_fg(),
                Keybind::get_key_str(KeyCmd::Prom(P_Cmd::EscPrompt)),
                Colors::get_default_fg(),
                &Lang::get().complement,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &Lang::get().select,
                Colors::get_msg_highlight_fg(),
                Colors::get_default_fg(),
                &Lang::get().movement,
                Colors::get_msg_highlight_fg(),
            );

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &Lang::get().filenm, Colors::get_default_fg());

            Log::debug("prom.prom_open_file.cache_disp_filenm", &prom.prom_open_file.cache_disp_filenm);

            if !prom.prom_open_file.cache_disp_filenm.is_empty() && prom.prom_open_file.file_type == OpenFileType::Normal {
                self.buf = prom.prom_open_file.cache_disp_filenm.chars().collect();
                prom.prom_open_file.base_path = prom.prom_open_file.cache_full_path.clone();
            } else {
                match prom.prom_open_file.file_type {
                    OpenFileType::Normal => {
                        if let Ok(path) = env::current_dir() {
                            self.buf = format!("{}{}", path.to_string_lossy().to_string(), path::MAIN_SEPARATOR).chars().collect();
                        }
                    }
                    OpenFileType::JsMacro => {
                        let mut path_str = String::new();
                        if let Some(base_dirs) = BaseDirs::new() {
                            let macros_dir = base_dirs.config_dir().join(APP_NAME).join(MACROS_DIR);
                            if macros_dir.exists() {
                                path_str = macros_dir.to_string_lossy().to_string();
                                path_str.push(path::MAIN_SEPARATOR);
                            }
                        }
                        self.buf = path_str.chars().collect();
                    }
                };
            };
            self.cur.x = self.buf.len();
            self.cur.disp_x = get_str_width(&self.buf.iter().collect::<String>());
        } else if self.posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &Lang::get().file_list, Colors::get_default_fg());
        }
        self.clone()
    }
}

impl PromOpenFile {
    pub const PATH_INPUT_FIELD: usize = USIZE_UNDEFINED;

    pub fn set_file_path(prom: &mut Prompt, path: &str) {
        let path = &path.replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str);
        // -2 is margin
        let disp_path = cut_str(path.clone(), prom.disp_col_num - 2, true, true);

        let tmp = disp_path.replace(CONTINUE_STR, "");
        prom.prom_open_file.omitted_path_str = path.replace(&tmp, "");

        let width = get_str_width(&disp_path);
        prom.cont_1.cur.disp_x = width;
        prom.cont_1.cur.x = disp_path.chars().count();
        prom.cont_1.buf = disp_path.chars().collect();
    }

    pub fn set_file_path_parent(prom: &mut Prompt, path: &str) {
        if File::is_root_dir(path) {
            PromOpenFile::set_file_path(prom, path);
            return;
        }
        let path = &path.replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str);

        let mut parent_str = Path::new(path).parent().unwrap().display().to_string();
        if !File::is_root_dir(&parent_str) {
            parent_str.push_str(&MAIN_SEPARATOR.to_string());
        }
        PromOpenFile::set_file_path(prom, &parent_str);
    }

    pub fn select_open_file(&mut self, path: &str) -> String {
        let disp_filenm = get_dir_path(path);
        let full_path = path.replace(CONTINUE_STR, &self.omitted_path_str);
        if self.file_type == OpenFileType::Normal {
            self.cache_disp_filenm = disp_filenm;
            self.cache_full_path = full_path.clone();
        }
        full_path
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
        prom.prom_open_file.base_path = get_dir_path(&prom.cont_1.buf.iter().collect::<String>().replace(CONTINUE_STR, &prom.prom_open_file.omitted_path_str));
    }

    pub fn get_disp_file_count(&self) -> usize {
        let mut count = 0;
        let dest = min(self.vec.len(), self.offset + self.disp_row_len);
        for vec in self.vec[0..dest].iter() {
            count += vec.len();
        }
        count
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

    pub fn set_vec_posi(&mut self, cur_direction: Direction) {
        match cur_direction {
            Direction::Right | Direction::Left => {
                if let Some(vec) = self.vec.get(self.vec_y) {
                    if cur_direction == Direction::Right {
                        if vec.get(self.vec_x + 1).is_some() {
                            self.vec_x += 1;
                        }
                    } else if self.vec_x != 0 && vec.get(self.vec_x - 1).is_some() {
                        self.vec_x -= 1;
                    }
                }
            }
            Direction::Up => {
                if self.vec_y == 0 {
                    self.vec_y = PromOpenFile::PATH_INPUT_FIELD;
                } else if let Some(vec) = self.vec.get(self.vec_y - 1) {
                    if vec.get(self.vec_x).is_some() {
                        self.vec_y -= 1;
                    }
                    if self.vec_y < self.offset {
                        self.offset -= 1;
                    }
                }
            }
            Direction::Down => {
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
                } else if let Some(vec) = self.vec.get(self.vec_y + 1) {
                    if vec.get(self.vec_x).is_some() {
                        if self.vec_y >= self.offset + self.disp_row_len - 1 {
                            self.offset += 1;
                        }
                        self.vec_y += 1;
                    }
                }
            }
        }
    }
    pub fn get_file_disp_str(&self, op_file: &OpenFile, y: usize, x: usize) -> String {
        // Select
        if y == self.vec_y && x == self.vec_x {
            if op_file.file.is_dir {
                Colors::get_file_dir_inversion_fg_bg()
            } else if File::is_executable(&op_file.file.name) {
                Colors::get_file_executable_inversion_fg_bg()
            } else {
                Colors::get_file_normal_inversion_fg_bg()
            }
        } else if op_file.file.is_dir {
            Colors::get_file_dir_fg_bg()
        } else if File::is_executable(&op_file.file.name) {
            Colors::get_file_executable_fg_bg()
        } else {
            Colors::get_file_normal_fg_bg()
        }
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
            max_len = get_term_size().0 as usize;
        }

        let rest_num = file_vec_len % split_idx;
        if rest_num != 0 {
            row_num += 1;
        }

        let mut row_vec: Vec<OpenFile> = vec![];
        for (idx, file) in file_vec.iter_mut().enumerate() {
            row_vec.push(OpenFile { file: file.clone(), ..OpenFile::default() });
            if row_vec.len() == row_num || idx == file_vec_len - 1 {
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

    if !column_len_file_vec.is_empty() {
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
    (all_row_vec, all_count)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromOpenFile {
    pub file_type: OpenFileType,
    pub vec: Vec<Vec<OpenFile>>,
    pub file_all_count: usize,
    pub offset: usize,
    pub disp_row_len: usize,
    pub cache_disp_filenm: String,
    pub cache_full_path: String,
    pub tab_comp: TabComp,
    pub vec_y: usize,
    pub vec_x: usize,
    pub base_path: String,
    pub omitted_path_str: String,
}

impl Default for PromOpenFile {
    fn default() -> Self {
        PromOpenFile { file_type: OpenFileType::Normal, vec: vec![], file_all_count: 0, offset: 0, disp_row_len: 0, cache_disp_filenm: String::new(), cache_full_path: String::new(), tab_comp: TabComp::default(), vec_y: PromOpenFile::PATH_INPUT_FIELD, vec_x: 0, base_path: String::new(), omitted_path_str: String::new() }
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
