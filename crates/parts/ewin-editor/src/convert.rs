use crate::model::*;
use ewin_cfg::{lang::lang_cfg::Lang, log::*};
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
};
use ewin_key::{key::cmd::*, model::*};
use kana::*;

impl Editor {
    pub fn convert(&mut self, conv_type: ConvType) -> ActType {
        Log::debug_key("Editor.convert");

        if !self.win_mgr.curt_mut().sel.is_selected() {
            return ActType::Draw(DrawParts::TabsAllMsgBar(Lang::get().no_sel_range.to_string()));
        }
        let sel = self.win_mgr.curt_mut().sel.get_range();
        Log::debug("sel", &sel);

        let tgt_str = self.buf.slice(sel);

        Log::debug("conv_type", &conv_type);

        let convert_str = match conv_type {
            ConvType::Lowercase => tgt_str.chars().map(to_lowercase).collect::<String>(),
            ConvType::Uppercase => tgt_str.chars().map(to_uppercase).collect::<String>(),
            ConvType::HalfWidth => to_half_width(&tgt_str),
            ConvType::FullWidth => to_full_width(&tgt_str),
            ConvType::Space => tgt_str.replace(&TAB_CHAR.to_string(), " "),
            ConvType::Tab => tgt_str.replace(' ', &TAB_CHAR.to_string()),
        };

        self.edit_proc_cmd_type(CmdType::InsertStr(convert_str));

        return ActType::Draw(DrawParts::TabsAll);
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
