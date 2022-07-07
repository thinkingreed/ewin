use super::path_comp::*;
use crate::{each::open_file::*, model::*, prom_trait::cont_trait::*};
use crossterm::{cursor::MoveTo, terminal::ClearType::*, terminal::*};
use ewin_cfg::{colors::*, lang::lang_cfg::Lang, log::*};
use ewin_com::{_cfg::key::cmd::CmdType, files::file::*, model::*, util::*};
use ewin_const::def::*;
use std::cmp::min;
use std::fmt::Write as _;

impl PromContPluginTrait for PromContFileList {
    fn as_base(&self) -> &PromptContBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromptContBase {
        &mut self.base
    }
    fn draw(&self, str_vec: &mut Vec<String>, is_curt: bool) {
        str_vec.push(MoveTo(0, (self.as_base().row_posi_range.start) as u16).to_string());
        let num_of_disp = format!("{}/{}", self.get_disp_file_count(), self.file_all_count);
        str_vec.push(format!("{}{}({}){}", if is_curt { Colors::get_msg_highlight_inversion_fg_bg() } else { Colors::get_msg_highlight_fg() }, &Lang::get().file_list, num_of_disp, Colors::get_default_fg_bg()));

        str_vec.push(MoveTo(0, (self.as_base().row_posi_range.start + self.desc_str_vec.len()) as u16).to_string());
        // cont_2.buf
        for y in 0..self.row_num - self.desc_str_vec.len() {
            str_vec.push(format!("{}{}", MoveTo(0, (self.base.row_posi_range.start + self.desc_str_vec.len() + y) as u16), Clear(CurrentLine),));

            let mut row_str = String::new();
            let mut file_disp_str_org = String::new();
            let vec_y = y + self.offset;
            if let Some(vec) = self.vec.get(vec_y) {
                for (x, op_file) in vec.iter().enumerate() {
                    let file_disp_str = self.get_file_disp_str(op_file, vec_y, x);
                    if file_disp_str != file_disp_str_org {
                        let _ = write!(row_str, "{}{}", file_disp_str, op_file.filenm_disp);
                    } else {
                        row_str.push_str(&op_file.filenm_disp);
                    }
                    file_disp_str_org = file_disp_str.clone();
                }
            }
            row_str.push_str(&Colors::get_default_fg_bg());
            if !row_str.is_empty() {
                str_vec.push(format!("{}{}", MoveTo(0, (self.base.row_posi_range.start + self.desc_str_vec.len() + y) as u16), row_str));
            }
        }
    }

    fn check_allow_p_cmd(&self) -> bool {
        return match self.as_base().cmd.cmd_type {
            CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::Cut | CmdType::CursorLeft | CmdType::CursorRight | CmdType::CursorRowHome | CmdType::CursorRowEnd | CmdType::CursorLeftSelect | CmdType::CursorRightSelect | CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect | CmdType::Copy | CmdType::Undo | CmdType::Redo => true,
            CmdType::MouseDownLeft(y, _) | CmdType::MouseDragLeftLeft(y, _) | CmdType::MouseDragLeftRight(y, _) if self.base.row_posi_range.start <= y && y <= self.base.row_posi_range.end => true,
            CmdType::MouseScrollUp | CmdType::MouseScrollDown => true,
            _ => false,
        };
    }
}
impl PromContFileList {
    pub fn get_disp_file_count(&self) -> usize {
        let mut count = 0;
        let dest = min(self.vec.len(), self.offset + self.row_num);
        for vec in self.vec[0..dest].iter() {
            count += vec.len();
        }
        count
    }

    pub fn get_file_disp_str(&self, op_file: &OpenFile, y: usize, x: usize) -> String {
        // Select
        //   if y == self.vec_y + self.desc_str_vec.len() && x == self.vec_x {
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
    pub fn down_disp_file_list(&mut self) {
        if self.vec.len() - 1 - self.offset >= self.row_num {
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
                    self.vec_y = USIZE_UNDEFINED;
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
                if self.vec_y == USIZE_UNDEFINED {
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
                        if self.vec_y >= self.offset + self.row_num - 1 {
                            self.offset += 1;
                        }
                        self.vec_y += 1;
                    }
                }
            }
        }
    }
}

pub fn get_shaping_file_list(file_vec: &mut [File], cols: usize) -> (Vec<Vec<OpenFile>>, usize) {
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
            max_len = get_term_size().0;
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
                    let cut_str = cut_str(&op_file.file.name, max_len, false, true);
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

#[derive(PartialEq, Default, Eq, Debug, Clone)]
pub struct PromContFileList {
    pub base: PromptContBase,
    pub desc_str_vec: Vec<String>,
    pub vec: Vec<Vec<OpenFile>>,
    pub file_all_count: usize,

    pub offset: usize,
    pub row_num: usize,
    pub tab_comp: PathComp,
    pub vec_y: usize,
    pub vec_x: usize,
}
