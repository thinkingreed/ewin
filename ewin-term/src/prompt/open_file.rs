use crate::{
    ewin_core::{_cfg::keys::*, def::*, file::*, global::*, log::Log, model::*, util::*},
    model::*,
    tab::Tab,
    terminal::*,
};
use ewin_prom::{open_file::*, prompt::prompt::Prompt};
use std::{
    cmp::min,
    path::{self, Path},
    usize,
};

impl EvtAct {
    pub fn open_file(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.open_file");

        Log::debug("term.curt().prom.keycmd", &term.curt().prom.keycmd);

        match term.curt().prom.keycmd {
            KeyCmd::Resize => {
                term.set_disp_size();
                PromOpenFile::set_file_list(&mut term.curt().prom);
                return EvtActType::Next;
            }
            KeyCmd::InsertStr(_) | KeyCmd::DeleteNextChar | KeyCmd::DeletePrevChar | KeyCmd::CursorRowHome | KeyCmd::CursorRowEnd | KeyCmd::Tab => PromOpenFile::set_file_list(&mut term.curt().prom),
            KeyCmd::CursorUp => EvtAct::prom_open_file_move_vec(term, CurDirection::Up),
            KeyCmd::CursorDown => EvtAct::prom_open_file_move_vec(term, CurDirection::Down),
            KeyCmd::CursorLeft | KeyCmd::CursorRight => {
                if term.curt().prom.prom_open_file.vec_y == PromOpenFile::PATH_INPUT_FIELD {
                    PromOpenFile::set_file_list(&mut term.curt().prom);
                } else {
                    let cur_direction = if term.curt().prom.keycmd == KeyCmd::CursorLeft { CurDirection::Left } else { CurDirection::Right };
                    EvtAct::prom_open_file_move_vec(term, cur_direction);
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
                                    return EvtAct::prom_open_file_select_file(term, op_file, true);
                                }
                            }
                        }
                    }
                }
            }
            KeyCmd::MouseScrollUp => EvtAct::prom_open_file_move_vec(term, CurDirection::Up),
            KeyCmd::MouseScrollDown => EvtAct::prom_open_file_move_vec(term, CurDirection::Down),
            KeyCmd::ConfirmPrompt => {
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
                    Log::debug("full_path_str", &full_path_str);
                    Log::debug("term.curt().prom.prom_open_file.keycmd", &term.curt().prom.prom_open_file.keycmd);

                    if term.curt().prom.prom_open_file.keycmd == KeyCmd::OpenFile(OpenFileType::Normal) {
                        let mut tgt_idx = USIZE_UNDEFINED;
                        // Check if the file is already open
                        for (idx, h_file) in term.hbar.file_vec.iter().enumerate() {
                            if full_path_str == h_file.fullpath {
                                tgt_idx = idx;
                            }
                        }
                        Log::debug("tgt_idx", &tgt_idx);

                        if tgt_idx == USIZE_UNDEFINED {
                            if term.open(&path.display().to_string(), &mut Tab::new()) {
                                term.clear_pre_tab_status();
                            }
                        } else {
                            term.idx = tgt_idx;
                            term.curt().editor.set_keys(&Keys::Null);
                        }
                    } else if term.curt().prom.prom_open_file.keycmd == KeyCmd::OpenFile(OpenFileType::JsMacro) {
                        Macros::exec_js_macro(term, &full_path_str);
                        term.clear_curt_tab();
                    }
                    return EvtActType::DrawOnly;
                }
                term.curt().editor.draw_type = DrawType::All;
                return EvtActType::DrawOnly;
            }
            _ => return EvtActType::Hold,
        }
        return EvtActType::Hold;
    }

    pub fn prom_open_file_move_vec(term: &mut Terminal, cur_direction: CurDirection) {
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

        let _ = EvtAct::prom_open_file_select_file(term, &op_file, false);
    }

    pub fn prom_open_file_select_file(term: &mut Terminal, op_file: &OpenFile, is_click: bool) -> EvtActType {
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
}
