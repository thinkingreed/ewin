extern crate ropey;
use std::fmt;

use encoding_rs::Encoding;
use ewin_cfg::log::*;

impl Encode {
    pub fn try_read_bytes(vec: &[u8]) -> (String, Encode) {
        // UTF8
        let (str, enc, had_errors) = Encode::read_bytes(vec, Encode::UTF8);
        Log::debug("UTF8 had_errors", &had_errors);
        if !had_errors {
            return (str, enc);
        }
        // SJIS
        let (str, enc, had_errors) = Encode::read_bytes(vec, Encode::SJIS);
        Log::debug("SJIS had_errors", &had_errors);
        if !had_errors {
            return (str, enc);
        }
        // EUC_JP
        let (str, enc, had_errors) = Encode::read_bytes(vec, Encode::EucJp);
        if !had_errors {
            return (str, enc);
        }
        // GBK
        let (str, enc, had_errors) = Encode::read_bytes(vec, Encode::GBK);
        if !had_errors {
            return (str, enc);
        }
        // UTF16LE・UTF16BE
        // Read once with UTF16LE / UTF16BE to be judged by bom
        let (str, enc, had_errors) = Encode::read_bytes(vec, Encode::UTF16LE);
        if !had_errors {
            return (str, enc);
        }

        // Encoding::Unknown
        return ((*String::from_utf8_lossy(vec)).to_string(), Encode::Unknown);
    }

    pub fn read_bytes(bytes: &[u8], encode: Encode) -> (String, Encode, bool) {
        Log::debug_key("Encode::read_bytes");

        Log::debug("encode", &encode);
        let (cow, enc, had_errors) = Encode::into_encoding(encode).decode(bytes);
        Log::debug("had_errors", &had_errors);
        return ((*cow).to_string(), Encode::from_encoding(enc), had_errors);
    }

    pub fn into_encoding(self) -> &'static Encoding {
        match self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
