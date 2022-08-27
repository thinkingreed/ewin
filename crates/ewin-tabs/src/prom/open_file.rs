use ewin_cfg::{lang::lang_cfg::*, log::*};

use ewin_const::{
    def::*,
    models::{draw::*, evt::*, file::*, model::*},
};
use ewin_key::key::cmd::*;
use ewin_plugin::plugin::*;
use ewin_prom::each::open_file::*;
use ewin_state::term::*;
use ewin_utils::{char_edit::*, files::file::*, str_edit::*};

use std::{
    cmp::min,
    path::{self, Path, MAIN_SEPARATOR},
};

use crate::{tab::Tab, tabs::*};

impl Tabs {
    pub fn open_file_prom(&mut self) -> ActType {
        Log::debug_s("EvtAct.open_file");

        match self.curt().prom.curt.as_base().cmd.cmd_type {
            CmdType::CursorUp | CmdType::MouseScrollUp => return self.curt().prom.curt::<PromOpenFile>().move_file_list(Direction::Up),
            CmdType::CursorDown | CmdType::MouseScrollDown => return self.curt().prom.curt::<PromOpenFile>().move_file_list(Direction::Down),
            CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => {}
            CmdType::NextContent => self.curt().prom.curt::<PromOpenFile>().set_file_list(),
            CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::CursorRowHome | CmdType::CursorRowEnd => {
                self.curt().prom.curt::<PromOpenFile>().set_file_list();
            }
            CmdType::CursorLeft | CmdType::CursorRight => {
                if self.curt().prom.curt::<PromOpenFile>().file_cont().vec_y == USIZE_UNDEFINED {
                    self.curt().prom.curt::<PromOpenFile>().set_file_list();
                } else {
                    let cur_direction = if self.curt().prom.curt.as_base().cmd.cmd_type == CmdType::CursorLeft { Direction::Left } else { Direction::Right };
                    self.curt().prom.curt::<PromOpenFile>().move_file_list(cur_direction);
                }
            }
            CmdType::MouseDownLeft(y, x) => {
                let file_list_cont_range = &self.curt().prom.curt::<PromOpenFile>().file_cont().base.row_posi_range.clone();
                let input_cont_range = &self.curt().prom.curt.as_mut_base().get_tgt_input_area(0).unwrap().base.row_posi_range.clone();

                if y == input_cont_range.end {
                    let input_cont = &self.curt().prom.curt.as_mut_base().get_tgt_input_area(0).unwrap();
                    let disp_vec = split_chars(&input_cont.buf.iter().collect::<String>(), true, true, &[MAIN_SEPARATOR]);

                    // Identifying the path of the clicked position
                    let (mut all_width, mut path_str) = (0, String::new());
                    for path in disp_vec.iter() {
                        if path == &path::MAIN_SEPARATOR.to_string() {
                            all_width += 1;
                        } else {
                            let width = get_str_width(path);
                            if all_width <= x && x <= all_width + width {
                                path_str.push_str(path);
                                path_str = path_str.replace(CONTINUE_STR, &self.curt().prom.curt::<PromOpenFile>().omitted_path_str);
                                if Path::new(&path_str).metadata().unwrap().is_dir() {
                                    path_str.push(path::MAIN_SEPARATOR);
                                    self.curt().prom.curt::<PromOpenFile>().set_file_path(&path_str);
                                    self.curt().prom.curt::<PromOpenFile>().set_file_list();
                                }
                                break;
                            }
                            all_width += width;
                        }
                        path_str.push_str(path);
                    }
                } else if file_list_cont_range.start <= y && y <= file_list_cont_range.end {
                    let file_list_cont = self.curt().prom.curt::<PromOpenFile>().file_cont();
                    let op_file_vec = file_list_cont.vec.clone();
                    let dest = min(file_list_cont.vec.len(), file_list_cont.offset + file_list_cont.row_num);
                    // Identifying the file of the clicked position
                    for (row_idx, vec) in op_file_vec[file_list_cont.offset..dest].iter().enumerate() {
                        for op_file in vec.iter() {
                            if y - file_list_cont.base.row_posi_range.start - file_list_cont.desc_str_vec.len() == row_idx && op_file.filenm_area.0 <= x && x <= op_file.filenm_area.1 {
                                return self.prom_open_file_confirm(op_file);
                            }
                        }
                    }
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::Confirm => {
                let path_str = &self.curt().prom.curt::<PromOpenFile>().base.get_tgt_input_area_str(0);
                let full_path_str = &self.curt().prom.curt::<PromOpenFile>().select_open_file(path_str);
                let path = Path::new(full_path_str);

                if path_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else if !path.exists() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().file_not_found.to_string()));
                } else if !File::is_readable(full_path_str) {
                    return ActType::Draw(DParts::MsgBar(Lang::get().no_read_permission.to_string()));
                } else if path.metadata().unwrap().is_dir() {
                    self.curt().prom.curt::<PromOpenFile>().set_file_list();
                    return ActType::Draw(DParts::Prompt);
                } else if self.curt().prom.curt::<PromOpenFile>().file_type == OpenFileType::Normal {
                    return self.prom_file_open(&path.display().to_string());
                } else if self.curt().prom.curt::<PromOpenFile>().file_type == OpenFileType::JsMacro {
                    let act_type = Macros::exec_js_macro(full_path_str);
                    if let ActType::Draw(DParts::MsgBar(_)) = act_type {
                        return act_type;
                    } else {
                        self.curt().clear_curt_tab(true);
                        return ActType::Draw(DParts::All);
                    };
                }
            }
            _ => return ActType::Cancel,
        };
        return ActType::Draw(DParts::Prompt);
    }
    pub fn prom_file_open(&mut self, full_path: &String) -> ActType {
        // for (idx, h_file) in FileBar::get().file_vec.iter().enumerate() {
        for (idx, state) in State::get().tabs.vec.iter().enumerate() {
            if full_path == &state.file.fullpath {
                self.idx = idx;
                self.curt().clear_curt_tab(true);
                self.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                return ActType::Draw(DParts::All);
            }
        }
        let act_type = self.open_file(full_path, FileOpenType::Nomal, Some(&mut Tab::new()), None);
        Log::debug("act_type", &act_type);
        if act_type == ActType::Next {
            self.clear_pre_tab_status();
            return ActType::Draw(DParts::All);
        } else {
            return act_type;
        }
    }
    pub fn prom_open_file_confirm(&mut self, op_file: &OpenFile) -> ActType {
        Log::debug_key("prom_open_file_confirm");
        if op_file.file.is_dir {
            let mut path = self.curt().prom.curt::<PromOpenFile>().base_path.clone();
            path.push_str(&op_file.file.name);
            Log::debug("base_path", &path);
            if !File::is_readable(&path) {
                return ActType::Draw(DParts::MsgBar(Lang::get().no_read_permission.to_string()));
            }
            self.curt().prom.curt::<PromOpenFile>().chenge_file_path(op_file);
            self.curt().prom.curt::<PromOpenFile>().set_file_list();
            return ActType::Draw(DParts::Prompt);
        } else {
            let base_path = self.curt().prom.curt::<PromOpenFile>().base_path.clone();
            let path = self.curt().prom.curt::<PromOpenFile>().select_open_file(&base_path);
            return self.prom_file_open(&format!("{}{}", &path, op_file.file.name));
        }
    }
}
