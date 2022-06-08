extern crate ropey;
use crate::model::*;
use ewin_cfg::{lang::lang_cfg::*, log::*, model::default::*};
use faccess::PathExt;
#[cfg(target_os = "windows")]
use regex::Regex;
use std::io::Write;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::path::MAIN_SEPARATOR;
use std::{
    fmt,
    fs::OpenOptions,
    io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom},
    path::Path,
    time::SystemTime,
};

use super::bom::Bom;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File filenm:{}, is_dir:{} ", self.name, self.is_dir)
    }
}
impl File {
    pub fn is_readable(path_str: &str) -> bool {
        if path_str.is_empty() {
            true
        } else {
            let path = Path::new(path_str);
            path.readable()
        }
    }
    pub fn is_readable_writable(path_str: &str) -> (bool, bool) {
        if path_str.is_empty() {
            (true, true)
        } else {
            let path = Path::new(path_str);
            (path.readable(), path.writable())
        }
    }

    pub fn is_executable(path: &str) -> bool {
        if path.is_empty() {
            false
        } else {
            let path = Path::new(path);
            path.executable()
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    pub fn is_root_dir(path: &str) -> bool {
        *path == MAIN_SEPARATOR.to_string()
    }

    #[cfg(target_os = "windows")]
    pub fn is_root_dir(path: &str) -> bool {
        // C:\ or D:\ etc ... or \\
        let re_1 = Regex::new(r"[a-zA-Z]:\\").unwrap();
        let re_2 = Regex::new(r"\\\\").unwrap();
        return re_1.is_match(path) && path.chars().count() == 3 || re_2.is_match(path);
    }

    pub fn read_external_file(filepath: &str) -> (String, String) {
        let is_readable = File::is_readable(filepath);

        match File::read(filepath) {
            Ok((string, _, _, _)) => (string, "".to_string()),
            Err(err) => {
                let filenm = Path::new(&filepath).file_name().unwrap().to_string_lossy().to_string();
                Log::error_s(&err.to_string());
                match err.kind() {
                    ErrorKind::PermissionDenied => {
                        if !is_readable {
                            return ("".to_string(), format!("{} {}", &filenm, &Lang::get().no_read_permission.clone()));
                        }
                    }
                    ErrorKind::NotFound => return ("".to_string(), format!("{} {}", &filenm, &Lang::get().file_not_found.clone())),
                    _ => return ("".to_string(), format!("{} {}", &filenm, &Lang::get().file_opening_problem.clone())),
                }
                ("".to_string(), err.to_string())
            }
        }
    }

    pub fn read(path: &str) -> io::Result<(String, Encode, Option<Encode>, SystemTime)> {
        let (vec, bom, modified_time) = File::read_file(path)?;
        let (str, enc) = Encode::try_read_bytes(&vec);
        return Ok((str, enc, bom, modified_time));
    }

    pub fn read_file(path: &str) -> io::Result<(Vec<u8>, Option<Encode>, SystemTime)> {
        let mut file = std::fs::File::open(path)?;

        Log::debug("file metadata", &file.metadata()?.modified());
        let modified_time = file.metadata()?.modified()?;

        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(&file).read_to_end(&mut vec)?;
        file.seek(SeekFrom::Start(0))?;
        let bom = Bom::check_file_bom(&file);

        Ok((vec, bom, modified_time))
    }

    pub fn get_modified_time(path: &str) -> Option<SystemTime> {
        if File::is_exist_file(path) {
            return File::get_modified(path).ok();
        } else {
            return None;
        }
    }

    fn get_modified(path: &str) -> Result<SystemTime, io::Error> {
        let file = std::fs::File::open(path)?;
        let mod_time = file.metadata()?.modified()?;
        return Ok(mod_time);
    }

    pub fn is_exist_file(path: &str) -> bool {
        return Path::new(&path).exists() && Path::new(&path).is_file();
    }

    pub fn create_write_file(full_path: &Path, s: &str) -> anyhow::Result<()> {
        File::create_dir_all_full_path(full_path)?;
        let mut f: std::fs::File = OpenOptions::new().create(true).write(true).open(full_path)?;
        f.write_all(s.as_bytes())?;
        f.flush()?;
        Ok(())
    }

    pub fn create_dir_all_full_path(full_path: &Path) -> anyhow::Result<()> {
        let prefix = Path::new(full_path).parent().unwrap();
        std::fs::create_dir_all(prefix)?;
        Ok(())
    }

    /*
    pub fn get_assign_ext(filenm: &str) -> String {
        return if !filenm.contains('.') && !Cfg::get().general.editor.save.candidate_extension_when_saving_new_file.is_empty() { format!(".{}", &Cfg::get().general.editor.save.candidate_extension_when_saving_new_file) } else { "".to_string() };
    }
    */
}
