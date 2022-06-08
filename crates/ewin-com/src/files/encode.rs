extern crate ropey;
use crate::model::*;
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
}
