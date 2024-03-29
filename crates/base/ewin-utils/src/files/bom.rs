extern crate ropey;
use super::{encode::*, file::*};
use encoding_rs::Encoding;
use ewin_cfg::{lang::lang_cfg::*, log::*};
use std::io::{BufReader, Read};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Bom {}

impl Bom {
    pub fn check_file_bom(file: &std::fs::File) -> Option<Encode> {
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
    pub fn get_select_item_bom(encode: &Encode, bom_item_name: &str) -> Option<Encode> {
        let bom = match *encode {
            Encode::UTF16LE => Some(Encode::UTF16LE),
            Encode::UTF16BE => Some(Encode::UTF16BE),
            Encode::UTF8 => {
                if bom_item_name == format!("BOM{}", &Lang::get().with) {
                    Some(Encode::UTF8)
                } else {
                    None
                }
            }
            _ => None,
        };
        return bom;
    }
    pub fn add_bom(vec: &mut Vec<u8>, file: &File) -> Vec<u8> {
        let mut bom_vec: Vec<u8> = vec![];
        match file.bom {
            Some(Encode::UTF16LE) => bom_vec = vec![0xFF, 0xFE],
            Some(Encode::UTF16BE) => bom_vec = vec![0xFE, 0xFF],
            Some(Encode::UTF8) => bom_vec = vec![0xEF, 0xBB, 0xBF],
            Some(_) => {}
            None => {}
        };
        bom_vec.append(vec);
        bom_vec
    }
}
