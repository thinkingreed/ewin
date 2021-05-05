use crate::{def::*, log::*, model::*};
use anyhow::Result;
use encoding_rs::*;
use ropey::RopeBuilder;
use std::{
    cmp::min,
    fs::File,
    io::*,
    io::{BufReader, Read},
    option::Option,
    *,
};

impl TextBuffer {
    pub fn from_path(path: &str, encodhing: Encode) -> io::Result<(TextBuffer, Encode, String, Option<Encode>)> {
        let (read_str, enc, bom_exsist) = TextBuffer::read_file(path, encodhing).unwrap();

        Log::info("encoding", &enc);

        let mut b = RopeBuilder::new();
        b.append(&read_str);
        let text_buf = TextBuffer { text: b.finish() };

        let mut new_line = NEW_LINE_CRLF.to_string();

        // 2048 Newline character judgment at a specific size
        let crlf_len = text_buf.search(&NEW_LINE_CRLF, 0, min(2048, text_buf.len_chars())).len();
        if crlf_len == 0 {
            new_line = NEW_LINE_LF.to_string();
        };

        return Ok((text_buf, enc, new_line, bom_exsist));
    }

    pub fn read_file(path: &str, encodhing: Encode) -> io::Result<(String, Encode, Option<Encode>)> {
        let file = File::open(path)?;

        Log::info("file len", &file.metadata()?.len());

        let bom_option = TextBuffer::check_bom(&file);

        let mut vec: Vec<u8> = vec![];
        // Read all bytes of the file
        BufReader::new(File::open(path)?).read_to_end(&mut vec)?;

        // UTF8
        if let Some(str_cow) = UTF_8.decode_without_bom_handling_and_without_replacement(&vec[..]) {
            return Ok(((*str_cow).to_string(), Encode::UTF8, bom_option));
        }

        // SHIFT_JIS
        if let Some(str_cow) = SHIFT_JIS.decode_without_bom_handling_and_without_replacement(&vec[..]) {
            return Ok(((*str_cow).to_string(), Encode::SJIS, bom_option));
        }

        // EUC_JP
        if let Some(str_cow) = EUC_JP.decode_without_bom_handling_and_without_replacement(&vec[..]) {
            return Ok(((*str_cow).to_string(), Encode::EucJp, bom_option));
        }

        // GBK
        if let Some(str_cow) = GBK.decode_without_bom_handling_and_without_replacement(&vec[..]) {
            return Ok(((*str_cow).to_string(), Encode::GBK, bom_option));
        }

        // UTF16LE・UTF16BE
        if let Some(enc) = bom_option {
            match enc {
                Encode::UTF16LE => {
                    let (cow, _, had_errors) = UTF_16LE.decode(&vec[..]);
                    if !had_errors {
                        return Ok(((*cow).to_string(), Encode::UTF16LE, bom_option));
                    }
                }
                Encode::UTF16BE => {
                    let (cow, _, had_errors) = UTF_16BE.decode(&vec[..]);
                    if !had_errors {
                        return Ok(((*cow).to_string(), Encode::UTF16BE, bom_option));
                    }
                }
                _ => {}
            }
        }

        // Encoding::Unknown
        return Ok(((*String::from_utf8_lossy(&vec[..])).to_string(), Encode::Unknown, bom_option));
    }

    fn check_bom(file: &File) -> Option<Encode> {
        let mut reader = BufReader::new(file);
        let mut bom = [0u8; 3];

        if let Ok(_) = reader.read_exact(&mut bom) {
            match Encoding::for_bom(&bom) {
                Some((enc, _)) => {
                    Log::info("BOM", &bom);
                    Log::debug_s(&enc.name());
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

    pub fn write_to(&mut self, path: &str, encodhing: Encode) -> Result<()> {
        self.text.remove(self.text.len_chars() - 1..self.text.len_chars());

        match encodhing {
            Encode::SJIS => {
                let string = self.text.to_string();
                let (cow, _, had_errors) = encoding_rs::SHIFT_JIS.encode(&string);
                if had_errors {
                    // TODO test
                    Log::error_s(&format!("Replaced a character that could not be encoded with {}", encoding_rs::SHIFT_JIS.name()));
                }
                BufWriter::new(File::create(path)?).write_all(&*cow).unwrap();
            }
            // UTF-8
            _ => self.text.write_to(BufWriter::new(File::create(path)?))?,
        }

        self.insert_end(EOF_MARK.to_string().as_str());
        Ok(())
    }
}
