extern crate ropey;
use crate::{_cfg::lang::lang_cfg::*, log::*, model::*};
use encoding_rs::Encoding;
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
        // C:\ or D:\ ... or \\
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

        // UTF8
        let (str, enc) = File::read_bytes(&vec[..], Encode::UTF8);
        if !str.is_empty() {
            return Ok((str, enc, bom, modified_time));
        }
        // SJIS
        let (str, enc) = File::read_bytes(&vec[..], Encode::SJIS);
        if !str.is_empty() {
            return Ok((str, enc, bom, modified_time));
        }
        // EUC_JP
        let (str, enc) = File::read_bytes(&vec[..], Encode::EucJp);
        if !str.is_empty() {
            return Ok((str, enc, bom, modified_time));
        }
        // GBK
        let (str, enc) = File::read_bytes(&vec[..], Encode::GBK);
        if !str.is_empty() {
            return Ok((str, enc, bom, modified_time));
        }
        // UTF16LE・UTF16BE
        // Read once with UTF16LE / UTF16BE to be judged by bom
        let (str, enc) = File::read_bytes(&vec[..], Encode::UTF16LE);
        if !str.is_empty() {
            return Ok((str, enc, bom, modified_time));
        }

        // Encoding::Unknown
        return Ok(((*String::from_utf8_lossy(&vec[..])).to_string(), Encode::Unknown, bom, modified_time));
    }

    pub fn read_file(path: &str) -> io::Result<(Vec<u8>, Option<Encode>, SystemTime)> {
        let mut file = std::fs::File::open(path)?;

        Log::debug("file metadata", &file.metadata()?.modified());
        let modified_time = file.metadata()?.modified()?;

        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(&file).read_to_end(&mut vec)?;
        file.seek(SeekFrom::Start(0))?;
        let bom = File::check_file_bom(&file);

        Ok((vec, bom, modified_time))
    }
    pub fn read_bytes(bytes: &[u8], encode: Encode) -> (String, Encode) {
        let encoding: &Encoding = Encode::into_encoding(encode);

        // If BOM exists, BOM priority
        // UTF16LE・UTF16BE・UTF8 BOM
        match encode {
            Encode::UTF8 | Encode::UTF16LE | Encode::UTF16BE => {
                //Do not load bom to prevent false recognition
                let (cow, enc, had_errors) = Encode::into_encoding(encode).decode(bytes);
                if !had_errors {
                    return ((*cow).to_string(), Encode::from_encoding(enc));
                }
            }
            _ => {
                if let Some(str_cow) = encoding.decode_without_bom_handling_and_without_replacement(bytes) {
                    return ((*str_cow).to_string(), encode);
                };
            }
        }

        ("".to_string(), Encode::Unknown)
    }

    fn check_file_bom(file: &std::fs::File) -> Option<Encode> {
        let mut reader = BufReader::new(file);
        let mut bom = [0u8; 3];

        if reader.read_exact(&mut bom).is_ok() {
            Log::debug("BOM", &bom);
            match Encoding::for_bom(&bom) {
                Some((enc, _)) => {
                    Log::info("BOM", enc);
                    if enc.name() == encoding_rs::UTF_16LE_INIT.name() {
                        return Some(Encode::UTF16LE);
                    } else if enc.name() == encoding_rs::UTF_16BE_INIT.name() {
                        return Some(Encode::UTF16BE);
                    } else if enc.name() == encoding_rs::UTF_8_INIT.name() {
                        return Some(Encode::UTF8);
                    } else {
                        return Some(Encode::Unknown);
                    }
                }
                None => {
                    Log::info_s("BOM None");
                    return None;
                }
            }
        }
        None
    }
    /*
    pub fn get_modified_time(path: &str) -> Option<SystemTime> {
        if Path::new(&path).exists() && Path::new(&path).is_file() {
            let file = std::fs::File::open(path).unwrap();
            return Some(file.metadata().unwrap().modified().unwrap());
        } else {
            return None;
        }
    }
     */

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
}
