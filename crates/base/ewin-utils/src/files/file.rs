extern crate ropey;
use super::{bom::Bom, encode::*};
use crate::global::*;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use ewin_const::{def::*, models::model::*};
use faccess::PathExt;
#[cfg(target_os = "windows")]
use regex::Regex;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::path::MAIN_SEPARATOR;
use std::{
    ffi::OsStr,
    fmt,
    fs::OpenOptions,
    io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom},
    path::Path,
    time::SystemTime,
};
use std::{fs, io::Write};

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
    #[allow(unused_assignments)]
    pub fn new(path_str: &str) -> Self {
        Log::debug("path_str", &path_str);
        let mut filename = String::new();
        let mut fullpath = String::new();
        let mut is_dir = false;
        let mut ext = String::new();
        let mut len = 0;
        let mut create_time = SystemTime::UNIX_EPOCH;
        let mut mod_time = SystemTime::UNIX_EPOCH;

        let path = Path::new(&path_str);
        if path_str.is_empty() {
            filename = Lang::get().new_file.clone()
        } else {
            if path.is_absolute() {
                filename = path.file_name().unwrap().to_string_lossy().to_string();
                fullpath = path_str.to_string();
            } else {
                filename = path_str.to_string();
                fullpath = Path::new(&*CURT_DIR).join(path_str).to_string_lossy().to_string();
            }

            let metadata = fs::metadata(path_str).unwrap();
            is_dir = metadata.is_dir();

            create_time = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
            mod_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
            len = metadata.len();
        };
        File { name: filename, fullpath, is_dir, ext, len, create_time, mod_time, ..File::default() }
    }
}

pub fn change_nl(string: &mut String, to_nl: &str) {
    // Since it is not possible to replace only LF from a character string containing CRLF,
    // convert it to LF and then convert it to CRLF.
    *string = string.replace(NEW_LINE_CRLF, &NEW_LINE_LF.to_string());
    if to_nl == NEW_LINE_CRLF_STR {
        *string = string.replace(&NEW_LINE_LF.to_string(), NEW_LINE_CRLF);
    }
}

pub fn del_nl(string: &mut String) {
    *string = string.replace(NEW_LINE_CRLF, "");
    *string = string.replace(NEW_LINE_LF, "");
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub fullpath: String,
    pub is_dir: bool,
    // new line
    pub nl: String,
    pub enc: Encode,
    pub ext: String,
    pub bom: Option<Encode>,
    pub len: u64,
    pub create_time: SystemTime,

    pub mod_time: SystemTime,
    pub watch_mode: WatchMode,
}

impl Default for File {
    fn default() -> Self {
        File { name: String::new(), fullpath: String::new(), is_dir: false, ext: String::new(), nl: NEW_LINE_LF_STR.to_string(), enc: Encode::UTF8, bom: None, len: 0, create_time: SystemTime::UNIX_EPOCH, mod_time: SystemTime::UNIX_EPOCH, watch_mode: WatchMode::Normal }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File filenm:{}, is_dir:{} ", self.name, self.is_dir)
    }
}
