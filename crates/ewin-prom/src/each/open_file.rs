use crate::ewin_com::util::*;
use crate::{
    cont::parts::{file_list::*, info::*, input_area::*, key_desc::*},
    ewin_com::_cfg::key::keycmd::*,
    model::*,
    prom_trait::main_trait::*,
};
use directories::BaseDirs;
use ewin_cfg::{colors::*, lang::lang_cfg::*, log::*};
use ewin_com::{files::file::*, model::*};
use ewin_const::def::*;
use std::{
    env,
    path::{self, Path, *},
    usize,
};

impl PromOpenFile {
    pub const OPEN_FILE_FIXED_PHRASE_ROW_NUM: usize = 5;

    pub fn new(open_file_type: OpenFileType) -> Self {
        let mut plugin = PromOpenFile { base: PromPluginBase { config: PromptPluginConfig { is_updown_valid: true, ..PromptPluginConfig::default() }, ..PromPluginBase::default() }, ..PromOpenFile::default() };

        plugin.file_type = open_file_type;

        plugin.base.cont_vec.push(Box::new(PromContInfo {
            desc_str_vec: vec![match plugin.file_type {
                OpenFileType::Normal => Lang::get().set_open_filenm.to_string(),
                OpenFileType::JsMacro => Lang::get().set_exec_mocro_filenm.to_string(),
            }],
            fg_color: Colors::get_msg_highlight_fg(),
            ..PromContInfo::default()
        }));

        let open_file = PromContKeyMenu { disp_str: Lang::get().open_file.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Confirm) };
        let selct = PromContKeyMenu { disp_str: Lang::get().select.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::MouseDownLeft(0, 0)) };
        let switch_area = PromContKeyMenu { disp_str: Lang::get().movement.to_string(), key: PromContKeyMenuType::create_cmds(vec![P_Cmd::NextContent, P_Cmd::CursorUp, P_Cmd::CursorDown, P_Cmd::CursorLeft, P_Cmd::CursorRight], &mut vec![P_Cmd::BackContent]) };
        let cancel = PromContKeyMenu { disp_str: Lang::get().cancel.to_string(), key: PromContKeyMenuType::PCmd(P_Cmd::Cancel) };
        plugin.base.cont_vec.push(Box::new(PromContKeyDesc { desc_vecs: vec![vec![open_file, selct, switch_area, cancel]], ..PromContKeyDesc::default() }));

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

        plugin.base.cont_vec.push(Box::new(input_area.clone()));
        plugin.base.curt_cont_idx = plugin.base.cont_vec.len() - 1;

        plugin.base.cont_vec.push(Box::new(PromContFileList { desc_str_vec: vec![Lang::get().file_list.to_string()], ..PromContFileList::default() }));
        plugin.set_file_list();
        // let input_path = &plugin.as_mut_base().get_curt_input_area_str();
        plugin.base_path = get_dir_path(&plugin.as_mut_base().get_tgt_input_area_str(0).replace(CONTINUE_STR, &plugin.omitted_path_str));

        return plugin;
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
        vec.insert(0, File { name: PARENT_FOLDER.to_string(), is_dir: true });

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
                    return ActType::Draw(DParts::Prompt);
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
            return ActType::Draw(DParts::Prompt);
        }
        return ActType::Draw(DParts::Prompt);
    }
}

impl PromPluginTrait for PromOpenFile {
    fn as_base(&self) -> &PromPluginBase {
        &self.base
    }
    fn as_mut_base(&mut self) -> &mut PromPluginBase {
        &mut self.base
    }
}

#[derive(Debug, Clone, Default)]
pub struct PromOpenFile {
    pub base: PromPluginBase,
    pub base_path: String,
    pub file_type: OpenFileType,
    pub omitted_path_str: String,
    pub cache_disp_filenm: String,
    pub cache_full_path: String,
}

/*
impl Default for PromPluginOpenFile {
    fn default() -> Self {
        PromPluginOpenFile { file_type: OpenFileType::Normal,..PromPluginOpenFile::default(), } // vec: vec![], file_all_count: 0, offset: 0, disp_row_len: 0, cache_disp_filenm: String::new(), cache_full_path: String::new(), tab_comp: PathComp::default(), vec_y: PromOpenFile::PATH_INPUT_FIELD, vec_x: 0, base_path: String::new(), omitted_path_str: String::new() }
    }
}
 */

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
