use crate::{
    cont::parts::{file_list::*, info::*, input_area::*, key_desc::*},
    ewin_key::key::cmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use directories::BaseDirs;
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_const::term::*;
use ewin_const::{
    def::*,
    models::{draw::*, event::*, model::*},
};
use ewin_job::job::*;
use ewin_key::model::*;
use ewin_state::term::*;
use ewin_utils::{char_edit::*, files::file::*, path::*, str_edit::*};
use std::{
    cmp::min,
    env,
    path::{self, Path, *},
    usize,
};

impl PromOpenFile {
    pub const OPEN_FILE_FIXED_PHRASE_ROW_NUM: usize = 5;
    /*
     * evt_act
     */
    pub fn open_file_prom(&mut self) -> ActType {
        Log::debug_s("EvtAct.open_file");

        match self.as_base().cmd.cmd_type {
            CmdType::CursorUp | CmdType::MouseScrollUp => return self.move_file_list(Direction::Up),
            CmdType::CursorDown | CmdType::MouseScrollDown => return self.move_file_list(Direction::Down),
            CmdType::CursorRowHomeSelect | CmdType::CursorRowEndSelect => {}
            CmdType::NextContent => self.set_file_list(),
            CmdType::InsertStr(_) | CmdType::DelNextChar | CmdType::DelPrevChar | CmdType::CursorRowHome | CmdType::CursorRowEnd => {
                self.set_file_list();
            }
            CmdType::CursorLeft | CmdType::CursorRight => {
                if self.file_cont().vec_y == USIZE_UNDEFINED {
                    self.set_file_list();
                } else {
                    let cur_direction = if self.as_base().cmd.cmd_type == CmdType::CursorLeft { Direction::Left } else { Direction::Right };
                    self.move_file_list(cur_direction);
                }
            }
            CmdType::MouseDownLeft(y, x) => {
                let file_list_cont_range = &self.file_cont().base.row_posi_range.clone();
                let input_cont_range = &self.as_mut_base().get_tgt_input_area(0).unwrap().base.row_posi_range.clone();

                if y == input_cont_range.end {
                    let input_cont = &self.as_mut_base().get_tgt_input_area(0).unwrap();
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
                                path_str = path_str.replace(CONTINUE_STR, &self.omitted_path_str);
                                if Path::new(&path_str).metadata().unwrap().is_dir() {
                                    path_str.push(path::MAIN_SEPARATOR);
                                    self.set_file_path(&path_str);
                                    self.set_file_list();
                                }
                                break;
                            }
                            all_width += width;
                        }
                        path_str.push_str(path);
                    }
                } else if file_list_cont_range.start <= y && y <= file_list_cont_range.end {
                    let file_list_cont = self.file_cont();
                    let op_file_vec = file_list_cont.vec.clone();
                    let dest = min(file_list_cont.vec.len(), file_list_cont.offset + file_list_cont.row_num);
                    // Identifying the file of the clicked position
                    for (row_idx, vec) in op_file_vec[file_list_cont.offset..dest].iter().enumerate() {
                        for op_file in vec.iter() {
                            if y - file_list_cont.base.row_posi_range.start - file_list_cont.desc_str_vec.len() == row_idx && op_file.filenm_area.0 <= x && x <= op_file.filenm_area.1 {
                                return self.file_confirm(op_file);
                            }
                        }
                    }
                } else {
                    return ActType::Cancel;
                }
            }
            CmdType::Confirm => {
                let path_str = &self.base.get_tgt_input_area_str(0);
                let full_path_str = &self.select_open_file(path_str);
                let path = Path::new(full_path_str);

                if path_str.is_empty() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().not_entered_filenm.to_string()));
                } else if !path.exists() {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().file_not_found.to_string()));
                } else if !File::is_readable(full_path_str) {
                    return ActType::Draw(DrawParts::MsgBar(Lang::get().no_read_permission.to_string()));
                } else if path.metadata().unwrap().is_dir() {
                    self.set_file_list();
                    return ActType::Draw(DrawParts::Prompt);
                } else if self.file_type == OpenFileType::Normal {
                    return self.file_open(path.display().to_string());

                    /*
                    } else if self.file_type == OpenFileType::JsMacro {
                        let act_type = Macros::exec_js_macro(full_path_str);
                        if let ActType::Draw(DParts::MsgBar(_)) = act_type {
                            return act_type;
                        } else {
                            self.curt().clear_curt_tab(true);
                            return ActType::Draw(DParts::All);
                        };
                        */
                }
            }
            _ => return ActType::Cancel,
        };
        return ActType::Draw(DrawParts::Prompt);
    }

    pub fn file_open(&mut self, full_path: String) -> ActType {
        // For file already open
        for (idx, state) in State::get().tabs.vec.iter_mut().enumerate() {
            if full_path == state.file.fullpath {
                state.clear();
                Job::send_cmd(CmdType::ChangeFile(idx));
                return ActType::None;
            }
        }
        State::get().curt_mut_state().clear();
        Job::send_cmd(CmdType::OpenTgtFile(full_path));
        return ActType::None;
    }

    pub fn file_confirm(&mut self, op_file: &OpenFile) -> ActType {
        Log::debug_key("prom_open_file_confirm");
        if op_file.file.is_dir {
            let mut path = self.base_path.clone();
            path.push_str(&op_file.file.name);
            Log::debug("base_path", &path);
            if !File::is_readable(&path) {
                return ActType::Draw(DrawParts::MsgBar(Lang::get().no_read_permission.to_string()));
            }
            self.chenge_file_path(op_file);
            self.set_file_list();
            return ActType::Draw(DrawParts::Prompt);
        } else {
            let base_path = self.base_path.clone();
            let path = self.select_open_file(&base_path);
            return self.file_open(format!("{}{}", &path, op_file.file.name));
        }
    }

    /*
     * other
     */

    pub fn new(open_file_type: &OpenFileType) -> Self {
        let mut prom = PromOpenFile { base: PromBase { cfg: PromptConfig { is_updown_valid: true }, ..PromBase::default() }, ..PromOpenFile::default() };

        prom.file_type = *open_file_type;

        prom.base.cont_vec.push(Box::new(PromContInfo {
            desc_str_vec: vec![match prom.file_type {
                OpenFileType::Normal => Lang::get().set_open_filenm.to_string(),
                OpenFileType::JsMacro => Lang::get().set_exec_mocro_filenm.to_string(),
            }],
            fg_color: Colors::get_msg_highlight_fg(),
            ..PromContInfo::default()
        }));

        let open_file = PromContKeyMenu { disp_str: Lang::get().open_file.to_string(), key: PromContKeyMenuType::Cmd(CmdType::Confirm) };
        let selct = PromContKeyMenu { disp_str: Lang::get().select.to_string(), key: PromContKeyMenuType::Cmd(CmdType::MouseDownLeft(0, 0)) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().movement.to_string(), key: PromContKeyMenuType::create_cmds(vec![CmdType::NextContent, CmdType::CursorUp, CmdType::CursorDown, CmdType::CursorLeft, CmdType::CursorRight], &mut vec![CmdType::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::Cmd(CmdType::CancelProm) };
        prom.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![open_file, selct, switch_area, cancel]], ..PromContKeyDesc::default() }));

        let mut input_area = PromContInputArea { desc_str_vec: vec![Lang::get().filenm.to_string()], buf: vec![], config: PromInputAreaConfig { is_edit_proc_later: true, is_path: true, ..PromInputAreaConfig::default() }, ..PromContInputArea::default() };

        match open_file_type {
            OpenFileType::Normal => {
                if let Ok(path) = env::current_dir() {
                    input_area.buf = format!("{}{}", path.to_string_lossy(), path::MAIN_SEPARATOR).chars().collect();
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
                input_area.buf = path_str.chars().collect();
            }
        };
        input_area.cur.x = input_area.buf.len();
        input_area.cur.disp_x = get_str_width(&input_area.buf.iter().collect::<String>());

        prom.base.cont_vec.push(Box::new(input_area));
        prom.base.curt_cont_idx = prom.base.cont_vec.len() - 1;

        prom.base.cont_vec.push(Box::new(PromContFileList { desc_str_vec: vec![Lang::get().file_list.to_string()], ..PromContFileList::default() }));
        prom.set_file_list();
        // let input_path = &plugin.as_mut_base().get_curt_input_area_str();
        prom.base_path = get_dir_path(&prom.as_mut_base().get_tgt_input_area_str(0).replace(CONTINUE_STR, &prom.omitted_path_str));

        return prom;
    }

    pub fn set_file_path(&mut self, path: &str) {
        let path = &path.replace(CONTINUE_STR, &self.omitted_path_str);
        // -2 is margin
        let disp_path = cut_str(path, get_term_size().0 - 2, true, true);

        let tmp = disp_path.replace(CONTINUE_STR, "");
        self.omitted_path_str = path.replace(&tmp, "");

        let input_area = self.as_mut_base().get_tgt_input_area(0).unwrap();
        let width = get_str_width(&disp_path);
        input_area.cur.disp_x = width;
        input_area.cur.x = disp_path.chars().count();
        input_area.buf = disp_path.chars().collect();
    }

    pub fn set_file_path_parent(&mut self, path: &str) {
        if File::is_root_dir(path) {
            self.set_file_path(path);
            return;
        }
        let path = &path.replace(CONTINUE_STR, &self.omitted_path_str);

        let mut parent_str = Path::new(path).parent().unwrap().display().to_string();
        if !File::is_root_dir(&parent_str) {
            parent_str.push_str(&MAIN_SEPARATOR.to_string());
        }
        self.set_file_path(&parent_str);
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

    pub fn chenge_file_path(&mut self, op_file: &OpenFile) {
        let mut path = self.base_path.clone();

        path.push_str(&op_file.file.name);
        if op_file.file.is_dir {
            path.push_str(&MAIN_SEPARATOR.to_string());
        }
        self.set_file_path(&path);
    }
    pub fn file_cont(&mut self) -> &mut PromContFileList {
        return self.as_mut_base().get_tgt::<PromContFileList>(3);
    }

    pub fn set_file_list(&mut self) {
        Log::debug_s("set_file_list");

        // Initialize
        self.file_cont().offset = 0;
        self.file_cont().vec_x = 0;
        self.file_cont().vec_y = USIZE_UNDEFINED;

        let cont = self.as_mut_base().get_tgt_input_area(0).unwrap();
        let path = cont.buf[..cont.cur.x].iter().collect::<String>();
        let path = path.replace(CONTINUE_STR, &self.omitted_path_str);

        let mut vec = get_path_comp_files(path, false, false);
        Log::debug("vec", &vec);
        vec.insert(0, File::new(PARENT_FOLDER));

        let (op_file_row_vec, file_count) = get_shaping_file_list(&mut vec, get_term_size().0);
        self.file_cont().vec = op_file_row_vec;
        self.file_cont().file_all_count = file_count;
    }

    pub fn move_file_list(&mut self, cur_direction: Direction) -> ActType {
        Log::debug("self.base_path 111", &self.base_path);
        Log::debug("self.get_file_cont().unwrap().vec_y", &self.file_cont().vec_y);

        // Beginning of file_cont
        if self.file_cont().vec_y == 0 {
            match cur_direction {
                Direction::Up => {
                    self.base.curt_cont_idx -= 1;
                    self.file_cont().set_vec_posi(cur_direction);
                    self.set_file_path(&self.base_path.clone());
                    return ActType::Draw(DrawParts::Prompt);
                }
                Direction::Down => self.base_path = get_dir_path(&self.as_mut_base().get_tgt_input_area_str(0).replace(CONTINUE_STR, &self.omitted_path_str)),
                _ => {}
            };
        }

        // Input Area
        if self.file_cont().vec_y == USIZE_UNDEFINED {
            match cur_direction {
                Direction::Up => return ActType::Cancel,
                Direction::Down => {
                    self.as_mut_base().curt_cont_idx += 1;
                    self.set_file_path(&self.base_path.clone());
                }
                _ => {}
            }
        };
        Log::debug("self.base_path 222", &self.base_path);
        self.file_cont().set_vec_posi(cur_direction);

        let (y, x) = (self.file_cont().vec_y, self.file_cont().vec_x);
        let op_file = &self.file_cont().vec.get(y).unwrap().get(x).unwrap().clone();

        if op_file.file.is_dir {
            if op_file.file.name == PARENT_FOLDER {
                self.set_file_path_parent(&self.base_path.clone());
            } else {
                let mut path = self.base_path.clone();
                path.push_str(&op_file.file.name);
                self.chenge_file_path(op_file);
            }
        } else {
            self.chenge_file_path(op_file);
            return ActType::Draw(DrawParts::Prompt);
        }
        return ActType::Draw(DrawParts::Prompt);
    }

    pub fn init(open_file_type: &OpenFileType) -> ActType {
        State::get().curt_mut_state().prom = PromState::OpenFile;
        Prom::get().init(Box::new(PromOpenFile::new(open_file_type)));
        return ActType::Draw(DrawParts::TabsAll);
    }
}

impl PromTrait for PromOpenFile {
    fn as_base(&self) -> &PromBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromBase {
        &mut self.base
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromOpenFile {
    pub base: PromBase,
    pub base_path: String,
    pub file_type: OpenFileType,
    pub omitted_path_str: String,
    pub cache_disp_filenm: String,
    pub cache_full_path: String,
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
