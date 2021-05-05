use crate::{colors::*, global::*, log::*, model::*, prompt::prompt::*, prompt::promptcont::promptcont::*, tab::Tab, terminal::*, util::*};
use crossterm::event::{Event::*, KeyCode::*, KeyEvent, MouseButton as M_Btn, MouseEvent as M_Event, MouseEventKind as M_Kind};
use crossterm::{cursor::*, terminal::ClearType::*, terminal::*};
use std::{
    env,
    path::{self, Path},
};

impl EvtAct {
    pub fn open_file(term: &mut Terminal) -> EvtActType {
        Log::debug_s("Process.open_file");

        match term.curt().editor.evt {
            Mouse(M_Event { kind: M_Kind::Down(M_Btn::Left), column: x, row: y, .. }) => {
                let (x, y) = (x as usize, y as usize);
                if y != term.curt().prom.cont_1.buf_row_posi as usize {
                    return EvtActType::Hold;
                } else {
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

                                    let width = get_str_width(&path_str);
                                    term.curt().prom.cont_1.cur.disp_x = width;
                                    term.curt().prom.cont_1.cur.x = path_str.chars().count();
                                    term.curt().prom.cont_1.buf = path_str.chars().collect();
                                }
                                break;
                            }
                            all_width += w;
                        }
                        path_str.push_str(&path);
                    }

                    return EvtActType::Hold;
                }
            }
            Key(KeyEvent { code, .. }) => match code {
                Enter => {
                    let open_filenm = term.curt().prom.cont_1.buf.iter().collect::<String>();
                    let path = Path::new(&open_filenm);

                    if open_filenm.len() == 0 {
                        term.curt().mbar.set_err(&LANG.not_entered_filenm);
                    } else if !path.exists() {
                        term.curt().mbar.set_err(&LANG.file_not_found);
                    } else if path.metadata().unwrap().is_dir() {
                        term.curt().mbar.set_err(&LANG.file_not_found);
                    } else {
                        term.curt().mbar.clear();
                        term.curt().prom.clear();
                        term.curt().state.clear();

                        term.open(&open_filenm, &mut Tab::new());
                        term.curt().editor.d_range.draw_type = DrawType::All;

                        return EvtActType::Next;
                    }
                    term.curt().editor.d_range.draw_type = DrawType::All;
                    return EvtActType::DrawOnly;
                }
                _ => return EvtActType::Hold,
            },
            _ => return EvtActType::Hold,
        }
    }
}

impl Prompt {
    const OPEN_FILE_LIST_ROW_NUM: usize = 5;

    pub fn open_file(term: &mut Terminal) {
        term.curt().state.is_open_file = true;
        term.curt().prom.disp_row_num = 10;
        term.set_disp_size();
        term.curt().prom.cont_1 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::First).get_open_file(&term.curt().prom);
        term.curt().prom.cont_2 = PromptCont::new_edit_type(term.curt().prom.disp_row_posi as u16, PromptContPosi::Second).get_open_file(&term.curt().prom);
    }

    pub fn draw_open_file(&mut self, str_vec: &mut Vec<String>) {
        Log::debug_s("　　　　　　　　　　　　draw_open_file");

        Prompt::set_draw_vec(str_vec, self.cont_1.buf_desc_row_posi, &self.cont_1.buf_desc.clone());
        Prompt::set_draw_vec(str_vec, self.cont_2.buf_desc_row_posi, &self.cont_2.buf_desc.clone());

        // cont_1.buf
        let buf = &self.cont_1.buf.iter().collect::<String>();

        let vec = split_inclusive(buf, path::MAIN_SEPARATOR);

        let mut buf = String::new();
        for (idx, path_str) in vec.iter().enumerate() {
            if path_str == &path::MAIN_SEPARATOR.to_string() || vec.len() - 1 == idx {
                buf.push_str(&path_str);
            } else {
                buf.push_str(&Colors::get_default_inversion_fg_bg());
                buf.push_str(&path_str);
                buf.push_str(&Colors::get_default_fg_bg());
            }
        }

        str_vec.push(format!("{}{}{}", MoveTo(0, self.cont_1.buf_row_posi), Clear(CurrentLine), buf));

        let col = self.disp_col_num;
        let (mut row_idx, mut width) = (0, 0);
        let mut row_str = String::new();

        let str = self.cont_1.buf[..self.cont_1.cur.x].iter().collect::<String>();
        let vec = self.get_tab_comp_files(str, false, false);

        // cont_2.buf
        for file in &vec {
            let filenm = format!("{} ", file.filenm);
            let filenm_len = get_str_width(&filenm);

            if width + filenm_len > col {
                Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi + row_idx as u16, &row_str);
                row_idx += 1;
                row_str.clear();
                width = 0;
            }
            if row_idx != Prompt::OPEN_FILE_LIST_ROW_NUM {
                if file.is_dir {
                    row_str.push_str(&format!("{}{}{}", Colors::get_msg_warning_fg(), &filenm, Colors::get_default_fg(),));
                } else {
                    row_str.push_str(&filenm);
                }
                width += filenm_len;
            }
        }

        if !row_str.is_empty() {
            Prompt::set_draw_vec(str_vec, self.cont_2.buf_row_posi + row_idx as u16, &row_str);
            row_idx += 1;
        }
        // redraw
        for idx in row_idx as usize..Prompt::OPEN_FILE_LIST_ROW_NUM {
            str_vec.push(format!("{}{}", MoveTo(0, self.cont_2.buf_row_posi + idx as u16), Clear(CurrentLine)));
        }
    }
}

impl PromptCont {
    pub fn get_open_file(&mut self, prom: &Prompt) -> PromptCont {
        let base_posi = self.disp_row_posi - 1;

        if self.prompt_cont_posi == PromptContPosi::First {
            self.guide = format!("{}{}", Colors::get_msg_highlight_fg(), &LANG.set_open_filenm);
            self.key_desc = format!(
                "{}{}:{}Enter  {}{}:{}Esc  {}{}:{}Tab  {}{}:{}Directory click",
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
                &LANG.move_dir,
                Colors::get_msg_highlight_fg(),
            );

            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.filenm, Colors::get_default_fg());

            if prom.cache_open_filenm.len() > 0 {
                self.buf = prom.cache_open_filenm.chars().collect();
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
        } else if self.prompt_cont_posi == PromptContPosi::Second {
            self.buf_desc = format!("{}{}{}", Colors::get_msg_highlight_fg(), &LANG.file_list, Colors::get_default_fg());
            self.buf_desc_row_posi = base_posi + 4;
            self.buf_row_posi = base_posi + 5;
        }
        return self.clone();
    }
}
