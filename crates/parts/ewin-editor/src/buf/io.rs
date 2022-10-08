use crate::model::*;
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use ewin_cfg::log::*;
use ewin_utils::files::{bom::*, encode::*, file::*};
use ropey::RopeBuilder;
use std::{fs::OpenOptions, io::*, option::Option, *};

impl TextBuffer {
    pub fn read_file(file: &mut File, specify_encoe_opt: Option<Encode>) -> io::Result<(TextBuffer, Option<Encode>)> {
        let (read_str, mut enc, nl, bom, mod_time) = File::read(&file.fullpath, specify_encoe_opt)?;

        if read_str.is_empty() {
            enc = Encode::UTF8;
        }
        let mut b = RopeBuilder::new();
        b.append(&read_str);
        let text_buf = TextBuffer { text: b.finish() };

        file.enc = enc;
        file.nl = nl;
        file.mod_time = mod_time;
        Ok((text_buf, bom))
    }

    pub fn write_to(&mut self, file: &mut File) -> io::Result<bool> {
        Log::debug("Write file info", &file);

        let (mut u8_vec, enc_errors) = self.encode(file)?;
        Log::debug("enc_errors", &enc_errors);
        if !enc_errors {
            let vec = Bom::add_bom(&mut u8_vec, file);
            BufWriter::new(fs::File::create(&file.fullpath)?).write_all(&vec[..])?;
        }

        Ok(enc_errors)
    }
    pub fn write_simple_to(&mut self, copy_str: &str) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open("clip.txt")?;
        file.write_all(copy_str.as_bytes())?;
        Ok(())
    }

    fn encode(&mut self, file: &mut File) -> io::Result<(Vec<u8>, bool)> {
        let mut u8_vec: Vec<u8> = vec![];
        let mut had_errors = false;

        match file.enc {
            Encode::UTF16LE | Encode::UTF16BE => {
                let u16_vec: Vec<u16> = self.text.to_string().encode_utf16().collect();

                for u16 in u16_vec {
                    if file.bom == Some(Encode::UTF16LE) {
                        u8_vec.write_u16::<LittleEndian>(u16)?;
                    } else {
                        u8_vec.write_u16::<BigEndian>(u16)?;
                    }
                }
            }
            Encode::UTF8 => u8_vec = Vec::from(self.text.to_string().as_bytes()),
            _ => {
                let str = self.text.to_string();

                let (cow, _, _had_errors) = Encode::into_encoding(file.enc).encode(&str);

                had_errors = _had_errors;
                u8_vec = Vec::from(&*cow);
            }
        }
        Ok((u8_vec, had_errors))
    }
}
