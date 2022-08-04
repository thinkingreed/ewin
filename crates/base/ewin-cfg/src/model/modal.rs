use std::fmt;

use clap::Parser;

use crate::lang::lang_cfg::Lang;

#[derive(Debug, Parser)]
#[clap(name = "ewin", author, version)]
pub struct Args {
    filenm: Option<String>,
    #[clap(short, long, help = "Configuration file output flag")]
    out_config_flg: bool,
}
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AppArgs {
    pub filenm: String,
    pub out_config_flg: bool,
}

impl AppArgs {
    pub fn new(arg: Args) -> Self {
        let filenm: String = if let Some(filenm) = arg.filenm.clone() { filenm } else { "".to_string() };
        AppArgs { filenm, out_config_flg: arg.out_config_flg }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFgFilePath {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// FormatType
pub enum FileType {
    JSON,
    JSON5,
    TOML,
    XML,
    HTML,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileType::JSON => write!(f, "JSON"),
            FileType::JSON5 => write!(f, "JSON5"),
            FileType::TOML => write!(f, "TOML"),
            FileType::XML => write!(f, "XML"),
            FileType::HTML => write!(f, "HTML"),
        }
    }
}
impl FileType {
    pub fn from_str_fmt_type(s: &str) -> FileType {
        if s == Lang::get().json {
            FileType::JSON
        } else if s == Lang::get().json5 {
            FileType::JSON5
        } else if s == Lang::get().toml {
            FileType::TOML
        } else if s == Lang::get().xml {
            FileType::XML
        } else {
            FileType::HTML
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum MacrosFunc {
    insertString,
    getSelectedString,
    getAllString,
    searchAll,
}

impl fmt::Display for MacrosFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MacrosFunc::insertString => write!(f, "insertString"),
            MacrosFunc::getSelectedString => write!(f, "getSelectedString"),
            MacrosFunc::getAllString => write!(f, "getAllString"),
            MacrosFunc::searchAll => write!(f, "searchAll"),
        }
    }
}
