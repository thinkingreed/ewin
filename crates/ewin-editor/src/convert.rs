use crate::{
    ewin_com::{_cfg::key::keycmd::*, def::*, log::*, model::*},
    model::*,
};

use kana::*;

impl Editor {
    pub fn convert(&mut self, conv_type: ConvType) {
        Log::debug_key("Editor.convert");

        let sel = self.sel.get_range();

        let tgt_str = self.buf.slice(sel);
        let convert_str = if ConvType::Lowercase == conv_type {
            tgt_str.chars().map(to_lowercase).collect::<String>()
        } else if ConvType::Uppercase == conv_type {
            tgt_str.chars().map(to_uppercase).collect::<String>()
        } else if ConvType::HalfWidth == conv_type {
            to_half_width(&tgt_str)
        } else if ConvType::FullWidth == conv_type {
            to_full_width(&tgt_str)
        } else if ConvType::Space == conv_type {
            tgt_str.replace(&TAB_CHAR.to_string(), " ").clone()
        } else if ConvType::Tab == conv_type {
            tgt_str.replace(" ", &TAB_CHAR.to_string()).clone()
        } else {
            todo!()
        };
        self.edit_proc(E_Cmd::InsertStr(convert_str));
    }
}

fn to_half_width(str: &str) -> String {
    let str = wide2ascii(str);
    let str = nowidespace(&str);
    let str = nowideyen(&str);
    return str;
}
fn to_full_width(str: &str) -> String {
    let str = ascii2wide(str);
    let str = space2wide(&str);
    let str = yen2wide(&str);
    return str;
}

fn to_lowercase(c: char) -> char {
    if c.is_uppercase() {
        return c.to_ascii_lowercase();
    }
    return c;
}

fn to_uppercase(c: char) -> char {
    if c.is_lowercase() {
        return c.to_ascii_uppercase();
    }
    return c;
}
