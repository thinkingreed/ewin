extern crate ropey;
use encoding_rs::Encoding;
use std::fmt;

impl Encode {
    pub fn into_encoding(encode: Encode) -> &'static Encoding {
        match encode {
            Encode::UTF16LE => &encoding_rs::UTF_16LE_INIT,
            Encode::UTF16BE => &encoding_rs::UTF_16BE_INIT,
            Encode::SJIS => &encoding_rs::SHIFT_JIS_INIT,
            Encode::JIS => &encoding_rs::ISO_2022_JP_INIT,
            Encode::EucJp => &encoding_rs::EUC_JP_INIT,
            Encode::GBK => &encoding_rs::GBK_INIT,
            _ => &encoding_rs::UTF_8_INIT,
        }
    }
    pub fn from_name(name: &str) -> Encode {
        if name == Encode::UTF16LE.to_string() {
            Encode::UTF16LE
        } else if name == Encode::UTF16BE.to_string() {
            Encode::UTF16BE
        } else if name == Encode::SJIS.to_string() {
            Encode::SJIS
        } else if name == Encode::EucJp.to_string() {
            Encode::EucJp
        } else if name == Encode::JIS.to_string() {
            Encode::JIS
        } else if name == Encode::GBK.to_string() {
            Encode::GBK
        } else {
            Encode::UTF8
        }
    }

    pub fn from_encoding(from: &encoding_rs::Encoding) -> Encode {
        if from == &encoding_rs::UTF_16LE_INIT {
            Encode::UTF16LE
        } else if from == &encoding_rs::UTF_16BE_INIT {
            Encode::UTF16BE
        } else if from == &encoding_rs::SHIFT_JIS_INIT {
            Encode::SJIS
        } else if from == &encoding_rs::EUC_JP_INIT {
            Encode::EucJp
        } else if from == &encoding_rs::ISO_2022_JP_INIT {
            Encode::JIS
        } else if from == &encoding_rs::GBK_INIT {
            Encode::GBK
        } else {
            Encode::UTF8
        }
    }
    /*
    fn encode(&mut self, file: &mut File, text: &mut TextBuffer) -> io::Result<(Vec<u8>, bool)> {
        let mut u8_vec: Vec<u8> = vec![];
        let mut had_errors = false;

        match file.enc {
            Encode::UTF16LE | Encode::UTF16BE => {
                let u16_vec: Vec<u16> = text.to_string().encode_utf16().collect();

                for u16 in u16_vec {
                    if file.bom == Some(Encode::UTF16LE) {
                        u8_vec.write_u16::<LittleEndian>(u16)?;
                    } else {
                        u8_vec.write_u16::<BigEndian>(u16)?;
                    }
                }
            }
            Encode::UTF8 => u8_vec = Vec::from(text.to_string().as_bytes()),
            _ => {
                let str = text.to_string();

                let (cow, _, _had_errors) = Encode::into_encoding(file.enc).encode(&str);

                had_errors = _had_errors;
                u8_vec = Vec::from(&*cow);
            }
        }
        Ok((u8_vec, had_errors))
    }
     */
}

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
pub enum Encode {
    UTF8,
    UTF16LE,
    UTF16BE,
    SJIS,
    JIS,
    EucJp,
    GBK,
    Unknown,
}
impl fmt::Display for Encode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Encode::UTF8 => write!(f, "UTF-8"),
            Encode::UTF16LE => write!(f, "UTF-16LE"),
            Encode::UTF16BE => write!(f, "UTF-16BE"),
            Encode::SJIS => write!(f, "Shift_JIS"),
            Encode::JIS => write!(f, "JIS"),
            Encode::EucJp => write!(f, "EUC-JP"),
            Encode::GBK => write!(f, "GBK"),
            Encode::Unknown => write!(f, "Unknown"),
        }
    }
}
