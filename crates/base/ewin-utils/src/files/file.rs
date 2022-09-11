extern crate ropey;
use super::{bom::*, encode::*, nl::*};
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
        match File::read(filepath, None) {
            Ok((string, _, _, _, _)) => (string, "".to_string()),
            Err(err) => {
                let filenm = Path::new(&filepath).file_name().unwrap().to_string_lossy().to_string();

                let err_str = File::get_io_err_str(err);
                Log::error_s(&err_str);
                ("".to_string(), format!("{} {}", &filenm, err_str))
            }
        }
    }

    pub fn read(path: &str, specify_encoe_opt: Option<Encode>) -> io::Result<(String, Encode, String, Option<Encode>, SystemTime)> {
        let (vec, nl, bom, modified_time) = File::read_file(path)?;

        let (str, enc) = if let Some(specify_encoe) = specify_encoe_opt {
            let (str, enc, had_errors) = File::read_bytes(&vec, specify_encoe);
            Log::debug("specify encoe had_errors", &had_errors);
            (str, enc)
        } else {
            File::try_read_bytes(&vec)
        };
        return Ok((str, enc, nl, bom, modified_time));
    }

    pub fn read_file(path: &str) -> io::Result<(Vec<u8>, String, Option<Encode>, SystemTime)> {
        let mut file = std::fs::File::open(path)?;

        Log::debug("file metadata", &file.metadata()?.modified());
        let modified_time = file.metadata()?.modified()?;

        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(&file).read_to_end(&mut vec)?;
        file.seek(SeekFrom::Start(0))?;
        let bom = Bom::check_file_bom(&file);
        let nl = NL::check_nl(&file);

        Ok((vec, nl, bom, modified_time))
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
    pub fn reload_with_specify_encoding(file: &mut File, enc_name: &str) -> io::Result<(String, bool)> {
        let encode = Encode::from_name(enc_name);

        let (vec, nl, bom, modified_time) = File::read_file(&file.name)?;
        let (mut decode_str, enc, had_errors) = File::read_bytes(&vec, encode);
        if had_errors {
            decode_str = (*String::from_utf8_lossy(&vec)).to_string();
        } else {
            file.bom = bom;
            file.enc = enc;
            file.nl = nl;
            file.mod_time = modified_time;
        }

        Log::info("File info", &file);

        Ok((decode_str, had_errors))
    }
     */

    pub fn get_io_err_str(err: io::Error) -> String {
        return match err.kind() {
            ErrorKind::PermissionDenied => Lang::get().no_read_permission.to_string(),
            ErrorKind::NotFound => Lang::get().file_not_found.to_string(),
            _ => Lang::get().file_opening_problem.to_string(),
        };
    }

    pub fn try_read_bytes(vec: &[u8]) -> (String, Encode) {
        // UTF8
        let (str, enc, had_errors) = File::read_bytes(vec, Encode::UTF8);
        Log::debug("UTF8 had_errors", &had_errors);
        if !had_errors {
            return (str, enc);
        }
        // SJIS
        let (str, enc, had_errors) = File::read_bytes(vec, Encode::SJIS);
        Log::debug("SJIS had_errors", &had_errors);
        if !had_errors {
            return (str, enc);
        }
        // EUC_JP
        let (str, enc, had_errors) = File::read_bytes(vec, Encode::EucJp);
        if !had_errors {
            return (str, enc);
        }
        // GBK
        let (str, enc, had_errors) = File::read_bytes(vec, Encode::GBK);
        if !had_errors {
            return (str, enc);
        }
        // UTF16LE・UTF16BE
        // Read once with UTF16LE / UTF16BE to be judged by bom
        let (str, enc, had_errors) = File::read_bytes(vec, Encode::UTF16LE);
        if !had_errors {
            return (str, enc);
        }

        // Encoding::Unknown
        return ((*String::from_utf8_lossy(vec)).to_string(), Encode::Unknown);
    }

    pub fn read_bytes(bytes: &[u8], encode: Encode) -> (String, Encode, bool) {
        Log::debug_key("Encode::read_bytes");

        Log::debug("encode", &encode);
        let (cow, enc, had_errors) = Encode::into_encoding(encode).decode(bytes);
        Log::debug("had_errors", &had_errors);
        return ((*cow).to_string(), Encode::from_encoding(enc), had_errors);
    }

    pub fn get_absolute_path(path_str: &str) -> String {
        let path = Path::new(path_str);
        return if path.is_absolute() { path_str.to_string() } else { Path::new(&*CURT_DIR).join(path_str).to_string_lossy().to_string() };
    }

    pub fn get_filenm(path_str: &str) -> String {
        let path = Path::new(path_str);
        return if path.is_absolute() { path.file_name().unwrap().to_string_lossy().to_string() } else { path_str.to_string() };
    }

    #[allow(unused_assignments)]
    pub fn new(path_str: &str) -> Self {
        Log::debug("path_str", &path_str);
        let mut filenm = String::new();
        let fullpath = File::get_absolute_path(path_str);
        let mut is_dir = false;
        let mut ext = String::new();
        let mut len = 0;
        let mut create_time = SystemTime::UNIX_EPOCH;
        let mut mod_time = SystemTime::UNIX_EPOCH;

        let path = Path::new(&fullpath);
        if path_str.is_empty() {
            filenm = Lang::get().new_file.clone()
        } else {
            filenm = File::get_filenm(&fullpath);

            let metadata = fs::metadata(&fullpath).unwrap();
            is_dir = metadata.is_dir();

            create_time = metadata.created().unwrap_or(SystemTime::UNIX_EPOCH);
            mod_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            ext = path.extension().unwrap_or_else(|| OsStr::new("txt")).to_string_lossy().to_string();
            len = metadata.len();
        };
        File { name: filenm, fullpath, is_dir, ext, len, create_time, mod_time, ..File::default() }
    }
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum FileOpenType {
    Nomal,
    Reopen,
    ReopenEncode(Encode),
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
