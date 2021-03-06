use super::edit::TextBuffer;
use crate::{bar::headerbar::HeaderFile, def::*, global::*, log::*, model::*, prompt::choice::Choice};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use encoding_rs::*;
use ropey::RopeBuilder;
use std::{cmp::min, fs::File, io::*, option::Option, *};

impl TextBuffer {
    pub fn from_path(path: &str) -> io::Result<(TextBuffer, Encode, String, Option<Encode>)> {
        let (mut read_str, mut enc, bom) = TextBuffer::read_encode(path)?;

        if read_str.is_empty() {
            enc = Encode::UTF8;
        }
        let mut b = RopeBuilder::new();
        read_str.push(EOF_MARK);
        b.append(&read_str);
        let text_buf = TextBuffer { text: b.finish() };

        let nl = text_buf.check_nl();
        return Ok((text_buf, enc, nl, bom));
    }

    pub fn set_encoding(&mut self, h_file: &mut HeaderFile, enc_item: &Choice, nl_item: &Choice, apply_item: &Choice, bom_item: &Choice) -> io::Result<()> {
        let to_encode = Encode::from_name(&enc_item.name);

        if apply_item.name == LANG.file_reload {
            let (vec, bom) = TextBuffer::read_file(&h_file.filenm)?;
            h_file.bom = bom;

            let (mut decode_str, enc) = TextBuffer::read_bytes(&vec, to_encode);
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
            h_file.bom = TextBuffer::get_select_item_bom(&to_encode, bom_item);
            h_file.nl_org = h_file.nl.clone();
            h_file.nl = nl_item.name.clone();

            if h_file.nl != h_file.nl_org {
                self.change_nl(&h_file.nl_org, &h_file.nl);
            }
        }
        Log::info("File info", &h_file);

        return Ok(());
    }

    pub fn read_file(path: &str) -> io::Result<(Vec<u8>, Option<Encode>)> {
        if path.is_empty() {
            //     Err(Error::kind(std::io::Error));
        }
        let mut file = File::open(path)?;

        Log::debug("file len", &file.metadata()?.len());
        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(&file).read_to_end(&mut vec)?;
        file.seek(SeekFrom::Start(0))?;
        let bom = TextBuffer::check_file_bom(&file);

        return Ok((vec, bom));
    }

    pub fn read_encode(path: &str) -> io::Result<(String, Encode, Option<Encode>)> {
        let (vec, bom) = TextBuffer::read_file(path)?;

        // UTF8
        let (str, enc) = TextBuffer::read_bytes(&vec[..], Encode::UTF8);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // SJIS
        let (str, enc) = TextBuffer::read_bytes(&vec[..], Encode::SJIS);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // EUC_JP
        let (str, enc) = TextBuffer::read_bytes(&vec[..], Encode::EucJp);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // GBK
        let (str, enc) = TextBuffer::read_bytes(&vec[..], Encode::GBK);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }
        // UTF16LE・UTF16BE
        // Read once with UTF16LE / UTF16BE to be judged by bom
        let (str, enc) = TextBuffer::read_bytes(&vec[..], Encode::UTF16LE);
        if !str.is_empty() {
            return Ok((str, enc, bom));
        }

        // Encoding::Unknown
        return Ok(((*String::from_utf8_lossy(&vec[..])).to_string(), Encode::Unknown, bom));
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

    fn check_nl(&self) -> String {
        let mut new_line = NEW_LINE_CRLF_STR.to_string();
        // 2048 Newline character judgment at a specific size
        let crlf_len = self.search(&NEW_LINE_CRLF, 0, min(2048, self.len_chars())).len();
        if crlf_len == 0 {
            new_line = NEW_LINE_LF_STR.to_string();
        };
        return new_line;
    }

    fn check_file_bom(file: &File) -> Option<Encode> {
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

    fn get_select_item_bom(encode: &Encode, bom_item: &Choice) -> Option<Encode> {
        let bom = match *encode {
            Encode::UTF16LE => Some(Encode::UTF16LE),
            Encode::UTF16BE => Some(Encode::UTF16BE),
            Encode::UTF8 => {
                if bom_item.name == format!("BOM{}", &LANG.with) {
                    Some(Encode::UTF8)
                } else {
                    None
                }
            }
            _ => None,
        };
        return bom;
    }

    pub fn change_nl(&mut self, from_nl_str: &str, to_nl_str: &str) {
        let from_nl = &NL::get_nl(from_nl_str);
        let to_nl = &NL::get_nl(to_nl_str);

        let search_set = self.search(from_nl, 0, self.text.len_chars());
        self.replace(to_nl, &search_set);
    }

    pub fn write_to(&mut self, path: &str, h_file: &HeaderFile) -> io::Result<bool> {
        Log::debug("Write file info", &h_file);

        // Delete EOF_MARK once
        self.text.remove(self.text.len_chars() - 1..self.text.len_chars());

        let (mut u8_vec, enc_errors) = self.encode(h_file)?;
        if !enc_errors {
            let vec = self.add_bom(&mut u8_vec, h_file);
            BufWriter::new(File::create(path)?).write_all(&vec[..])?;
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

        return bom_vec;
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
        return Ok((u8_vec, had_errors));
    }
}
