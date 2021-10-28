use crate::{
    ewin_com::{_cfg::lang::lang_cfg::*, def::*, file::*, global::*, log::*, model::*},
    model::*,
};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use ropey::RopeBuilder;
use std::{cmp::min, io::*, option::Option, *};

impl TextBuffer {
    pub fn from_path(path: &str) -> io::Result<(TextBuffer, Encode, String, Option<Encode>)> {
        let (mut read_str, mut enc, bom) = File::read(path)?;

        if read_str.is_empty() {
            enc = Encode::UTF8;
        }
        let mut b = RopeBuilder::new();
        read_str.push(EOF_MARK);
        b.append(&read_str);
        let text_buf = TextBuffer { text: b.finish() };

        let nl = text_buf.check_nl();
        Ok((text_buf, enc, nl, bom))
    }

    pub fn set_encoding(&mut self, h_file: &mut HeaderFile, to_encode: Encode, nl_item_name: &str, apply_item_name: &str, bom_item_name: &str) -> io::Result<()> {
        if apply_item_name == Lang::get().file_reload {
            let (vec, bom) = File::read_file(&h_file.filenm)?;
            h_file.bom = bom;

            let (mut decode_str, enc) = File::read_bytes(&vec, to_encode);
            if decode_str.is_empty() {
                decode_str = (*String::from_utf8_lossy(&vec)).to_string();
            }
            decode_str.push(EOF_MARK);

            h_file.enc = enc;
            h_file.nl = self.check_nl();

            let mut b = RopeBuilder::new();
            b.append(&decode_str);
            self.text = b.finish();
        } else {
            h_file.enc = to_encode;
            h_file.bom = TextBuffer::get_select_item_bom(&to_encode, bom_item_name);
            h_file.nl_org = h_file.nl.clone();
            h_file.nl = nl_item_name.to_string();

            if h_file.nl != h_file.nl_org {
                self.change_nl(&h_file.nl_org, &h_file.nl);
            }
        }
        Log::info("File info", &h_file);

        Ok(())
    }

    fn check_nl(&self) -> String {
        let mut new_line = NEW_LINE_CRLF_STR.to_string();
        // 2048 Newline character judgment at a specific size
        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;
        let crlf_len = self.search(NEW_LINE_CRLF, 0, min(2048, self.len_chars()), cfg_search).len();
        if crlf_len == 0 {
            new_line = NEW_LINE_LF_STR.to_string();
        };
        new_line
    }

    fn get_select_item_bom(encode: &Encode, bom_item_name: &str) -> Option<Encode> {
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

    pub fn change_nl(&mut self, from_nl_str: &str, to_nl_str: &str) {
        let from_nl = &NL::get_nl(from_nl_str);
        let to_nl = &NL::get_nl(to_nl_str);
        let cfg_search = &CFG.get().unwrap().try_lock().unwrap().general.editor.search;

        let search_map = self.search(from_nl, 0, self.text.len_chars(), cfg_search);

        self.replace(cfg_search.regex, to_nl, &search_map);
    }

    pub fn write_to(&mut self, path: &str, h_file: &HeaderFile) -> io::Result<bool> {
        Log::debug("Write file info", &h_file);

        // Delete EOF_MARK once
        self.text.remove(self.text.len_chars() - 1..self.text.len_chars());

        let (mut u8_vec, enc_errors) = self.encode(h_file)?;
        if !enc_errors {
            let vec = self.add_bom(&mut u8_vec, h_file);
            BufWriter::new(fs::File::create(path)?).write_all(&vec[..])?;
            self.insert_end(EOF_MARK.to_string().as_str());
        }
        Ok(enc_errors)
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