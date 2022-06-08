use crate::{
    ewin_com::{_cfg::key::keycmd::*, model::*},
    model::*,
};
use ewin_cfg::{lang::lang_cfg::Lang, log::*};
use ewin_const::def::*;
use kana::*;

impl Editor {
    pub fn convert(&mut self, conv_type: ConvType) -> ActType {
        Log::debug_key("Editor.convert");

        if !self.sel.is_selected() {
            return ActType::Draw(DParts::AllMsgBar(Lang::get().no_sel_range.to_string()));
        }
        let sel = self.sel.get_range();
        Log::debug("sel", &sel);

        let tgt_str = self.buf.slice(sel);

        let convert_str = match conv_type {
            ConvType::Lowercase => tgt_str.chars().map(to_lowercase).collect::<String>(),
            ConvType::Uppercase => tgt_str.chars().map(to_uppercase).collect::<String>(),
            ConvType::HalfWidth => to_half_width(&tgt_str),
            ConvType::FullWidth => to_full_width(&tgt_str),
            ConvType::Space => tgt_str.replace(&TAB_CHAR.to_string(), " "),
            ConvType::Tab => tgt_str.replace(' ', &TAB_CHAR.to_string()),
        };

        let e_cmd = E_Cmd::InsertStr(convert_str);
        self.set_keycmd(KeyCmd::Edit(e_cmd.clone()));
        self.edit_proc(e_cmd);

        return ActType::Draw(DParts::All);
    }
}

fn to_half_width(str: &str) -> String {
    let str = wide2ascii(str);
    let str = nowidespace(&str);
    return nowideyen(&str);
}
fn to_full_width(str: &str) -> String {
    let str = ascii2wide(str);
    let str = space2wide(&str);
    return yen2wide(&str);
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
