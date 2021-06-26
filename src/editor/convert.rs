use crate::{def::TAB_CHAR, global::*, log::*, model::*};
use kana::*;

impl Editor {
    pub fn convert(&mut self, convert_type: &str) {
        Log::debug_key("Editor.convert");
        let tgt_str = self.buf.slice(self.sel.get_range());

        let convert_str = if &LANG.to_lowercase == convert_type {
            tgt_str.chars().map(to_lowercase).collect::<String>()
        } else if &LANG.to_uppercase == convert_type {
            tgt_str.chars().map(to_uppercase).collect::<String>()
        } else if &LANG.to_half_width == convert_type {
            to_half_width(&tgt_str)
        } else if &LANG.to_full_width == convert_type {
            to_full_width(&tgt_str)
        } else if &LANG.to_space == convert_type {
            tgt_str.replace(&TAB_CHAR.to_string(), " ").clone()
        } else if &LANG.to_tab == convert_type {
            tgt_str.replace(" ", &TAB_CHAR.to_string()).clone()
        } else {
            todo!()
        };
        self.buf.replace_onece(&convert_str, &self.sel);
        if !(&LANG.to_lowercase == convert_type || &LANG.to_uppercase == convert_type) {
            self.set_cur_target(self.cur.y, self.cur.x, false);
        }
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
