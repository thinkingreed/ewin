use crate::{
    ewin_com::{_cfg::lang::lang_cfg::*, def::*, file::*, log::*, model::*},
    model::*,
};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use ewin_com::_cfg::model::default::Cfg;
use ropey::RopeBuilder;
use std::{cmp::min, fs::OpenOptions, io::*, option::Option, time::SystemTime, *};

impl TextBuffer {
    pub fn read_file(path: &str) -> io::Result<(TextBuffer, Encode, String, Option<Encode>, SystemTime)> {
        let (read_str, mut enc, bom, modified_time) = File::read(path)?;

        if read_str.is_empty() {
            enc = Encode::UTF8;
        }
        let mut b = RopeBuilder::new();
        b.append(&read_str);
        let text_buf = TextBuffer { text: b.finish() };

        let nl = text_buf.check_nl();
        Ok((text_buf, enc, nl, bom, modified_time))
    }

    pub fn check_nl(&self) -> String {
        let mut new_line = NEW_LINE_CRLF_STR.to_string();
        // 2048 Newline character judgment at a specific size

        let cfg_search = Cfg::get_edit_search();
        let crlf_len = self.search(NEW_LINE_CRLF, 0, min(2048, self.len_chars()), &cfg_search).len();
        if crlf_len == 0 {
            new_line = NEW_LINE_LF_STR.to_string();
        };
        return new_line;
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
        bom
    }

    pub fn write_to(&mut self, h_file: &HeaderFile) -> io::Result<bool> {
        Log::debug("Write file info", &h_file);

        let (mut u8_vec, enc_errors) = self.encode(h_file)?;
        if !enc_errors {
            let vec = self.add_bom(&mut u8_vec, h_file);
            BufWriter::new(fs::File::create(&h_file.fullpath)?).write_all(&vec[..])?;
        }
        Ok(enc_errors)
    }
    pub fn write_simple_to(&mut self, copy_str: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open("clip.txt")?;
        file.write_all(copy_str.as_bytes())?;
        Ok(())
    }

    fn add_bom(&mut self, vec: &mut Vec<u8>, h_file: &HeaderFile) -> Vec<u8> {
        let mut bom_vec: Vec<u8> = vec![];
        match h_file.bom {
            Some(Encode::UTF16LE) => bom_vec = vec![0xFF, 0xFE],
            Some(Encode::UTF16BE) => bom_vec = vec![0xFE, 0xFF],
            Some(Encode::UTF8) => bom_vec = vec![0xEF, 0xBB, 0xBF],
            Some(_) => {}
            None => {}
        };
        bom_vec.append(vec);

        bom_vec
    }

    fn encode(&mut self, h_file: &HeaderFile) -> io::Result<(Vec<u8>, bool)> {
        let mut u8_vec: Vec<u8> = vec![];
        let mut had_errors = false;

        match h_file.enc {
            Encode::UTF16LE | Encode::UTF16BE => {
                let u16_vec: Vec<u16> = self.text.to_string().encode_utf16().collect();

                for u16 in u16_vec {
                    if h_file.bom == Some(Encode::UTF16LE) {
                        u8_vec.write_u16::<LittleEndian>(u16)?;
                    } else {
                        u8_vec.write_u16::<BigEndian>(u16)?;
                    }
                }
            }
            Encode::UTF8 => u8_vec = Vec::from(self.text.to_string().as_bytes()),
            _ => {
                let str = self.text.to_string();
                let (cow, _, _had_errors) = Encode::into_encoding(h_file.enc).encode(&str);

                had_errors = _had_errors;
                u8_vec = Vec::from(&*cow);
            }
        }
        Ok((u8_vec, had_errors))
    }
}
