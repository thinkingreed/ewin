use crate::model::*;
use ewin_cfg::{
    lang::lang_cfg::*,
    log::*,
    model::{general::default::*, modal::*},
};
use ewin_const::{
    def::*,
    models::{draw::*, event::*},
};
use ewin_key::key::cmd::*;
use ewin_state::term::State;
use ewin_utils::files::nl::*;
use serde::Serialize;
use serde_json::Value;

impl Editor {
    pub fn format(&mut self, fmt_type: FileType) -> ActType {
        Log::debug_key("Editor.format");

        if !self.win_mgr.curt_mut().sel.is_selected() {
            return ActType::Draw(DrawParts::MsgBar(Lang::get().no_sel_range.to_string()));
        } else if let Err(err) = self.exec_format(fmt_type) {
            let err_str = format!("{}{}", fmt_type, Lang::get().parsing_failed);
            Log::error(&err_str, &err);
            return ActType::Draw(DrawParts::MsgBar(err_str));
        } else {
            // self.draw_cache.clear();

            return ActType::Draw(DrawParts::TabsAllCacheClear);
        }
    }

    pub fn exec_format(&mut self, fmt_type: FileType) -> anyhow::Result<()> {
        Log::debug_key(&format!("{}:{}", "Editor.format", fmt_type));

        let format_str = match fmt_type {
            FileType::JSON => {
                let value: Value = serde_json::from_str(&self.buf.slice_string(self.win_mgr.curt_mut().sel.get_range()))?;
                let buf = Vec::new();
                let indent = &Cfg::get().general.editor.format.indent;
                let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
                let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
                value.serialize(&mut ser).unwrap();
                let mut format_str = String::from_utf8(ser.into_inner()).unwrap();
                // New line code conversion
                if State::get().curt_ref_state().file.nl == NEW_LINE_CRLF_STR {
                    format_str = format_str.replace(NEW_LINE_LF, NEW_LINE_CRLF)
                }
                format_str
            }
            FileType::XML | FileType::HTML => {
                let slice = self.buf.slice(self.win_mgr.curt_mut().sel.get_range());
                let nl = NL::get_nl(&State::get().curt_ref_state().file.nl.to_string());
                FormatXml::format_xml_html(slice, fmt_type, nl)
            }
            _ => todo!(),
        };

        self.edit_proc_cmd_type(CmdType::InsertStr(format_str));

        Ok(())
    }
}
