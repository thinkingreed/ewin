use crate::{
    ewin_com::{model::*, util::*},
    global_term::H_FILE_VEC,
    model::*,
};
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_com::{
    _cfg::key::cmd::{Cmd, CmdType},
    files::file::*,
};
use ewin_const::def::*;
use ewin_prom::each::open_file::*;
use std::{
    cmp::min,
    path::{self, Path, MAIN_SEPARATOR},
};

impl EvtAct {
    pub fn open_file(term: &mut Terminal) -> ActType {
        Log::debug_s("EvtAct.open_file");

        match term.curt().prom.curt.as_base().cmd.cmd_type {
            CmdType::CursorUp | CmdType::MouseScrollUp => return term.curt().prom.curt::<PromOpenFile>().move_file_list(Direction::Up),
            CmdType::CursorDown | CmdType::MouseScrollDown => return term.curt().prom.curt::<PromOpenFile>().move_file_list(Direction::Down),
            CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => {}
            CmdType::NextContent => term.curt().prom.curt::<PromOpenFile>().set_file_list(),
            CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::CursorRowHome | CmdType::CursorRowEnd => {
                term.curt().prom.curt::<PromOpenFile>().set_file_list();
            }
            CmdType::CursorLeft | CmdType::CursorRight => {
                if term.curt().prom.curt::<PromOpenFile>().file_cont().vec_y == USIZE_UNDEFINED {
                    term.curt().prom.curt::<PromOpenFile>().set_file_list();
                } else {
                    let cur_direction = if term.curt().prom.curt.as_base().cmd.cmd_type == CmdType::CursorLeft { Direction::Left } else { Direction::Right };
                    term.curt().prom.curt::<PromOpenFile>().move_file_list(cur_direction);
                }
            }
            CmdType::MouseDownLeft(y, x) => {
                let file_list_cont_range = &term.curt().prom.curt::<PromOpenFile>().file_cont().base.row_posi_range.clone();
                let input_cont_range = &term.curt().prom.curt.as_mut_base().get_tgt_input_area(0).unwrap().base.row_posi_range.clone();

                if y == input_cont_range.end {
                    let input_cont = &term.curt().prom.curt.as_mut_base().get_tgt_input_area(0).unwrap();
                    Log::debug_s("33333333333333333333333333333333333333333333333");
                    let disp_vec = split_chars(&input_cont.buf.iter().collect::<String>(), true, true, &[MAIN_SEPARATOR]);
                    Log::debug("disp_vec", &disp_vec);

                    // Identifying the path of the clicked position
                    let (mut all_width, mut path_str) = (0, String::new());
                    for path in disp_vec.iter() {
                        if path == &path::MAIN_SEPARATOR.to_string() {
                            all_width += 1;
                        } else {
                            let width = get_str_width(path);
                            if all_width <= x && x <= all_width + width {
                                path_str.push_str(path);
                                path_str = path_str.replace(CONTINUE_STR, &term.curt().prom.curt::<PromOpenFile>().omitted_path_str);
                                if Path::new(&path_str).metadata().unwrap().is_dir() {
                                    path_str.push(path::MAIN_SEPARATOR);
                                    term.curt().prom.curt::<PromOpenFile>().set_file_path(&path_str);
                                    term.curt().prom.curt::<PromOpenFile>().set_file_list();
                                }
                                break;
                            }
                            all_width += width;
                        }
                        path_str.push_str(path);
                    }
                } else if file_list_cont_range.start <= y && y <= file_list_cont_range.end {
                    let file_list_cont = term.curt().prom.curt::<PromOpenFile>().file_cont();
                    let op_file_vec = file_list_cont.vec.clone();
                    let dest = min(file_list_cont.vec.len(), file_list_cont.offset + file_list_cont.row_num);
                    // Identifying the file of the clicked position
                    for (row_idx, vec) in op_file_vec[file_list_cont.offset..dest].iter().enumerate() {
                        for op_file in vec.iter() {
                            if y - file_list_cont.base.row_posi_range.start - file_list_cont.desc_str_vec.len() == row_idx && op_file.filenm_area.0 <= x && x <= op_file.filenm_area.1 {
                                return EvtAct::prom_open_file_confirm(term, op_file);
                            }
                        }
                    }
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::Confirm => {
                let path_str = &term.curt().prom.curt::<PromOpenFile>().base.get_tgt_input_area_str(0);
                let full_path_str = &term.curt().prom.curt::<PromOpenFile>().select_open_file(path_str);
                let path = Path::new(full_path_str);

                if path_str.is_empty() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else if !path.exists() {
                    return ActType::Draw(DParts::MsgBar(Lang::get().file_not_found.to_string()));
                } else if !File::is_readable(full_path_str) {
                    return ActType::Draw(DParts::MsgBar(Lang::get().no_read_permission.to_string()));
                } else if path.metadata().unwrap().is_dir() {
                    term.curt().prom.curt::<PromOpenFile>().set_file_list();
                    return ActType::Draw(DParts::Prompt);
                } else if term.curt().prom.curt::<PromOpenFile>().file_type == OpenFileType::Normal {
                    return EvtAct::prom_file_open(term, &path.display().to_string());
                } else if term.curt().prom.curt::<PromOpenFile>().file_type == OpenFileType::JsMacro {
                    let act_type = Macros::exec_js_macro(term, full_path_str);
                    if let ActType::Draw(DParts::MsgBar(_)) = act_type {
                        return act_type;
                    } else {
                        term.curt().clear_curt_tab(true);
                        return ActType::Draw(DParts::All);
                    };
                }
            }
            _ => return ActType::Cancel,
        };
        return ActType::Draw(DParts::Prompt);
    }
    pub fn prom_file_open(term: &mut Terminal, full_path: &String) -> ActType {
        for (idx, h_file) in H_FILE_VEC.get().unwrap().try_lock().unwrap().iter().enumerate() {
            if full_path == &h_file.fullpath {
                term.tab_idx = idx;
                term.curt().clear_curt_tab(true);
                term.curt().editor.set_cmd(Cmd::to_cmd(CmdType::Null));
                return ActType::Draw(DParts::All);
            }
        }
        let act_type = term.open_file(full_path, FileOpenType::Nomal, Some(&mut Tab::new()), None);
        Log::debug("act_type", &act_type);
        if act_type == ActType::Next {
            term.clear_pre_tab_status();
            return ActType::Draw(DParts::All);
        } else {
            return act_type;
        }
    }
    pub fn prom_open_file_confirm(term: &mut Terminal, op_file: &OpenFile) -> ActType {
        Log::debug_key("prom_open_file_confirm");
        if op_file.file.is_dir {
            let mut path = term.curt().prom.curt::<PromOpenFile>().base_path.clone();
            path.push_str(&op_file.file.name);
            Log::debug("base_path", &path);
            if !File::is_readable(&path) {
                return ActType::Draw(DParts::MsgBar(Lang::get().no_read_permission.to_string()));
            }
            term.curt().prom.curt::<PromOpenFile>().chenge_file_path(op_file);
            term.curt().prom.curt::<PromOpenFile>().set_file_list();
            return ActType::Draw(DParts::Prompt);
        } else {
            let base_path = term.curt().prom.curt::<PromOpenFile>().base_path.clone();
            let path = term.curt().prom.curt::<PromOpenFile>().select_open_file(&base_path);
            return EvtAct::prom_file_open(term, &format!("{}{}", &path, op_file.file.name));
        }
    }
}
