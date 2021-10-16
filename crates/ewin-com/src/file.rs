extern crate ropey;
use crate::{global::LANG, log::*, model::Encode};
use encoding_rs::Encoding;
use faccess::PathExt;
#[cfg(target_os = "windows")]
use regex::Regex;
use std::io::{self, BufReader, ErrorKind, Read, Seek, SeekFrom};
#[cfg(target_os = "linux")]
use std::path::MAIN_SEPARATOR;
use std::{fmt, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub is_dir: bool,
}

impl Default for File {
    fn default() -> Self {
        File { name: String::new(), is_dir: false }
    }
}
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "File filenm:{}, is_dir:{} ", self.name, self.is_dir)
    }
}
impl File {
    pub fn is_readable(path_str: &str) -> bool {
        if path_str.is_empty() {
            return true;
        } else {
            let path = Path::new(path_str);
            return path.readable();
        }
    }
    pub fn is_readable_writable(path_str: &str) -> (bool, bool) {
        if path_str.is_empty() {
            return (true, true);
        } else {
            let path = Path::new(path_str);
            return (path.readable(), path.writable());
        }
    }
    pub fn is_executable(path: &String) -> bool {
        if path.is_empty() {
            return false;
        } else {
            let path = Path::new(path);
            return path.executable();
        }
    }
    #[cfg(target_os = "linux")]
    pub fn is_root_dir(path: &String) -> bool {
        return path == &MAIN_SEPARATOR.to_string();
    }

    #[cfg(target_os = "windows")]
    pub fn is_root_dir(path: &String) -> bool {
        // C:\ or D:\ ...
        let re = Regex::new(r"[a-zA-Z]:\\").unwrap();
        return re.is_match(path) && path.chars().count() == 3;
    }

    pub fn read_external_file(filepath: &str) -> (String, String) {
        let is_readable = File::is_readable(&filepath);

        match File::read(filepath) {
            Ok((string, _, _)) => return (string, "".to_string()),
            Err(err) => {
                let filenm = Path::new(&filepath).file_name().unwrap().to_string_lossy().to_string();
                Log::error_s(&err.to_string());
                match err.kind() {
                    ErrorKind::PermissionDenied => {
                        if !is_readable {
                            return ("".to_string(), format!("{} {}", &filenm, &LANG.no_read_permission.clone()));
                        }
                    }
                    ErrorKind::NotFound => return ("".to_string(), format!("{} {}", &filenm, &LANG.file_not_found.clone())),
                    _ => return ("".to_string(), format!("{} {}", &filenm, &LANG.file_opening_problem.clone())),
                }
                return ("".to_string(), err.to_string());
            }
        };
    }

    pub fn read(path: &str) -> io::Result<(String, Encode, Option<Encode>)> {
        let (vec, bom) = File::read_file(path)?;

        // UTF8
        let (str, enc) = File::read_bytes(&vec[..], Encode::UTF8);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // SJIS
        let (str, enc) = File::read_bytes(&vec[..], Encode::SJIS);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // EUC_JP
        let (str, enc) = File::read_bytes(&vec[..], Encode::EucJp);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // GBK
        let (str, enc) = File::read_bytes(&vec[..], Encode::GBK);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // UTF16LE・UTF16BE
        // Read once with UTF16LE / UTF16BE to be judged by bom
        let (str, enc) = File::read_bytes(&vec[..], Encode::UTF16LE);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }

        // Encoding::Unknown
        return Ok(((*String::from_utf8_lossy(&vec[..])).to_string(), Encode::Unknown, bom));
    }

    pub fn read_file(path: &str) -> io::Result<(Vec<u8>, Option<Encode>)> {
        if path.is_empty() {
            //     Err(Error::kind(std::io::Error));
        }
        let mut file = std::fs::File::open(path)?;

        Log::debug("file len", &file.metadata()?.len());
        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(&file).read_to_end(&mut vec)?;
        file.seek(SeekFrom::Start(0))?;
        let bom = File::check_file_bom(&file);

        return Ok((vec, bom));
    }
    pub fn read_bytes(bytes: &[u8], encode: Encode) -> (String, Encode) {
        let encoding: &Encoding = Encode::into_encoding(encode);

        // If BOM exists, BOM priority
        // UTF16LE・UTF16BE・UTF8 BOM
        match encode {
            Encode::UTF8 | Encode::UTF16LE | Encode::UTF16BE => {
                //Do not load bom to prevent false recognition
                let (cow, enc, had_errors) = Encode::into_encoding(encode).decode(&bytes);
                if !had_errors {
                    return ((*cow).to_string(), Encode::from_encoding(enc));
                }
            }
            _ => {
                if let Some(str_cow) = encoding.decode_without_bom_handling_and_without_replacement(&bytes) {
                    return ((*str_cow).to_string(), encode);
                };
            }
        }

        return ("".to_string(), Encode::Unknown);
    }

    fn check_file_bom(file: &std::fs::File) -> Option<Encode> {
        let mut reader = BufReader::new(file);
        let mut bom = [0u8; 3];

        if let Ok(_) = reader.read_exact(&mut bom) {
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
        return None;
    }
}
