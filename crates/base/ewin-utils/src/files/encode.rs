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
